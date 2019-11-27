use core::time::Duration;
pub use num_rational::Ratio;

pub(crate) fn mul_duration(duration: Duration, ratio: Ratio<u64>) -> Duration {
    let whole_duration = duration * ratio.to_integer() as u32;
    let (numer, denom) = ratio.fract().into();
    let fract_duration = duration / (denom as u32) * (numer as u32);
    whole_duration + fract_duration
}

pub(crate) fn div_duration(duration: Duration, ratio: Ratio<u64>) -> Duration {
    let (numer, denom) = ratio.into();
    if numer == 0 {
        return Duration::from_secs(0);
    }
    duration / (numer as u32) * (denom as u32)
}

#[test]
fn div_duration_test() {
    assert_eq!(
        div_duration(Duration::from_secs(1), Ratio::new(4, 3)),
        Duration::from_millis(750)
    );
}

macro_rules! new_ratio {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub struct $name(pub u64, pub u64);

        impl $name {
            pub fn new<Value: Into<Self>>(value: Value) -> Self {
                value.into()
            }

            pub(crate) fn as_ratio(self) -> $crate::time::ratio::Ratio<u64> {
                $crate::time::ratio::Ratio::new_raw(self.0, self.1)
            }
        }

        impl core::ops::Add for $name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self {
                self.as_ratio().add(rhs.as_ratio()).into()
            }
        }

        impl core::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                *self = core::ops::Add::add(*self, rhs);
            }
        }

        impl core::ops::Sub for $name {
            type Output = $name;

            fn sub(self, rhs: Self) -> Self {
                self.as_ratio().sub(rhs.as_ratio()).into()
            }
        }

        impl core::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = core::ops::Sub::sub(*self, rhs);
            }
        }

        impl core::ops::Div for $name {
            type Output = $crate::time::ratio::Ratio<u64>;

            fn div(self, rhs: Self) -> Self::Output {
                self.as_ratio().div(rhs.as_ratio())
            }
        }

        new_ratio_conversion!($name, i8);
        new_ratio_conversion!($name, u8);
        new_ratio_conversion!($name, i16);
        new_ratio_conversion!($name, u16);
        new_ratio_conversion!($name, i32);
        new_ratio_conversion!($name, u32);
        new_ratio_conversion!($name, i64);
        new_ratio_conversion!($name, u64);
        new_ratio_conversion!($name, isize);
        new_ratio_conversion!($name, usize);
    };
}

macro_rules! new_ratio_conversion {
    ($name:ident, $ty:ident) => {
        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                use core::convert::TryInto;
                Self(value.try_into().expect("value should fit into a u64"), 1)
            }
        }

        impl From<$crate::time::ratio::Ratio<$ty>> for $name {
            fn from(value: $crate::time::ratio::Ratio<$ty>) -> Self {
                use core::convert::TryInto;
                Self(
                    (*value.numer())
                        .try_into()
                        .expect("value should fit into a u64"),
                    (*value.denom())
                        .try_into()
                        .expect("value should fit into a u64"),
                )
            }
        }

        impl From<($ty, $ty)> for $name {
            fn from(value: ($ty, $ty)) -> Self {
                let value: $crate::time::ratio::Ratio<$ty> = value.into();
                value.into()
            }
        }

        new_ratio_arithmetic!($name, Add, add, AddAssign, add_assign, $ty);
        new_ratio_arithmetic!($name, Sub, sub, SubAssign, sub_assign, $ty);
        new_ratio_arithmetic!($name, Mul, mul, MulAssign, mul_assign, $ty);
        new_ratio_arithmetic!($name, Div, div, DivAssign, div_assign, $ty);
    };
}

macro_rules! new_ratio_arithmetic {
    ($name:ident, $op:ident, $call:ident, $assign_op:ident, $assign:ident, $ty:ident) => {
        impl core::ops::$op<$ty> for $name {
            type Output = $name;

            fn $call(self, rhs: $ty) -> Self {
                let rhs: Self = rhs.into();
                self.as_ratio().$call(rhs.as_ratio()).into()
            }
        }

        impl core::ops::$op<$crate::time::ratio::Ratio<$ty>> for $name {
            type Output = $name;

            fn $call(self, rhs: $crate::time::ratio::Ratio<$ty>) -> Self {
                let rhs: Self = rhs.into();
                self.as_ratio().$call(rhs.as_ratio()).into()
            }
        }

        impl core::ops::$op<($ty, $ty)> for $name {
            type Output = $name;

            fn $call(self, rhs: ($ty, $ty)) -> Self {
                let rhs: Self = rhs.into();
                self.as_ratio().$call(rhs.as_ratio()).into()
            }
        }

        impl core::ops::$assign_op<$ty> for $name {
            fn $assign(&mut self, rhs: $ty) {
                *self = core::ops::$op::$call(*self, rhs);
            }
        }

        impl core::ops::$assign_op<$crate::time::ratio::Ratio<$ty>> for $name {
            fn $assign(&mut self, rhs: $crate::time::ratio::Ratio<$ty>) {
                *self = core::ops::$op::$call(*self, rhs);
            }
        }

        impl core::ops::$assign_op<($ty, $ty)> for $name {
            fn $assign(&mut self, rhs: ($ty, $ty)) {
                *self = core::ops::$op::$call(*self, rhs);
            }
        }
    };
}
