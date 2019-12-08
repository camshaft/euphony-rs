//! https://en.wikipedia.org/wiki/Chromatic_scale

use crate::pitch::interval::Interval;

named_interval!(A(0, 12));
named_interval!(B(2, 12));
named_interval!(C(3, 12));
named_interval!(D(5, 12));
named_interval!(E(7, 12));
named_interval!(F(8, 12));
named_interval!(G(10, 12));

named_interval!(SEMITONE(1, 12));
named_interval!(WHOLETONE(2, 12));

pub trait ChromaticInterval {
    fn sharp(self) -> Self;
    fn flat(self) -> Self;
}

impl ChromaticInterval for Interval {
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
