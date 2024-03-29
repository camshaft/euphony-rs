use crate::pitch::{interval::Interval, mode::Mode};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Key(pub Interval, pub Mode);
