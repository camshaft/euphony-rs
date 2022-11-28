use crate::{
    ratio::Ratio,
    time::{measure::Measure, time_signature::TimeSignature},
};
use core::ops::{Div, Mul, Rem};

new_ratio!(Beat, u64);
new_ratio!(Instant, u64);

new_extern_ratio_arithmetic!(Instant, Add, add, AddAssign, add_assign, Beat);
new_extern_ratio_arithmetic!(Instant, Sub, sub, SubAssign, sub_assign, Beat);

impl Div<Beat> for Instant {
    type Output = Ratio<u64>;

    fn div(self, rhs: Beat) -> Self::Output {
        self.as_ratio().div(rhs.as_ratio())
    }
}

impl Rem<Beat> for Instant {
    type Output = Ratio<u64>;

    fn rem(self, rhs: Beat) -> Self::Output {
        self.as_ratio().rem(rhs.as_ratio())
    }
}

impl Beat {
    pub const DEFAULT_RESOLUTION: Self = Beat(1, Self::DEFAULT_TICKS_PER_BEAT);
    pub const DEFAULT_TICKS_PER_BEAT: u64 = 4096;

    pub const EIGHTH: Beat = Beat(1, 8);
    pub const EIGHTH_TRIPLET: Beat = Beat(1, 6);
    pub const HALF: Beat = Beat(1, 2);
    pub const QUARTER: Beat = Beat(1, 4);
    pub const QUARTER_TRIPLET: Beat = Beat(1, 3);
    pub const SIXTEENTH: Beat = Beat(1, 16);
    pub const SIXTY_FOURTH: Beat = Beat(1, 64);
    pub const THIRTY_SECOND: Beat = Beat(1, 32);
    pub const WHOLE: Beat = Beat(1, 1);

    pub fn vec(denominators: impl IntoIterator<Item = u64>) -> alloc::vec::Vec<Self> {
        denominators.into_iter().map(|d| Beat(1, d)).collect()
    }
}

impl Mul<TimeSignature> for Beat {
    type Output = Beat;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, time_signature: TimeSignature) -> Self::Output {
        (self / time_signature.beat()).into()
    }
}

impl core::ops::Div<TimeSignature> for Beat {
    type Output = Measure;

    fn div(self, time_signature: TimeSignature) -> Self::Output {
        let beat_count = self / time_signature.beat();
        (beat_count / time_signature.count()).into()
    }
}

impl Instant {
    pub fn arc_after(self, duration: Beat) -> Arc {
        let start = self;
        let end = self + duration;
        Arc { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Arc {
    pub start: Instant,
    pub end: Instant,
}

impl Arc {
    pub fn contains(&self, instant: Instant) -> bool {
        self.start <= instant && instant < self.end
    }
}

#[test]
fn div_time_signature_test() {
    assert_eq!(Beat(1, 4) / TimeSignature(4, 4), Measure(1, 4));
    assert_eq!(Beat(2, 4) / TimeSignature(4, 4), Measure(1, 4) * 2);
    assert_eq!(Beat(1, 4) / TimeSignature(6, 8), Measure(1, 3));
    assert_eq!(Beat(5, 4) / TimeSignature(4, 4), Measure(5, 4));
}
