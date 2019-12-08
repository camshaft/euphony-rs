new_ratio!(Interval, i64);

#[macro_export]
macro_rules! named_interval {
    ($name:ident($n:expr, $d:expr)) => {
        pub const $name: $crate::pitch::interval::Interval =
            $crate::pitch::interval::Interval($n, $d);
    };
}

named_interval!(UNISON(0, 1));
named_interval!(OCTAVE(1, 1));
named_interval!(DOUBLE_OCTAVE(2, 1));

impl core::ops::Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        self.as_ratio().neg().into()
    }
}

new_ratio_struct!(AbsoluteInterval, i64);

impl core::ops::Add<Interval> for AbsoluteInterval {
    type Output = AbsoluteInterval;

    fn add(self, rhs: Interval) -> Self {
        self.as_ratio().add(rhs.as_ratio()).into()
    }
}

impl core::ops::AddAssign<Interval> for AbsoluteInterval {
    fn add_assign(&mut self, rhs: Interval) {
        *self = core::ops::Add::add(*self, rhs);
    }
}

impl core::ops::Sub for AbsoluteInterval {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Interval {
        self.as_ratio().sub(rhs.as_ratio()).into()
    }
}

impl core::ops::Sub<Interval> for AbsoluteInterval {
    type Output = AbsoluteInterval;

    fn sub(self, rhs: Interval) -> AbsoluteInterval {
        self.as_ratio().sub(rhs.as_ratio()).into()
    }
}

impl core::ops::SubAssign<Interval> for AbsoluteInterval {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = core::ops::Sub::sub(*self, rhs);
    }
}

impl core::ops::Div for AbsoluteInterval {
    type Output = Interval;

    fn div(self, rhs: Self) -> Self::Output {
        self.as_ratio().div(rhs.as_ratio()).into()
    }
}

new_ratio_conversions!(AbsoluteInterval, i64);
