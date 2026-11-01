//! Playback through an output device, by default the platform's default
//! output device on cpal's default host.
//!
//! The device is asked for the audio's sample rate and binary32 samples, and
//! one that can't play them is refused with a typed error: nothing
//! resamples. A reader thread at normal priority fills a ring sized from the
//! stream's measured buffer size, and the callback only pops and copies.

use crate::{
    ring::{ring_with, Callback, Counters, Events, PlaybackEvent, Ring, TransportError},
    Device, DeviceError, DeviceFormat, PreparedAudio,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rtrb::RingBuffer;
use std::{sync::Arc, thread::JoinHandle, time::Duration};

/// An output device of any cpal host.
pub struct NativeDevice {
    device: cpal::Device,
    default: DeviceFormat,
    periods_ahead: usize,
    playback: Option<Playback>,
}

/// A playing stream, and the reader thread that feeds it.
struct Playback {
    stream: Option<cpal::Stream>,
    reader: Option<JoinHandle<()>>,
    events: Events,
    buffer_frames: usize,
    ring_frames: usize,
}

impl Drop for Playback {
    fn drop(&mut self) {
        // The stream holds the callback's end of the ring: once it is gone,
        // the reader's next fill finds the ring abandoned and returns.
        self.stream = None;
        if let Some(reader) = self.reader.take() {
            reader.thread().unpark();
            let _ = reader.join();
        }
    }
}

fn unavailable(error: impl ToString) -> DeviceError {
    DeviceError::Unavailable(error.to_string())
}

impl NativeDevice {
    /// Opens the default output device of the platform's default host.
    pub fn open_default() -> Result<Self, DeviceError> {
        let host = cpal::default_host();
        let device = host.default_output_device();
        let device = device.ok_or_else(|| unavailable("there is no output device"))?;
        Self::open(device)
    }

    /// Opens `device`, of any host.
    pub fn open(device: cpal::Device) -> Result<Self, DeviceError> {
        let config = device.default_output_config().map_err(unavailable)?;
        let default = DeviceFormat {
            sample_rate: config.sample_rate(),
            channels: config.channels(),
        };
        Ok(Self {
            device,
            default,
            periods_ahead: crate::ring::DEFAULT_PERIODS_AHEAD,
            playback: None,
        })
    }

    /// Sizes each later stream's ring at `periods` of its measured periods.
    pub fn with_periods_ahead(mut self, periods: usize) -> Self {
        self.periods_ahead = periods.max(1);
        self
    }

    /// The frames of each callback, as the playing stream measured them.
    pub fn buffer_frames(&self) -> Option<usize> {
        Some(self.playback.as_ref()?.buffer_frames)
    }

    /// The frames the playing stream's ring holds.
    pub fn ring_frames(&self) -> Option<usize> {
        Some(self.playback.as_ref()?.ring_frames)
    }

    /// The latest latency from a callback to the playback of its first
    /// frame, from the stream's timestamps.
    pub fn latency(&self) -> Option<Duration> {
        self.playback.as_ref()?.events.counters().latency()
    }

    /// The counts the playing stream's callback keeps.
    pub fn counters(&self) -> Option<&Arc<Counters>> {
        Some(self.playback.as_ref()?.events.counters())
    }

    /// The next event the playing stream reported.
    pub fn next_event(&mut self) -> Option<PlaybackEvent> {
        self.playback.as_mut()?.events.pop()
    }

    /// Stops the playing stream and its reader.
    pub fn stop(&mut self) {
        self.playback = None;
    }
}

/// The device's configuration for `wanted` in binary32 samples.
fn negotiate(
    device: &cpal::Device,
    default: DeviceFormat,
    wanted: DeviceFormat,
) -> Result<cpal::SupportedStreamConfig, DeviceError> {
    let ranges = device.supported_output_configs().map_err(unavailable)?;
    let mut binary32 = false;
    for range in ranges {
        if range.sample_format() != cpal::SampleFormat::F32 {
            continue;
        }
        binary32 = true;
        if range.channels() != wanted.channels {
            continue;
        }
        if let Some(config) = range.try_with_sample_rate(wanted.sample_rate) {
            return Ok(config);
        }
    }
    if !binary32 {
        return Err(DeviceError::SampleType);
    }
    Err(DeviceError::Unsupported {
        device: default,
        audio: wanted,
    })
}

/// The frames of each callback as the stream measures them, or else the
/// most the device reported.
fn measured_frames(
    stream: &cpal::Stream,
    config: &cpal::SupportedStreamConfig,
) -> Result<usize, DeviceError> {
    if let Ok(frames) = stream.buffer_size() {
        if frames > 0 {
            return Ok(frames as usize);
        }
    }
    match config.buffer_size() {
        cpal::SupportedBufferSize::Range { max, .. } if *max > 0 => Ok(*max as usize),
        _ => Err(DeviceError::BufferSize),
    }
}

/// The event a stream error reports.
fn event_for(error: &cpal::Error) -> PlaybackEvent {
    use cpal::ErrorKind;
    match error.kind() {
        ErrorKind::DeviceNotAvailable => PlaybackEvent::DeviceLost,
        ErrorKind::DeviceChanged => PlaybackEvent::DeviceChanged,
        ErrorKind::StreamInvalidated => PlaybackEvent::StreamInvalidated,
        ErrorKind::Xrun => PlaybackEvent::Xrun,
        ErrorKind::RealtimeDenied => PlaybackEvent::RealtimeDenied,
        ErrorKind::DeviceBusy => PlaybackEvent::Transport(TransportError::DeviceBusy),
        ErrorKind::HostUnavailable => PlaybackEvent::Transport(TransportError::HostUnavailable),
        ErrorKind::PermissionDenied => PlaybackEvent::Transport(TransportError::PermissionDenied),
        ErrorKind::ResourceExhausted => PlaybackEvent::Transport(TransportError::ResourceExhausted),
        ErrorKind::BackendError => PlaybackEvent::Transport(TransportError::Backend),
        _ => PlaybackEvent::Transport(TransportError::Other),
    }
}

impl Device for NativeDevice {
    fn format(&self) -> DeviceFormat {
        self.default
    }

    fn play(&mut self, audio: &PreparedAudio) -> Result<(), DeviceError> {
        self.playback = None;
        let wanted = DeviceFormat {
            sample_rate: audio.sample_rate(),
            channels: audio.layout().channels(),
        };
        let supported = negotiate(&self.device, self.default, wanted)?;
        let mut config = supported.config();
        config.buffer_size = cpal::BufferSize::Default;

        // The stream is built stopped, before its buffer size can be read to
        // size the ring, so the callback plays silence until the ring's end
        // arrives through this one-slot handoff.
        let (mut handoff_in, mut handoff) = RingBuffer::<Callback>::new(1);
        let (mut device_events, device_events_out) = RingBuffer::new(crate::ring::EVENT_CAPACITY);
        let counters = Arc::new(Counters::default());

        let latency = counters.clone();
        let mut active: Option<Callback> = None;
        let data = move |out: &mut [f32], info: &cpal::OutputCallbackInfo| {
            let timestamp = info.timestamp();
            latency.record_latency(timestamp.playback - timestamp.callback);
            if active.is_none() {
                active = handoff.pop().ok();
            }
            match active.as_mut() {
                Some(callback) => {
                    callback.render(out);
                }
                None => out.fill(0.0),
            }
        };
        let dropped = counters.clone();
        let error = move |error: cpal::Error| {
            if device_events.push(event_for(&error)).is_err() {
                dropped.drop_event();
            }
        };
        let stream = self.device.build_output_stream(config, data, error, None);
        let stream = stream.map_err(unavailable)?;

        let buffer_frames = measured_frames(&stream, &supported)?;
        let capacity = buffer_frames * self.periods_ahead;
        let Ring {
            reader,
            mut callback,
            mut events,
        } = ring_with(audio.stream(), wanted.channels, capacity, counters);
        events.with_device(device_events_out);
        let thread = std::thread::Builder::new()
            .name("euphony-daw-playback-reader".to_string())
            .spawn(move || reader.run())
            .map_err(unavailable)?;
        callback.wake(thread.thread().clone());
        let playback = Playback {
            stream: Some(stream),
            reader: Some(thread),
            events,
            buffer_frames,
            ring_frames: capacity,
        };
        let handed = handoff_in.push(callback);
        handed.map_err(|_| unavailable("the stream's callback didn't take its ring"))?;
        if let Some(stream) = &playback.stream {
            stream.play().map_err(unavailable)?;
        }
        self.playback = Some(playback);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpal::{
        platform::CustomDevice, Data, DeviceDescription, DeviceDescriptionBuilder, DeviceId,
        ErrorKind, FrameCount, InputCallbackInfo, OutputCallbackInfo, OutputStreamTimestamp,
        SampleFormat, StreamConfig, StreamInstant, SupportedBufferSize, SupportedStreamConfig,
        SupportedStreamConfigRange,
    };
    use euphony_daw_sample::ChannelLayout;
    use std::{
        fmt,
        hash::{Hash, Hasher},
        sync::{Mutex, MutexGuard},
    };

    fn check(kind: cpal::ErrorKind, event: PlaybackEvent) {
        assert_eq!(event_for(&cpal::Error::new(kind)), event, "{kind:?}");
    }

    #[test]
    fn each_stream_error_reports_its_typed_event() {
        use cpal::ErrorKind as K;
        use PlaybackEvent::*;
        use TransportError as T;
        check(K::DeviceNotAvailable, DeviceLost);
        check(K::DeviceChanged, DeviceChanged);
        check(K::StreamInvalidated, StreamInvalidated);
        check(K::Xrun, Xrun);
        check(K::RealtimeDenied, RealtimeDenied);
        check(K::DeviceBusy, Transport(T::DeviceBusy));
        check(K::HostUnavailable, Transport(T::HostUnavailable));
        check(K::PermissionDenied, Transport(T::PermissionDenied));
        check(K::ResourceExhausted, Transport(T::ResourceExhausted));
        check(K::BackendError, Transport(T::Backend));
        check(K::UnsupportedConfig, Transport(T::Other));
        check(K::Other, Transport(T::Other));
    }

    type DataCallback = Box<dyn FnMut(&mut Data, &OutputCallbackInfo) + Send>;
    type ErrorCallback = Box<dyn FnMut(cpal::Error) + Send>;

    /// What a scripted device answers, and what the stream it built holds.
    #[derive(Default)]
    struct Script {
        /// The configurations the device answers it plays.
        ranges: Vec<SupportedStreamConfigRange>,
        /// The buffer size the built stream measures, or none where it can't
        /// tell.
        measured: Option<FrameCount>,
        /// The frames of a period the device plays while it builds the
        /// stream, before anyone starts it.
        period_at_build: Option<usize>,
        /// What that period played.
        played_at_build: Option<Vec<f32>>,
        /// The configuration and sample format the stream was built with.
        built: Option<(StreamConfig, SampleFormat)>,
        data: Option<DataCallback>,
        errors: Option<ErrorCallback>,
        playing: bool,
    }

    /// An output device of cpal's custom host whose answers a test chooses,
    /// and whose stream's callbacks the test calls itself, one period at a
    /// time, as the platform's audio thread would.
    #[derive(Clone)]
    struct ScriptedDevice(Arc<Mutex<Script>>);

    impl ScriptedDevice {
        fn new(script: Script) -> Self {
            Self(Arc::new(Mutex::new(script)))
        }

        fn script(&self) -> MutexGuard<'_, Script> {
            self.0.lock().unwrap()
        }

        /// A native device that plays through this one.
        fn open(&self) -> NativeDevice {
            let device = cpal::Device::from(CustomDevice::from_device(self.clone()));
            NativeDevice::open(device).unwrap()
        }

        /// Plays one period of `frames` frames at the given timestamps.
        fn period(
            &self,
            frames: usize,
            callback: StreamInstant,
            playback: StreamInstant,
        ) -> Vec<f32> {
            play_period(&mut self.script(), frames, callback, playback)
        }

        /// Reports an error of `kind` on the stream's error callback.
        fn fail(&self, kind: ErrorKind) {
            let mut script = self.script();
            let errors = script.errors.as_mut().expect("a stream was built");
            errors(cpal::Error::new(kind));
        }
    }

    fn play_period(
        script: &mut Script,
        frames: usize,
        callback: StreamInstant,
        playback: StreamInstant,
    ) -> Vec<f32> {
        let (config, _) = script.built.as_ref().expect("a stream was built");
        let mut buffer = vec![0.0f32; frames * usize::from(config.channels)];
        let data = script.data.as_mut().expect("the stream is live");
        {
            let len = buffer.len();
            let samples = buffer.as_mut_ptr().cast();
            // SAFETY: `buffer` holds `len` binary32 samples and outlives
            // `raw`.
            let mut raw = unsafe { Data::from_parts(samples, len, SampleFormat::F32) };
            let timestamp = OutputStreamTimestamp { callback, playback };
            data(&mut raw, &OutputCallbackInfo::new(timestamp));
        }
        buffer
    }

    impl fmt::Debug for ScriptedDevice {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("ScriptedDevice")
        }
    }

    impl fmt::Display for ScriptedDevice {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("scripted")
        }
    }

    impl PartialEq for ScriptedDevice {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.0, &other.0)
        }
    }

    impl Eq for ScriptedDevice {}

    impl Hash for ScriptedDevice {
        fn hash<H: Hasher>(&self, state: &mut H) {
            Arc::as_ptr(&self.0).hash(state);
        }
    }

    impl DeviceTrait for ScriptedDevice {
        type SupportedInputConfigs = std::vec::IntoIter<SupportedStreamConfigRange>;
        type SupportedOutputConfigs = std::vec::IntoIter<SupportedStreamConfigRange>;
        type Stream = ScriptedStream;

        fn description(&self) -> Result<DeviceDescription, cpal::Error> {
            Ok(DeviceDescriptionBuilder::new("scripted").build())
        }

        fn id(&self) -> Result<DeviceId, cpal::Error> {
            Err(cpal::Error::new(ErrorKind::UnsupportedOperation))
        }

        fn supported_input_configs(&self) -> Result<Self::SupportedInputConfigs, cpal::Error> {
            Ok(Vec::new().into_iter())
        }

        fn supported_output_configs(&self) -> Result<Self::SupportedOutputConfigs, cpal::Error> {
            Ok(self.script().ranges.clone().into_iter())
        }

        fn default_input_config(&self) -> Result<SupportedStreamConfig, cpal::Error> {
            Err(cpal::Error::new(ErrorKind::UnsupportedConfig))
        }

        fn default_output_config(&self) -> Result<SupportedStreamConfig, cpal::Error> {
            let script = self.script();
            let range = script.ranges.first();
            let range = range.ok_or_else(|| cpal::Error::new(ErrorKind::UnsupportedConfig))?;
            Ok(SupportedStreamConfig::new(
                range.channels(),
                range.max_sample_rate(),
                range.buffer_size().clone(),
                range.sample_format(),
            ))
        }

        fn build_input_stream_raw<D, E>(
            &self,
            _: StreamConfig,
            _: SampleFormat,
            _: D,
            _: E,
            _: Option<Duration>,
        ) -> Result<Self::Stream, cpal::Error>
        where
            D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
            E: FnMut(cpal::Error) + Send + 'static,
        {
            Err(cpal::Error::new(ErrorKind::UnsupportedOperation))
        }

        fn build_output_stream_raw<D, E>(
            &self,
            config: StreamConfig,
            sample_format: SampleFormat,
            data: D,
            errors: E,
            _: Option<Duration>,
        ) -> Result<Self::Stream, cpal::Error>
        where
            D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
            E: FnMut(cpal::Error) + Send + 'static,
        {
            let mut script = self.script();
            script.built = Some((config, sample_format));
            script.data = Some(Box::new(data));
            script.errors = Some(Box::new(errors));
            script.playing = false;
            if let Some(frames) = script.period_at_build {
                let at = StreamInstant::new(0, 0);
                let played = play_period(&mut script, frames, at, at);
                script.played_at_build = Some(played);
            }
            Ok(ScriptedStream(self.0.clone()))
        }
    }

    /// The stream a scripted device built.
    struct ScriptedStream(Arc<Mutex<Script>>);

    impl StreamTrait for ScriptedStream {
        fn play(&self) -> Result<(), cpal::Error> {
            self.0.lock().unwrap().playing = true;
            Ok(())
        }

        fn pause(&self) -> Result<(), cpal::Error> {
            self.0.lock().unwrap().playing = false;
            Ok(())
        }

        fn now(&self) -> StreamInstant {
            StreamInstant::new(0, 0)
        }

        fn buffer_size(&self) -> Result<FrameCount, cpal::Error> {
            let measured = self.0.lock().unwrap().measured;
            measured.ok_or_else(|| cpal::Error::new(ErrorKind::UnsupportedOperation))
        }
    }

    impl Drop for ScriptedStream {
        fn drop(&mut self) {
            // A dropped stream stops and lets its callbacks go, as a
            // platform's stream does.
            if let Ok(mut script) = self.0.lock() {
                script.data = None;
                script.errors = None;
                script.playing = false;
            }
        }
    }

    const BUFFERS: SupportedBufferSize = SupportedBufferSize::Range { min: 64, max: 1024 };

    fn range(
        sample_rate: u32,
        format: SampleFormat,
        buffers: SupportedBufferSize,
    ) -> SupportedStreamConfigRange {
        SupportedStreamConfigRange::new(2, sample_rate, sample_rate, buffers, format)
    }

    fn device(
        ranges: Vec<SupportedStreamConfigRange>,
        measured: Option<FrameCount>,
    ) -> ScriptedDevice {
        ScriptedDevice::new(Script {
            ranges,
            measured,
            ..Script::default()
        })
    }

    fn stereo(sample_rate: u32) -> DeviceFormat {
        DeviceFormat {
            sample_rate,
            channels: 2,
        }
    }

    /// Stereo audio at 48 kHz whose samples count up from 1, so that no
    /// sample of it is silence.
    fn audio(frames: usize) -> PreparedAudio {
        let samples = (0..frames * 2).map(|i| (i + 1) as f32).collect();
        PreparedAudio::new(48_000, ChannelLayout::Stereo, samples)
    }

    fn at(secs: u64, nanos: u32) -> StreamInstant {
        StreamInstant::new(secs, nanos)
    }

    #[test]
    fn a_device_without_the_audios_rate_is_refused_as_unsupported() {
        let mono = SupportedStreamConfigRange::new(1, 48_000, 48_000, BUFFERS, SampleFormat::F32);
        let scripted = device(vec![range(44_100, SampleFormat::F32, BUFFERS), mono], Some(256));
        let mut native = scripted.open();
        let expected = DeviceError::Unsupported {
            device: stereo(44_100),
            audio: stereo(48_000),
        };
        assert_eq!(native.play(&audio(16)), Err(expected));
        assert!(scripted.script().built.is_none());
        assert_eq!(native.buffer_frames(), None);
    }

    #[test]
    fn a_device_without_binary32_samples_is_refused_with_the_sample_type_error() {
        let ranges = vec![
            range(48_000, SampleFormat::I16, BUFFERS),
            range(48_000, SampleFormat::I32, BUFFERS),
        ];
        let scripted = device(ranges, Some(256));
        let mut native = scripted.open();
        assert_eq!(native.play(&audio(16)), Err(DeviceError::SampleType));
        assert!(scripted.script().built.is_none());
    }

    #[test]
    fn the_ring_holds_the_measured_buffer_size_times_the_periods_ahead() {
        let ranges = vec![
            range(44_100, SampleFormat::F32, BUFFERS),
            range(48_000, SampleFormat::F32, BUFFERS),
        ];
        let scripted = device(ranges, Some(256));
        let mut native = scripted.open().with_periods_ahead(3);
        native.play(&audio(16)).unwrap();
        assert_eq!(native.buffer_frames(), Some(256));
        assert_eq!(native.ring_frames(), Some(768));

        let script = scripted.script();
        let (config, format) = script.built.as_ref().unwrap();
        assert_eq!(config.sample_rate, 48_000);
        assert_eq!(config.channels, 2);
        assert_eq!(config.buffer_size, cpal::BufferSize::Default);
        assert_eq!(*format, SampleFormat::F32);
        assert!(script.playing);
    }

    #[test]
    fn a_stream_that_doesnt_measure_its_buffer_sizes_the_ring_from_the_devices_most() {
        let scripted = device(vec![range(48_000, SampleFormat::F32, BUFFERS)], None);
        let mut native = scripted.open().with_periods_ahead(2);
        native.play(&audio(16)).unwrap();
        assert_eq!(native.buffer_frames(), Some(1024));
        assert_eq!(native.ring_frames(), Some(2048));
    }

    #[test]
    fn a_device_without_any_buffer_size_is_refused() {
        let unknown = range(48_000, SampleFormat::F32, SupportedBufferSize::Unknown);
        let scripted = device(vec![unknown], None);
        let mut native = scripted.open();
        assert_eq!(native.play(&audio(16)), Err(DeviceError::BufferSize));
        assert_eq!(native.buffer_frames(), None);
    }

    #[test]
    fn a_stream_built_stopped_plays_silence_until_its_ring_arrives_then_every_frame() {
        let scripted = ScriptedDevice::new(Script {
            ranges: vec![range(48_000, SampleFormat::F32, BUFFERS)],
            measured: Some(64),
            period_at_build: Some(64),
            ..Script::default()
        });
        let mut native = scripted.open();
        let prepared = audio(1000);
        native.play(&prepared).unwrap();
        {
            let script = scripted.script();
            let silence = script.played_at_build.as_ref().unwrap();
            assert_eq!(silence, &vec![0.0f32; 128]);
            assert!(script.playing);
        }
        assert_eq!(native.counters().unwrap().underruns(), 0);

        // The reader thread fills the ring at its own pace: keep the audio's
        // samples of each period, until the callback reports the end.
        let mut played = Vec::new();
        let mut ended = false;
        while !ended {
            let period = scripted.period(64, at(1, 0), at(1, 0));
            played.extend(period.into_iter().filter(|sample| sample.to_bits() != 0));
            while let Some(event) = native.next_event() {
                ended |= event == PlaybackEvent::Ended;
            }
            std::thread::yield_now();
        }
        assert_eq!(played.len(), prepared.samples().len());
        for (played, expected) in played.iter().zip(prepared.samples()) {
            assert_eq!(played.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn a_device_loss_reported_by_the_error_callback_comes_back_as_device_lost() {
        let scripted = device(vec![range(48_000, SampleFormat::F32, BUFFERS)], Some(64));
        let mut native = scripted.open();
        native.play(&audio(16)).unwrap();
        assert_eq!(native.next_event(), None);
        scripted.fail(ErrorKind::DeviceNotAvailable);
        assert_eq!(native.next_event(), Some(PlaybackEvent::DeviceLost));
        scripted.fail(ErrorKind::DeviceChanged);
        assert_eq!(native.next_event(), Some(PlaybackEvent::DeviceChanged));
        assert_eq!(native.next_event(), None);
    }

    #[test]
    fn the_reported_latency_is_the_playback_timestamp_less_the_callback_timestamp() {
        let scripted = device(vec![range(48_000, SampleFormat::F32, BUFFERS)], Some(64));
        let mut native = scripted.open();
        native.play(&audio(1000)).unwrap();
        assert_eq!(native.latency(), None);
        scripted.period(64, at(10, 250_000_000), at(10, 262_500_000));
        assert_eq!(native.latency(), Some(Duration::from_micros(12_500)));
        scripted.period(64, at(11, 999_000_000), at(12, 4_000_000));
        assert_eq!(native.latency(), Some(Duration::from_millis(5)));
    }
}
