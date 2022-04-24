pub use dasp_sample::*;
use euphony_units::ratio::Ratio;

pub type DefaultSample = f64;
pub type DefaultRate = Rate48000;

pub trait Rate: 'static + Send + Sync {
    const PERIOD: f64;
    const VALUE: f64;
    const COUNT: u64;
    const NANOS_PER_SAMPLE: Ratio<u64>;
}

pub struct Rate44100;

impl Rate for Rate44100 {
    const PERIOD: f64 = 1.0f64 / 44100.0;
    const VALUE: f64 = 44100.0;
    const COUNT: u64 = 44100;
    const NANOS_PER_SAMPLE: Ratio<u64> = Ratio(10000000, 441);
}

pub struct Rate48000;

impl Rate for Rate48000 {
    const PERIOD: f64 = 1.0f64 / 48000.0;
    const VALUE: f64 = 48000.0;
    const COUNT: u64 = 48000;
    const NANOS_PER_SAMPLE: Ratio<u64> = Ratio(62500, 3);
}
