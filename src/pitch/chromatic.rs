use crate::pitch::interval::{AbsoluteInterval, Interval};

pub const A: AbsoluteInterval = AbsoluteInterval(0, 12);
pub const B: AbsoluteInterval = AbsoluteInterval(2, 12);
pub const C: AbsoluteInterval = AbsoluteInterval(3, 12);
pub const D: AbsoluteInterval = AbsoluteInterval(5, 12);
pub const E: AbsoluteInterval = AbsoluteInterval(7, 12);
pub const F: AbsoluteInterval = AbsoluteInterval(8, 12);
pub const G: AbsoluteInterval = AbsoluteInterval(10, 12);

pub const SEMITONE: Interval = Interval(1, 12);
pub const WHOLETONE: Interval = Interval(2, 12);

pub trait ChromaticInterval {
    fn sharp(self) -> Self;
    fn flat(self) -> Self;
}

impl ChromaticInterval for AbsoluteInterval {
    fn sharp(self) -> Self {
        self + SEMITONE
    }

    fn flat(self) -> Self {
        self - SEMITONE
    }
}

#[test]
fn modifier_test() {
    assert_eq!(A.sharp(), B.flat());
}
