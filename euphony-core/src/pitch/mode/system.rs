use crate::pitch::mode::intervals::ModeIntervals;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModeSystem(pub &'static [ModeIntervals]);

impl ModeSystem {
    pub const fn new(intervals: &'static [ModeIntervals]) -> Self {
        Self(intervals)
    }
}

impl core::ops::Deref for ModeSystem {
    type Target = [ModeIntervals];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
