use core::ops;
use euphony_dsp::sample::{DefaultRate as Rate, Rate as _};
use euphony_units::{
    ratio::Ratio,
    time::{Beat, Tempo},
};

#[inline]
pub fn default_samples_per_tick() -> Ratio<u128> {
    let duration = Tempo::DEFAULT * Beat::DEFAULT_RESOLUTION;
    samples_per_tick(duration.as_nanos() as _)
}

#[inline]
pub fn samples_per_tick(nanos_per_tick: u64) -> Ratio<u128> {
    let Ratio(a, b) = Rate::NANOS_PER_SAMPLE;
    Ratio(nanos_per_tick as u128, 1) / Ratio(a as u128, b as u128)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sample(u64);

impl Sample {
    #[inline]
    pub fn since(self, prev: Self) -> RelSample {
        RelSample(self.0 - prev.0)
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    #[inline]
    pub fn checked_add(&mut self, samples: u64) -> Option<Self> {
        Some(Self(self.0.checked_add(samples)?))
    }

    #[inline]
    pub fn to_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl ops::Add<RelSample> for Sample {
    type Output = Self;

    fn add(self, rhs: RelSample) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign<u64> for Sample {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl From<Sample> for u64 {
    fn from(sample: Sample) -> u64 {
        sample.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelSample(u64);

impl RelSample {
    #[inline]
    pub fn to_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl From<RelSample> for u64 {
    fn from(sample: RelSample) -> u64 {
        sample.0
    }
}
