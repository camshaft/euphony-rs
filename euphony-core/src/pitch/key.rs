use crate::pitch::{interval::Interval, mode::Mode};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Key(pub Interval, pub Mode);
