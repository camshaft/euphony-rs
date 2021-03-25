macro_rules! new_ratio {
    ($name:ident, $inner:ty) => {
        new_ratio_struct!($name, $inner);
        new_ratio_methods!($name, $inner);
        new_ratio_ops!($name, $inner);
        new_ratio_conversions!($name, $inner);
    };
}

macro_rules! new_ratio_struct {
    ($name:ident, $inner:ty) => {
        #[derive(Clone, Copy, Eq, Hash)]
        pub struct $name(pub $inner, pub $inner);

        impl Default for $name {
            fn default() -> Self {
                Self(0, 1)
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}({})", stringify!($name), self)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.as_ratio().fmt(f)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == core::cmp::Ordering::Equal
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(&other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.as_ratio().cmp(&other.as_ratio())
            }
        }
    };
}

macro_rules! new_ratio_methods {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new<Value: Into<Self>>(value: Value) -> Self {
                value.into()
            }

            pub fn reduce(self) -> Self {
                self.as_ratio().reduce().into()
            }

            pub fn simplify(self, denominator: $inner) -> Self {
                self.as_ratio().simplify(denominator).into()
            }

            pub fn truncate(self) -> Self {
                self.as_ratio().truncate().into()
            }

            pub fn is_whole(&self) -> bool {
                self.as_ratio().is_whole()
            }

            pub fn whole(self) -> $inner {
                self.as_ratio().whole()
            }

            pub fn try_into_whole(&self) -> Option<$inner> {
                if self.is_whole() {
                    Some(self.whole())
                } else {
                    None
                }
            }

            pub fn fraction(self) -> Self {
                self.as_ratio().fraction().into()
            }

            pub fn as_ratio(self) -> $crate::ratio::Ratio<$inner> {
                $crate::ratio::Ratio(self.0, self.1)
            }
        }
    };
}

macro_rules! new_ratio_ops {
    ($name:ident, $inner:ty) => {
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
            type Output = $crate::ratio::Ratio<$inner>;

            fn div(self, rhs: Self) -> Self::Output {
                self.as_ratio().div(rhs.as_ratio())
            }
        }
    };
}

macro_rules! new_ratio_conversions {
    ($name:ident, $inner:ty) => {
        new_ratio_conversion!($name, $inner, i8);
        new_ratio_conversion!($name, $inner, u8);
        new_ratio_conversion!($name, $inner, i16);
        new_ratio_conversion!($name, $inner, u16);
        new_ratio_conversion!($name, $inner, i32);
        new_ratio_conversion!($name, $inner, u32);
        new_ratio_conversion!($name, $inner, i64);
        new_ratio_conversion!($name, $inner, u64);
        new_ratio_conversion!($name, $inner, isize);
        new_ratio_conversion!($name, $inner, usize);
    };
}

macro_rules! new_ratio_conversion {
    ($name:ident, $inner:ty, $ty:ident) => {
        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                use core::convert::TryInto;
                Self(
                    value
                        .try_into()
                        .expect(concat!("value should fit into a ", stringify!($inner))),
                    1,
                )
            }
        }

        impl From<$crate::ratio::Ratio<$ty>> for $name {
            fn from(value: $crate::ratio::Ratio<$ty>) -> Self {
                use core::convert::TryInto;
                Self(
                    (value.0)
                        .try_into()
                        .expect(concat!("value should fit into a ", stringify!($inner))),
                    (value.1)
                        .try_into()
                        .expect(concat!("value should fit into a ", stringify!($inner))),
                )
            }
        }

        impl From<($ty, $ty)> for $name {
            fn from(value: ($ty, $ty)) -> Self {
                let value: $crate::ratio::Ratio<$ty> = value.into();
                value.into()
            }
        }

        impl PartialEq<$ty> for $name {
            fn eq(&self, other: &$ty) -> bool {
                self.partial_cmp(other) == Some(core::cmp::Ordering::Equal)
            }
        }

        impl PartialOrd<$ty> for $name {
            fn partial_cmp(&self, other: &$ty) -> Option<core::cmp::Ordering> {
                use core::{cmp::Ordering::*, convert::TryInto};
                let other: $inner = (*other).try_into().ok()?;
                if self.1 == 1 {
                    self.0.partial_cmp(&other)
                } else {
                    Some(match (self.0 / self.1).partial_cmp(&other)? {
                        Equal | Greater => Greater,
                        Less => Less,
                    })
                }
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

        impl core::ops::$op<$crate::ratio::Ratio<$ty>> for $name {
            type Output = $name;

            fn $call(self, rhs: $crate::ratio::Ratio<$ty>) -> Self {
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

        impl core::ops::$assign_op<$crate::ratio::Ratio<$ty>> for $name {
            fn $assign(&mut self, rhs: $crate::ratio::Ratio<$ty>) {
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
