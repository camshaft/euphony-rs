use crate::{build::Build, Result};
use euphony_compiler::{sample, Hash};
use euphony_mix::{
    frame::{self, Frame as _},
    mono::Mono,
    stereo::Stereo,
};
use euphony_store::Store;
use std::marker::PhantomData;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Render {
    #[structopt(flatten)]
    build: Build,

    #[structopt(long, default_value = "2")]
    channels: u16,
}

impl Render {
    pub fn run(&self) -> Result<()> {
        let comps = self.build.build()?;
        for comp in comps {
            let store = comp.store();

            let buffer = match self.channels {
                1 => {
                    let mut mixer = Mixer::<1, f32>::default();
                    mixer.render(store)
                }
                2 => {
                    let mut mixer = Mixer::<2, f32>::default();
                    mixer.render(store)
                }
                channels => {
                    return Err(anyhow::anyhow!("Invalid number of channels: {}", channels));
                }
            };

            let mut path = comp.timeline_path().to_owned();
            path.set_extension("wav");

            let spec = hound::WavSpec {
                channels: self.channels,
                sample_rate: 48_000,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            let mut writer = hound::WavWriter::create(&path, spec)?;
            for sample in buffer {
                writer.write_sample(sample)?;
            }
            writer.finalize()?;
            log::info!("rendered {:?}", path);
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Mixer<const CHANNELS: usize, Sample: Copy> {
    samples: PhantomData<Sample>,
}

impl<Sample> Mixer<1, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64> + core::ops::AddAssign + Send + Sync,
{
    fn render(&mut self, store: &Store) -> Vec<Sample> {
        self.render_inner(store, |hash, writer, store| {
            let mut mixer = Mono::new(writer);

            if let Err(err) = store.mix_group(hash, &mut mixer) {
                log::error!("could not mix group: {}", err);
            }

            mixer.finish()
        })
    }
}

impl<Sample> Mixer<2, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64> + core::ops::AddAssign + Send + Sync,
{
    fn render(&mut self, store: &Store) -> Vec<Sample> {
        self.render_inner(store, |hash, writer, store| {
            let mut mixer = Stereo::new(writer);

            if let Err(err) = store.mix_group(hash, &mut mixer) {
                log::error!("could not mix group: {}", err);
            }

            mixer.finish()
        })
    }
}

impl<const CHANNELS: usize, Sample> Mixer<CHANNELS, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64> + Send + Sync,
{
    fn render_inner<F>(&mut self, store: &Store, mix: F) -> Vec<Sample>
    where
        F: Fn(&Hash, TrackWriter<CHANNELS, Sample>, &Store) -> TrackWriter<CHANNELS, Sample> + Sync,
    {
        let mut writer = <TrackWriter<CHANNELS, Sample>>::new();
        for group in store.timeline.groups.iter() {
            writer = mix(&group.entries, writer, store);
            writer.cursor = 0;
        }
        writer.buffer
    }
}

struct TrackWriter<const CHANNELS: usize, Sample: Copy> {
    buffer: Vec<Sample>,
    cursor: usize,
}

impl<const CHANNELS: usize, Sample: euphony_compiler::sample::Sample>
    TrackWriter<CHANNELS, Sample>
{
    #[inline]
    fn new() -> Self {
        Self {
            buffer: vec![],
            cursor: 0,
        }
    }
}

impl<const CHANNELS: usize, Sample: euphony_compiler::sample::Sample> euphony_mix::Writer
    for TrackWriter<CHANNELS, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64> + core::ops::AddAssign,
    [Sample; CHANNELS]: frame::Frame<Sample = Sample>,
{
    type Error = std::io::Error;
    type Sample = Sample;
    type Frame = [Sample; CHANNELS];

    #[inline]
    fn skip(&mut self, frames: usize) -> Result<(), Self::Error> {
        let samples = CHANNELS * frames;
        self.cursor += samples;

        Ok(())
    }

    #[inline]
    fn write(&mut self, frame: Self::Frame) -> Result<(), Self::Error> {
        let cursor = self.cursor;
        let new_cursor = cursor + CHANNELS;
        if self.buffer.len() <= new_cursor {
            self.buffer.resize(new_cursor, Sample::EQUILIBRIUM);
        }

        for (idx, sample) in frame.channels().enumerate() {
            unsafe { *self.buffer.get_unchecked_mut(cursor + idx) += sample };
        }

        self.cursor = new_cursor;

        Ok(())
    }
}
