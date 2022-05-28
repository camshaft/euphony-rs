use crate::{manifest::Manifest, Result};
use anyhow::anyhow;
use cpal::{traits::*, Device, Host, SampleFormat, SampleRate, SupportedStreamConfig};
use std::path::Path;
use structopt::StructOpt;

// TODO if remote file, download, and subscribe

mod stream;
mod ui;

#[derive(Debug, StructOpt)]
pub struct Play {
    #[structopt(long, short)]
    host: Option<String>,

    #[structopt(long, short)]
    device: Option<String>,

    #[structopt(long, short)]
    project: Option<String>,

    #[structopt(long, short)]
    channels: Option<u16>,

    #[structopt(long)]
    paused: bool,

    input: Option<String>,
}

impl Play {
    pub fn run(&self) -> Result<()> {
        let host = self.host()?;
        let device = self.device(&host)?;
        let config = self.device_config(&device)?;
        let stream = self.watch(&device, &config)?;

        if !self.paused {
            stream.play()?;
        }

        ui::start(stream)?;

        Ok(())
    }

    fn host(&self) -> Result<Host> {
        Ok(if let Some(name) = self.host.as_ref() {
            let id = cpal::available_hosts()
                .into_iter()
                .find(|id| id.name() == name)
                .ok_or_else(|| anyhow!("Invalid audio host name {:?}", name))?;
            cpal::host_from_id(id)?
        } else {
            cpal::default_host()
        })
    }

    fn device(&self, host: &Host) -> Result<Device> {
        if let Some(name) = self.device.as_ref() {
            for device in host.devices()? {
                if &device.name()? == name {
                    return Ok(device);
                }
            }

            Err(anyhow!("could not find device {:?}", name))
        } else {
            host.default_output_device()
                .ok_or_else(|| anyhow!("selected host does not have a default output device"))
        }
    }

    fn watch(&self, device: &Device, config: &SupportedStreamConfig) -> Result<stream::Stream> {
        if self.input.as_ref().map_or(false, |v| v.starts_with("http")) {
            // TODO do the http thing
            todo!();
        }

        let path = self.input.as_ref().map(Path::new);
        let mut manifest = Manifest::new(path, None)?;
        self.manifest_project(&mut manifest)?;

        let stream = stream::Stream::with_manifest(device, config, manifest)?;

        Ok(stream)
    }

    fn manifest_project(&self, manifest: &mut Manifest) -> Result<()> {
        if let Some(project) = self.project.as_ref() {
            manifest.set_project(project)?;
        } else {
            let project = manifest
                .default_project()
                .ok_or_else(|| anyhow!("no euphony projects found in {}", manifest.root.display()))?
                .to_owned();
            manifest.set_project(project)?;
        }

        Ok(())
    }

    fn device_config(&self, device: &Device) -> Result<SupportedStreamConfig> {
        let mut config = device.default_output_config().ok();

        const SAMPLE_RATE: SampleRate = SampleRate(48_000);

        fn is_preferred(config: &SupportedStreamConfig, channels: Option<u16>) -> bool {
            if let Some(channels) = channels {
                if config.channels() != channels {
                    return false;
                }
            }

            config.sample_rate() == SAMPLE_RATE && config.sample_format() == SampleFormat::F32
        }

        // if it's not ideal, then search for a better one
        if config
            .as_ref()
            .map_or(true, |c| !is_preferred(c, self.channels))
        {
            let channels = self
                .channels
                .unwrap_or_else(|| config.as_ref().map(|c| c.channels()).unwrap_or(2u16));

            for conf in device.supported_output_configs()? {
                let range = conf.min_sample_rate()..=conf.max_sample_rate();
                if !range.contains(&SAMPLE_RATE) {
                    continue;
                }

                let conf = conf.with_sample_rate(SAMPLE_RATE);
                if is_preferred(&conf, Some(channels)) {
                    config = Some(conf);
                    break;
                }
            }
        }

        let config =
            config.ok_or_else(|| anyhow!("could not find an acceptable output configuration"))?;

        Ok(config)
    }
}
