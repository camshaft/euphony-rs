use crate::{PreparedAudio, Stream};
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
    /// The device plays another format than the audio's: playback doesn't
    /// resample or remap channels silently.
    Unsupported {
        device: DeviceFormat,
        audio: DeviceFormat,
    },
    /// The device plays samples of another type than binary32.
    SampleType,
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

/// A device that calls the stream once for each of its periods, as a
/// hardware callback does, and keeps what it played.
#[derive(Clone, Debug)]
pub struct SimulatedDevice {
    format: DeviceFormat,
    period_frames: usize,
    played: Vec<f32>,
}

impl SimulatedDevice {
    /// A device of `format` whose callback asks for `period_frames` frames.
    pub fn new(format: DeviceFormat, period_frames: usize) -> Self {
        Self {
            format,
            period_frames: period_frames.max(1),
            played: Vec::new(),
        }
    }

    /// The samples played, without the silence after the audio's end.
    pub fn played(&self) -> &[f32] {
        &self.played
    }
}

impl Device for SimulatedDevice {
    fn format(&self) -> DeviceFormat {
        self.format
    }

    fn play(&mut self, audio: &PreparedAudio) -> Result<(), DeviceError> {
        let stream = stream_for(self.format, audio)?;
        let channels = usize::from(self.format.channels);
        let mut period = vec![0.0f32; self.period_frames * channels];
        loop {
            let count = stream.fill(&mut period);
            self.played.extend_from_slice(&period[..count]);
            if stream.finished() {
                return Ok(());
            }
        }
    }
}
