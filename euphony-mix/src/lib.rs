pub use dasp_frame as frame;
use euphony_dsp::sample::{self, DefaultSample as Sample, FromSample};
use euphony_units::coordinates::Cartesian;

mod bformat;
pub mod distance;
pub mod mono;
pub mod stereo;

#[derive(Clone, Copy, Debug, Default)]
pub struct SpatialSample<Coordinate = Cartesian<Sample>> {
    pub value: Sample,
    pub coordinate: Coordinate,
}

pub trait Mixer {
    type Error;

    fn skip(&mut self, samples: usize) -> Result<(), Self::Error>;
    fn mix(&mut self, samples: &[SpatialSample]) -> Result<(), Self::Error>;
}

pub trait Writer {
    type Error;
    type Sample: sample::Sample + FromSample<Sample>;
    type Frame: frame::Frame<Sample = Self::Sample>;

    fn skip(&mut self, frames: usize) -> Result<(), Self::Error>;
    fn write(&mut self, frame: Self::Frame) -> Result<(), Self::Error>;
}
