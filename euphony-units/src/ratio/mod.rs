use core::{cmp::Ordering, fmt, ops::Neg};
use num_integer::Integer;
use num_rational::Ratio as Inner;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero};

#[macro_use]
pub mod macros;

#[derive(Clone, Copy)]
pub struct Ratio<T>(pub T, pub T);

impl<T: Zero + One> Default for Ratio<T> {
    fn default() -> Self {
        Self(T::zero(), T::one())
    }
}

impl<T> fmt::Debug for Ratio<T>
where
    Ratio<T>: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<T: One + PartialEq + fmt::Display> fmt::Display for Ratio<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.1.is_one() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

impl<T: Copy + Integer> PartialEq for Ratio<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ratio().eq(&other.as_ratio())
    }
}

impl<T: Copy + Integer> PartialOrd for Ratio<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Copy + Integer> Eq for Ratio<T> {}

impl<T: Copy + Integer> Ord for Ratio<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ratio().cmp(&other.as_ratio())
    }
}

impl<T: Copy + Integer + core::hash::Hash> core::hash::Hash for Ratio<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl<T: Copy + Integer> Ratio<T> {
    pub fn simplify(self, d: T) -> Self {
        if self.1 < d {
            return self;
        }

        (self * d).round() / d
    }

    pub fn reduce(self) -> Self {
        self.as_ratio().reduced().into()
    }

    pub fn truncate(self) -> Self {
        self.as_ratio().trunc().into()
    }

    pub fn floor(self) -> Self {
        self.as_ratio().floor().into()
    }

    pub fn ceil(self) -> Self {
        self.as_ratio().ceil().into()
    }

    pub fn round(self) -> Self {
        self.as_ratio().round().into()
    }

    pub fn is_whole(&self) -> bool {
        self.as_ratio().is_integer()
    }

    pub fn whole(self) -> T {
        self.as_ratio().to_integer()
    }

    pub fn inverse(self) -> Self {
        Self(self.1, self.0)
    }

    pub fn try_into_whole(&self) -> Option<T> {
        if self.is_whole() {
            Some(self.whole())
        } else {
            None
        }
    }

    pub fn fraction(self) -> Self {
        self.as_ratio().fract().into()
    }

    fn as_ratio(self) -> Inner<T> {
        Inner::new_raw(self.0, self.1)
    }
}

macro_rules! checked {
    ($name:ident, $($constraint:ident),*) => {
        impl<T: Copy + Integer $(+ $constraint)*> Ratio<T> {
            pub fn $name(self, other: Self) -> Option<Self> {
                Some(self.as_ratio().$name(&other.as_ratio())?.into())
            }
        }
    };
}

checked!(checked_add, CheckedAdd, CheckedMul);
checked!(checked_sub, CheckedSub, CheckedMul);
checked!(checked_mul, CheckedMul);
checked!(checked_div, CheckedDiv, CheckedMul);

impl<T: Copy + Integer + CheckedDiv + CheckedMul> Ratio<T> {
    pub fn checked_rem(self, other: Self) -> Option<Self> {
        if other.0.is_zero() {
            return None;
        }

        Some((self.as_ratio() % other.as_ratio()).into())
    }
}

macro_rules! inner_ratio_arithmetic {
    ($op:ident, $call:ident, $assign_op:ident, $assign:ident) => {
        impl<T: Copy + Integer> core::ops::$op<T> for Ratio<T> {
            type Output = Ratio<T>;

            fn $call(self, rhs: T) -> Self {
                let rhs: Self = rhs.into();
                self.as_ratio().$call(rhs.as_ratio()).into()
            }
        }

        impl<T: Copy + Integer> core::ops::$op<Ratio<T>> for Ratio<T> {
            type Output = Ratio<T>;

            fn $call(self, rhs: Ratio<T>) -> Self {
                let rhs: Self = rhs.into();
                self.as_ratio().$call(rhs.as_ratio()).into()
            }
        }

        impl<T: Copy + Integer> core::ops::$op<(T, T)> for Ratio<T> {
            type Output = Ratio<T>;

            fn $call(self, rhs: (T, T)) -> Self {
                let rhs: Self = rhs.into();
                self.as_ratio().$call(rhs.as_ratio()).into()
            }
        }

        impl<T: Copy + Integer> core::ops::$assign_op<T> for Ratio<T> {
            fn $assign(&mut self, rhs: T) {
                *self = core::ops::$op::$call(*self, rhs);
            }
        }

        impl<T: Copy + Integer> core::ops::$assign_op<Ratio<T>> for Ratio<T> {
            fn $assign(&mut self, rhs: Ratio<T>) {
                *self = core::ops::$op::$call(*self, rhs);
            }
        }

        impl<T: Copy + Integer> core::ops::$assign_op<(T, T)> for Ratio<T> {
            fn $assign(&mut self, rhs: (T, T)) {
                *self = core::ops::$op::$call(*self, rhs);
            }
        }
    };
}

inner_ratio_arithmetic!(Add, add, AddAssign, add_assign);
inner_ratio_arithmetic!(Sub, sub, SubAssign, sub_assign);
inner_ratio_arithmetic!(Mul, mul, MulAssign, mul_assign);
inner_ratio_arithmetic!(Div, div, DivAssign, div_assign);
inner_ratio_arithmetic!(Rem, rem, RemAssign, rem_assign);

impl<T: Copy + Integer> From<Ratio<T>> for Inner<T> {
    fn from(ratio: Ratio<T>) -> Self {
        ratio.as_ratio()
    }
}

impl<T> From<Ratio<T>> for (T, T) {
    fn from(ratio: Ratio<T>) -> Self {
        (ratio.0, ratio.1)
    }
}

impl<T: Clone + Integer> From<Inner<T>> for Ratio<T> {
    fn from(inner: Inner<T>) -> Self {
        let (n, d) = inner.into();
        Self(n, d)
    }
}

impl<T: One> From<T> for Ratio<T> {
    fn from(inner: T) -> Self {
        Self(inner, T::one())
    }
}

impl<T: Clone + Integer> From<(T, T)> for Ratio<T> {
    fn from((n, d): (T, T)) -> Self {
        Self(n, d)
    }
}

impl<T: Copy + Integer> Neg for Ratio<T>
where
    Inner<T>: Neg<Output = Inner<T>>,
{
    type Output = Self;

    fn neg(self) -> Self {
        self.as_ratio().neg().into()
    }
}
