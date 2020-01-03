use crate::time::duration::Duration;
use core::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Timestamp(Duration);

impl Timestamp {
    #[allow(dead_code)]
    pub(crate) const fn from_duration(duration: Duration) -> Self {
        Self(duration)
    }

    pub const fn as_micros(&self) -> u64 {
        self.0.as_micros() as u64
    }
}

impl Add<Duration> for Timestamp {
    type Output = Timestamp;

    fn add(self, duration: Duration) -> Self::Output {
        Self(self.0 + duration)
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, duration: Duration) {
        self.0 += duration;
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = Duration;

    fn sub(self, duration: Timestamp) -> Self::Output {
        self.0 - duration.0
    }
}
