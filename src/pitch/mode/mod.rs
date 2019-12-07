use crate::pitch::{
    interval::Interval,
    mode::{intervals::ModeIntervals, system::ModeSystem},
};
use core::fmt;

#[macro_use]
pub mod intervals;
#[macro_use]
pub mod system;

pub mod heptatonic;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mode(pub usize, pub ModeSystem);

impl Default for Mode {
    fn default() -> Self {
        heptatonic::MAJOR
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mode({:?})", self.intervals)
    }
}

impl Mode {
    pub const fn system(&self) -> ModeSystem {
        self.1
    }

    const fn index(&self) -> usize {
        self.0
    }
}

impl core::ops::Deref for Mode {
    type Target = ModeIntervals;

    fn deref(&self) -> &Self::Target {
        &self.1[self.0]
    }
}

impl core::ops::Shr<usize> for Mode {
    type Output = Mode;

    fn shr(self, rhs: usize) -> Self::Output {
        let rhs = rhs % self.system().len();
        let value = self.index() + rhs;
        let value = value % self.system().len();
        Self(value, self.system())
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
        let rhs = rhs % self.system().len();
        let value = self.system().len() + self.index() - rhs;
        let value = value % self.system().len();
        Self(value, self.system())
    }
}

impl core::ops::ShlAssign<usize> for Mode {
    fn shl_assign(&mut self, rhs: usize) {
        *self = core::ops::Shl::shl(*self, rhs)
    }
}

impl core::ops::Mul<Interval> for Mode {
    type Output = Interval;

    fn mul(self, interval: Interval) -> Self::Output {
        self.apply(interval, Default::default())
    }
}
