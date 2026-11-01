//! Playback through the platform's default output device.

use crate::{device::stream_for, Device, DeviceError, DeviceFormat, PreparedAudio};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// The platform's default output device.
pub struct NativeDevice {
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

fn unavailable(error: impl ToString) -> DeviceError {
    DeviceError::Unavailable(error.to_string())
}

impl NativeDevice {
    /// Opens the default output device in the format it reports.
    pub fn open_default() -> Result<Self, DeviceError> {
        let host = cpal::default_host();
        let device = host.default_output_device();
        let device = device.ok_or_else(|| unavailable("there is no output device"))?;
        let config = device.default_output_config().map_err(unavailable)?;
        Ok(Self {
            device,
            config,
            stream: None,
        })
    }
}

impl Device for NativeDevice {
    fn format(&self) -> DeviceFormat {
        DeviceFormat {
            sample_rate: self.config.sample_rate().0,
            channels: self.config.channels(),
        }
    }

    fn play(&mut self, audio: &PreparedAudio) -> Result<(), DeviceError> {
        if self.config.sample_format() != cpal::SampleFormat::F32 {
            return Err(DeviceError::SampleType);
        }
        let stream = stream_for(self.format(), audio)?;
        let config: cpal::StreamConfig = self.config.clone().into();
        let data = move |out: &mut [f32], _: &cpal::OutputCallbackInfo| {
            stream.fill(out);
        };
        let error = |error: cpal::StreamError| eprintln!("the output stream failed: {error}");
        let built = self.device.build_output_stream(&config, data, error, None);
        let built = built.map_err(unavailable)?;
        built.play().map_err(unavailable)?;
        self.stream = Some(built);
        Ok(())
    }
}
