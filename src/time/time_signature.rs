use crate::{
    ratio::Ratio,
    time::{beat::Beat, measure::Measure},
};
use core::convert::TryInto;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TimeSignature(pub u64, pub u64);

impl Default for TimeSignature {
    fn default() -> Self {
        Self(4, 4)
    }
}

impl TimeSignature {
    pub fn new<T: Into<Self>>(value: T) -> Self {
        value.into()
    }

    pub fn beat(&self) -> Beat {
        Beat(1, self.1)
    }

    pub fn count(&self) -> u64 {
        self.0
    }

    pub fn total_beats(&self) -> Beat {
        Beat(self.0, self.1)
    }

    fn as_ratio(self) -> Ratio<u64> {
        Ratio(self.0, self.1)
    }
}

impl core::ops::Mul<Measure> for TimeSignature {
    type Output = Beat;

    fn mul(self, measure: Measure) -> Self::Output {
        let count = self.as_ratio() * measure.as_ratio();
        (count / self.beat().as_ratio()).into()
    }
}

#[test]
fn mul_measure_test() {
    assert_eq!(TimeSignature(4, 4) * Measure(1, 4), Beat(1, 1));
    assert_eq!(TimeSignature(4, 4) * Measure(2, 4), Beat(2, 1));
    assert_eq!(TimeSignature(7, 8) * Measure(2, 1), Beat(14, 1));
    assert_eq!(TimeSignature(6, 8) * Measure(1, 3), Beat(2, 1));
    assert_eq!(TimeSignature(6, 8) * Measure(2, 3), Beat(4, 1));
}

macro_rules! convert {
    ($ty:ty) => {
        impl From<($ty, $ty)> for TimeSignature {
            fn from((n, d): ($ty, $ty)) -> Self {
                Self(n.try_into().unwrap(), d.try_into().unwrap())
            }
        }
    };
}

convert!(i8);
convert!(u8);
convert!(i16);
convert!(u16);
convert!(i32);
convert!(u32);
convert!(i64);
convert!(u64);
convert!(isize);
convert!(usize);
