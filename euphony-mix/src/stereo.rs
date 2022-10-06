#![allow(dead_code)]

use crate::{
    frame::{Frame, N2},
    Mixer, Writer,
};
use euphony_dsp::sample::Sample;

#[derive(Clone, Copy, Debug, Default)]
pub struct Config {}

#[derive(Debug)]
pub struct Stereo<W: Writer> {
    writer: W,
    config: Config,
}

impl<W: Writer> Stereo<W> {
    #[inline]
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            config: Default::default(),
        }
    }

    #[inline]
    pub fn with_config(writer: W, config: Config) -> Self {
        Self { writer, config }
    }

    #[inline]
    pub fn finish(self) -> W {
        self.writer
    }
}

impl<W: Writer> Mixer for Stereo<W>
where
    W::Frame: Frame<NumChannels = N2>,
{
    type Error = W::Error;

    #[inline]
    fn skip(&mut self, frames: usize) -> Result<(), Self::Error> {
        self.writer.skip(frames)
    }

    #[inline]
    fn mix(&mut self, samples: &[crate::SpatialSample]) -> Result<(), Self::Error> {
        let mut frame = [0.0f64, 0.0f64];
        for s in samples.iter() {
            // TODO use the bformat
            let x = 0.5;
            frame[0] += s.value * x;
            frame[1] += s.value * x;
        }

        let frame = W::Frame::from_fn(|idx| unsafe { frame.get_unchecked(idx).to_sample() });
        self.writer.write(frame)?;

        Ok(())
    }
}
