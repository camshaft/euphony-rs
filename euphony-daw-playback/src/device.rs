use crate::{
    ring::{ring, PlaybackEvent, Reader, Render, Ring},
    PreparedAudio, Stream,
};
use core::fmt;

/// The format a device plays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceFormat {
    pub sample_rate: u32,
    pub channels: u16,
}

/// Why a device didn't play.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceError {
    /// The device can't play the audio's format: playback doesn't resample
    /// or remap channels silently.
    Unsupported {
        device: DeviceFormat,
        audio: DeviceFormat,
    },
    /// The device plays samples of another type than binary32.
    SampleType,
    /// The device reported neither its buffer size nor a range of them, so
    /// the ring can't be sized from it.
    BufferSize,
    /// The platform refused the device or its stream.
    Unavailable(String),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { device, audio } => {
                write!(f, "the device plays {device:?}, the audio is {audio:?}")
            }
            Self::SampleType => f.write_str("the device doesn't play binary32 samples"),
            Self::BufferSize => f.write_str("the device doesn't report its buffer size"),
            Self::Unavailable(reason) => write!(f, "the device is unavailable: {reason}"),
        }
    }
}

impl std::error::Error for DeviceError {}

/// An audio output.
pub trait Device {
    fn format(&self) -> DeviceFormat;

    /// Starts playing prepared audio.
    fn play(&mut self, audio: &PreparedAudio) -> Result<(), DeviceError>;
}

/// The stream for `audio`, where the device plays its format.
pub fn stream_for(device: DeviceFormat, audio: &PreparedAudio) -> Result<Stream, DeviceError> {
    let wanted = DeviceFormat {
        sample_rate: audio.sample_rate(),
        channels: audio.layout().channels(),
    };
    if wanted != device {
        return Err(DeviceError::Unsupported {
            device,
            audio: wanted,
        });
    }
    Ok(audio.stream())
}

/// A device that calls a ring's callback once for each of its periods, as a
/// hardware callback does, and keeps what it played.
#[derive(Clone, Debug)]
pub struct SimulatedDevice {
    format: DeviceFormat,
    period_frames: usize,
    played: Vec<f32>,
    underruns: u64,
    missing_frames: u64,
    events: Vec<PlaybackEvent>,
}

impl SimulatedDevice {
    /// A device of `format` whose callback asks for `period_frames` frames.
    pub fn new(format: DeviceFormat, period_frames: usize) -> Self {
        Self {
            format,
            period_frames: period_frames.max(1),
            played: Vec::new(),
            underruns: 0,
            missing_frames: 0,
            events: Vec::new(),
        }
    }

    /// The samples played, the silence of each underrun among them, without
    /// the silence after the audio's end.
    pub fn played(&self) -> &[f32] {
        &self.played
    }

    /// How many periods found the ring short.
    pub fn underruns(&self) -> u64 {
        self.underruns
    }

    /// How many frames of silence the underruns played.
    pub fn missing_frames(&self) -> u64 {
        self.missing_frames
    }

    /// The events the playback reported, in order.
    pub fn events(&self) -> &[PlaybackEvent] {
        &self.events
    }

    /// Plays `audio` through a ring of `ring_frames` frames, and calls
    /// `feed` with the ring's reader before each period, as a reader thread
    /// that runs at its own pace fills the ring.
    pub fn play_through(
        &mut self,
        audio: &PreparedAudio,
        ring_frames: usize,
        mut feed: impl FnMut(&mut Reader),
    ) -> Result<(), DeviceError> {
        let stream = stream_for(self.format, audio)?;
        let channels = self.format.channels;
        let Ring {
            mut reader,
            mut callback,
            mut events,
        } = ring(stream, channels, ring_frames);
        let channels = usize::from(channels.max(1));
        let mut period = vec![0.0f32; self.period_frames * channels];
        loop {
            feed(&mut reader);
            let render = callback.render(&mut period);
            while let Some(event) = events.pop() {
                self.events.push(event);
            }
            match render {
                Render::Played | Render::Underrun { .. } => {
                    self.played.extend_from_slice(&period);
                }
                Render::Ended { played_frames } => {
                    self.played
                        .extend_from_slice(&period[..played_frames * channels]);
                    break;
                }
            }
        }
        let counters = events.counters();
        self.underruns = counters.underruns();
        self.missing_frames = counters.missing_frames();
        Ok(())
    }
}

impl Device for SimulatedDevice {
    fn format(&self) -> DeviceFormat {
        self.format
    }

    fn play(&mut self, audio: &PreparedAudio) -> Result<(), DeviceError> {
        let ring_frames = self.period_frames * crate::ring::DEFAULT_PERIODS_AHEAD;
        self.play_through(audio, ring_frames, |reader| {
            reader.fill(usize::MAX);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use euphony_daw_sample::ChannelLayout;

    const FORMAT: DeviceFormat = DeviceFormat {
        sample_rate: 48_000,
        channels: 2,
    };

    /// Twelve stereo frames whose samples count up from 1.
    fn audio() -> PreparedAudio {
        let samples = (0..24).map(|i| (i + 1) as f32).collect();
        PreparedAudio::new(48_000, ChannelLayout::Stereo, samples)
    }

    fn assert_bits(played: &[f32], expected: &[f32]) {
        assert_eq!(played.len(), expected.len());
        for (played, expected) in played.iter().zip(expected) {
            assert_eq!(played.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn a_reader_short_of_each_period_plays_silence_and_counts_each_underrun() {
        let prepared = audio();
        let mut device = SimulatedDevice::new(FORMAT, 4);
        // The reader brings three frames before each period of four.
        let played = device.play_through(&prepared, 8, |reader| {
            reader.fill(3);
        });
        played.unwrap();

        let mut expected = Vec::new();
        for period in prepared.samples().chunks(6) {
            expected.extend_from_slice(period);
            expected.extend_from_slice(&[0.0, 0.0]);
        }
        expected.truncate(expected.len() - 2);
        assert_bits(device.played(), &expected);
        assert_eq!(device.underruns(), 3);
        assert_eq!(device.missing_frames(), 3);
        let underrun = PlaybackEvent::Underrun { missing_frames: 1 };
        assert_eq!(
            device.events(),
            [underrun, underrun, underrun, PlaybackEvent::Ended]
        );
    }

    #[test]
    fn an_empty_ring_plays_a_whole_period_of_silence() {
        let prepared = audio();
        let mut device = SimulatedDevice::new(FORMAT, 4);
        let mut first = true;
        let played = device.play_through(&prepared, 16, |reader| {
            if !core::mem::take(&mut first) {
                reader.fill(usize::MAX);
            }
        });
        played.unwrap();

        let mut expected = vec![0.0f32; 8];
        expected.extend_from_slice(prepared.samples());
        assert_bits(device.played(), &expected);
        assert_eq!(device.underruns(), 1);
        assert_eq!(device.missing_frames(), 4);
        let underrun = PlaybackEvent::Underrun { missing_frames: 4 };
        assert_eq!(device.events(), [underrun, PlaybackEvent::Ended]);
    }

    #[test]
    fn a_reader_that_keeps_up_plays_every_bit_without_an_underrun() {
        let prepared = audio();
        let mut device = SimulatedDevice::new(FORMAT, 5);
        device.play(&prepared).unwrap();
        assert_bits(device.played(), prepared.samples());
        assert_eq!(device.underruns(), 0);
        assert_eq!(device.events(), [PlaybackEvent::Ended]);
    }
}
