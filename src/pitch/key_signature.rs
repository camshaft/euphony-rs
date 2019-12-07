use crate::pitch::{
    interval::{AbsoluteInterval, Interval},
    mode::Mode,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct KeySignature(pub Interval, pub Mode);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AbsoluteKeySignature(pub AbsoluteInterval, pub Mode);
