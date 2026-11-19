//! Playback through the platform's default output device, on cpal's default
//! host.
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

/// The platform's default output device.
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
}
