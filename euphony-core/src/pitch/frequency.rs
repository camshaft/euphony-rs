use core::time::Duration;

// TODO

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Frequency(Period);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Period(Duration);

new_ratio!(FrequencyRatio, u64);
