use crate::pitch::mode::{intervals::ModeIntervals, system::ModeSystem};
use core::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModePosition(pub usize, pub ModeSystem);

impl ModePosition {
    pub const fn position(&self) -> usize {
        self.0
    }

    pub const fn system(&self) -> ModeSystem {
        self.1
    }

    pub fn intervals(&self) -> ModeIntervals {
        self.1[self.0]
    }
}

impl fmt::Debug for ModePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ModePosition({:?})", self.intervals())
    }
}

impl fmt::Display for ModePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.intervals().fmt(f)
    }
}

impl core::ops::Deref for ModePosition {
    type Target = ModeIntervals;

    fn deref(&self) -> &Self::Target {
        &self.1[self.0]
    }
}

impl core::ops::Shr<usize> for ModePosition {
    type Output = ModePosition;

    fn shr(self, rhs: usize) -> Self::Output {
        let rhs = rhs % self.system().len();
        let value = self.position() + rhs;
        let value = value % self.system().len();
        Self(value, self.system())
    }
}

impl core::ops::ShrAssign<usize> for ModePosition {
    fn shr_assign(&mut self, rhs: usize) {
        *self = core::ops::Shr::shr(*self, rhs)
    }
}

impl core::ops::Shl<usize> for ModePosition {
    type Output = ModePosition;

    fn shl(self, rhs: usize) -> Self::Output {
        let rhs = rhs % self.system().len();
        let value = self.system().len() + self.position() - rhs;
        let value = value % self.system().len();
        Self(value, self.system())
    }
}

impl core::ops::ShlAssign<usize> for ModePosition {
    fn shl_assign(&mut self, rhs: usize) {
        *self = core::ops::Shl::shl(*self, rhs)
    }
}
