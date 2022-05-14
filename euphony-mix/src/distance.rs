use crate::{bformat, frame::Frame, sample::Sample, Mixer, Writer};

#[derive(Debug)]
pub struct Distance<const CHANNELS: usize, W: Writer> {
    writer: W,
    config: [bformat::Weights; CHANNELS],
}

impl<const CHANNELS: usize, W: Writer> Distance<CHANNELS, W> {
    #[inline]
    pub fn new(writer: W, config: [bformat::Weights; CHANNELS]) -> Self {
        debug_assert_eq!(CHANNELS, W::Frame::CHANNELS);
        Self { writer, config }
    }

    #[inline]
    pub fn finish(self) -> W {
        self.writer
    }
}

impl<const CHANNELS: usize, W: Writer> Mixer for Distance<CHANNELS, W> {
    type Error = W::Error;

    #[inline]
    fn skip(&mut self, frames: usize) -> Result<(), Self::Error> {
        self.writer.skip(frames)
    }

    #[inline]
    fn mix(&mut self, samples: &[crate::SpatialSample]) -> Result<(), Self::Error> {
        debug_assert_eq!(CHANNELS, W::Frame::CHANNELS);

        let mut frame = [0.0f64; CHANNELS];

        for s in samples.iter() {
            for (to, config) in frame.iter_mut().zip(self.config.iter()) {
                *to = config.dot(s);
            }
        }

        let frame = W::Frame::from_fn(|idx| unsafe { frame.get_unchecked(idx).to_sample() });
        self.writer.write(frame)?;

        Ok(())
    }
}
