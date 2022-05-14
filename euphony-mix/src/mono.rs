use crate::{frame::Frame, Mixer, Writer};
use euphony_dsp::sample::Sample;

#[derive(Debug)]
pub struct Mono<W: Writer>(W);

impl<W: Writer> Mono<W> {
    #[inline]
    pub fn new(w: W) -> Self {
        Self(w)
    }

    #[inline]
    pub fn finish(self) -> W {
        self.0
    }
}

impl<W: Writer> Mixer for Mono<W> {
    type Error = W::Error;

    #[inline]
    fn skip(&mut self, frames: usize) -> Result<(), Self::Error> {
        self.0.skip(frames)
    }

    #[inline]
    fn mix(&mut self, samples: &[crate::SpatialSample]) -> Result<(), Self::Error> {
        let mut sample = 0.0f64;
        for s in samples.iter() {
            sample += s.value;
        }
        let sample: W::Sample = sample.to_sample();
        let frame = W::Frame::from_fn(|_| sample);
        self.0.write(frame)?;
        Ok(())
    }
}
