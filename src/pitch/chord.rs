use crate::pitch::{
    frequency::{Frequency, FrequencyRatio},
    interval::AbsoluteInterval,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Chord {
    pub base: Frequency,
    pub interval: AbsoluteInterval,
    pub system: ChordSystem,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ChordStep {
    Ratio(FrequencyRatio),
    Cents(f64), // TODO make a specific type
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ChordSystem(&'static [ChordStep]);
