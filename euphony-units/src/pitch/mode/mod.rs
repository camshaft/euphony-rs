use crate::pitch::{
    interval::Interval,
    mode::{intervals::RoundingStrategy, position::ModePosition, system::ModeSystem},
};
use core::fmt;

#[macro_use]
pub mod intervals;
#[macro_use]
pub mod system;
pub mod position;

#[macro_export]
macro_rules! named_mode {
    ($name:ident($position:expr, $system:ident)) => {
        pub const $name: $crate::pitch::mode::Mode =
            $crate::pitch::mode::Mode::new($position, $system);
    };
}

pub mod chromatic;
pub use chromatic as dodecatonic;
pub mod ditonic;
pub mod heptatonic;
pub mod pentatonic;

pub mod western {
    pub use super::{chromatic::*, heptatonic::*};
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mode {
    pub ascending: ModePosition,
    pub descending: ModePosition,
}

impl Default for Mode {
    fn default() -> Self {
        heptatonic::MAJOR
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ascending == self.descending {
            return write!(f, "Mode({})", self.ascending);
        }
        write!(f, "Mode(TODO)")
    }
}

impl Mode {
    pub const fn new(position: usize, system: ModeSystem) -> Self {
        Self {
            ascending: ModePosition(position, system),
            descending: ModePosition(position, system),
        }
    }

    pub const fn len(&self) -> usize {
        self.ascending.intervals().len()
    }

    pub const fn is_empty(&self) -> bool {
        self.ascending.intervals().is_empty()
    }

    pub fn collapse(&self, interval: Interval, rounding_strategy: RoundingStrategy) -> Interval {
        self.checked_collapse(interval, rounding_strategy)
            .expect("Interval could not be collapsed")
    }

    pub fn checked_collapse(
        &self,
        interval: Interval,
        rounding_strategy: RoundingStrategy,
    ) -> Option<Interval> {
        if interval < 0 {
            self.descending
                .checked_collapse(interval, rounding_strategy)
        } else {
            self.ascending.checked_collapse(interval, rounding_strategy)
        }
    }

    pub fn expand(&self, interval: Interval, rounding_strategy: RoundingStrategy) -> Interval {
        self.checked_expand(interval, rounding_strategy)
            .expect("Interval could not be expanded")
    }

    pub fn checked_expand(
        &self,
        interval: Interval,
        rounding_strategy: RoundingStrategy,
    ) -> Option<Interval> {
        if interval < 0 {
            self.descending.checked_expand(interval, rounding_strategy)
        } else {
            self.ascending.checked_expand(interval, rounding_strategy)
        }
    }
}

impl core::ops::Shr<usize> for Mode {
    type Output = Mode;

    fn shr(self, rhs: usize) -> Self::Output {
        Self {
            ascending: self.ascending.shr(rhs),
            descending: self.descending.shr(rhs),
        }
    }
}

impl core::ops::ShrAssign<usize> for Mode {
    fn shr_assign(&mut self, rhs: usize) {
        *self = core::ops::Shr::shr(*self, rhs)
    }
}

impl core::ops::Shl<usize> for Mode {
    type Output = Mode;

    fn shl(self, rhs: usize) -> Self::Output {
        Self {
            ascending: self.ascending.shl(rhs),
            descending: self.descending.shl(rhs),
        }
    }
}

impl core::ops::ShlAssign<usize> for Mode {
    fn shl_assign(&mut self, rhs: usize) {
        self.ascending <<= rhs;
        self.descending <<= rhs;
    }
}

impl core::ops::Mul<Interval> for Mode {
    type Output = Interval;

    fn mul(self, interval: Interval) -> Self::Output {
        self.expand(interval, Default::default())
    }
}

impl core::ops::Div<Mode> for Interval {
    type Output = Interval;

    fn div(self, mode: Mode) -> Self::Output {
        mode.collapse(self, Default::default())
    }
}

#[test]
fn shift_test() {
    use crate::pitch::mode::heptatonic::*;
    assert_eq!(MAJOR >> 1, DORIAN);
    assert_eq!(MAJOR << 2, MINOR);
    assert_eq!(MAJOR << 7, MAJOR);
}

#[test]
fn expansion_test() {
    use crate::pitch::mode::western::*;

    assert_eq!(A + MINOR * I, A);
    assert_eq!(A + MINOR * II, B);
    assert_eq!(A + MINOR * III, C);
    assert_eq!(A + MINOR * IV, D);
    assert_eq!(A + MINOR * V, E);
    assert_eq!(A + MINOR * VI, F);
    assert_eq!(A + MINOR * VII, G);

    assert_eq!(C + MAJOR * I, C);
    assert_eq!(C + MAJOR * II, D);
    assert_eq!(C + MAJOR * III, E);
    assert_eq!(C + MAJOR * IV, F);
    assert_eq!(C + MAJOR * V, G);
    assert_eq!(C + MAJOR * VI, A + 1);
    assert_eq!(C + MAJOR * VII, B + 1);

    assert_eq!(C + MINOR * I, C);
    assert_eq!(C + MINOR * II, D);
    assert_eq!(C + MINOR * III, E.flat());
    assert_eq!(C + MINOR * IV, F);
    assert_eq!(C + MINOR * V, G);
    assert_eq!(C + MINOR * VI, A.flat() + 1);
    assert_eq!(C + MINOR * VII, B.flat() + 1);

    assert_eq!(A + MINOR * -I, A);
    assert_eq!(A + MINOR * -II, G - 1);
    assert_eq!(A + MINOR * -III, F - 1);
    assert_eq!(A + MINOR * -IV, E - 1);
    assert_eq!(A + MINOR * -V, D - 1);
    assert_eq!(A + MINOR * -VI, C - 1);
    assert_eq!(A + MINOR * -VII, B - 1);

    assert_eq!(C + MAJOR * -I, C);
    assert_eq!(C + MAJOR * -II, B);
    assert_eq!(C + MAJOR * -III, A);
    assert_eq!(C + MAJOR * -IV, G - 1);
    assert_eq!(C + MAJOR * -V, F - 1);
    assert_eq!(C + MAJOR * -VI, E - 1);
    assert_eq!(C + MAJOR * -VII, D - 1);
}

#[test]
fn collapse_test() {
    use crate::pitch::mode::western::*;

    assert_eq!(A / MINOR, I);
    assert_eq!(A.sharp() / MINOR, I);
    assert_eq!(B / MINOR, II);
    assert_eq!(C / MINOR, III);
    assert_eq!(C.sharp() / MINOR, III);
    assert_eq!(D / MINOR, IV);
    assert_eq!(D.sharp() / MINOR, IV);
    assert_eq!(E / MINOR, V);
    assert_eq!(F / MINOR, VI);
    assert_eq!(F.sharp() / MINOR, VI);
    assert_eq!(G / MINOR, VII);
    assert_eq!(G.sharp() / MINOR, VII);
    assert_eq!((A + 1) / MINOR, Interval(7, 7));
    assert_eq!(A.flat() / MINOR, -Interval(1, 7));
    assert_eq!(A.flat().flat() / MINOR, -Interval(1, 7));
    assert_eq!(A.flat().flat().flat() / MINOR, -Interval(2, 7));
}
