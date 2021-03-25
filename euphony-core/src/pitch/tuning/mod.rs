use crate::pitch::{
    frequency::{Frequency, FrequencyRatio},
    interval::Interval,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tuning {
    pub base: Frequency,
    pub interval: Interval,
    pub system: TuningSystem,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TuningStep {
    Ratio(FrequencyRatio),
    Cents(f64), // TODO make a specific type
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TuningSystem(&'static [TuningStep]);
