use crate::time::{beat::Beat, measure::Measure, time_signature::TimeSignature};
use core::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CompoundDuration {
    pub measure: Measure,
    pub beat: Beat,
}

macro_rules! compound_conversion {
    ($ty:ident, $field:ident, $other_ty:ident, $other_field:ident) => {
        impl From<$ty> for CompoundDuration {
            fn from($field: $ty) -> Self {
                Self {
                    $other_field: Default::default(),
                    $field,
                }
            }
        }

        impl Add<$ty> for CompoundDuration {
            type Output = CompoundDuration;

            fn add(self, $field: $ty) -> Self::Output {
                CompoundDuration {
                    $field: self.$field + $field,
                    $other_field: self.$other_field,
                }
            }
        }

        impl AddAssign<$ty> for CompoundDuration {
            fn add_assign(&mut self, $field: $ty) {
                self.$field += $field;
            }
        }

        impl Sub<$ty> for CompoundDuration {
            type Output = CompoundDuration;

            fn sub(self, $field: $ty) -> Self::Output {
                CompoundDuration {
                    $field: self.$field - $field,
                    $other_field: self.$other_field,
                }
            }
        }

        impl SubAssign<$ty> for CompoundDuration {
            fn sub_assign(&mut self, $field: $ty) {
                self.$field -= $field;
            }
        }

        impl Add<$ty> for $other_ty {
            type Output = CompoundDuration;

            fn add(self, value: $ty) -> Self::Output {
                let compound: CompoundDuration = self.into();
                compound + value
            }
        }

        impl Sub<$ty> for $other_ty {
            type Output = CompoundDuration;

            fn sub(self, value: $ty) -> Self::Output {
                let compound: CompoundDuration = self.into();
                compound - value
            }
        }
    };
}

compound_conversion!(Beat, beat, Measure, measure);
compound_conversion!(Measure, measure, Beat, beat);

impl Mul<TimeSignature> for CompoundDuration {
    type Output = Beat;

    fn mul(self, time_signature: TimeSignature) -> Self::Output {
        self.beat * time_signature + self.measure * time_signature
    }
}
