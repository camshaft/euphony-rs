use core::{cmp::Ordering, fmt, ops::Neg};
use num_integer::Integer;
use num_rational::Ratio as Inner;
use num_traits::{One, Zero};

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
        write!(f, "{}", self)
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
        self.as_ratio().partial_cmp(&other.as_ratio())
    }
}

impl<T: Copy + Integer> Eq for Ratio<T> {}

impl<T: Copy + Integer> Ord for Ratio<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ratio().cmp(&other.as_ratio())
    }
}

impl<T: Copy + Integer> Ratio<T> {
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

impl<T: Copy + Integer> Into<Inner<T>> for Ratio<T> {
    fn into(self) -> Inner<T> {
        self.as_ratio()
    }
}

impl<T: Clone + Integer> Into<(T, T)> for Ratio<T> {
    fn into(self) -> (T, T) {
        (self.0, self.1)
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
