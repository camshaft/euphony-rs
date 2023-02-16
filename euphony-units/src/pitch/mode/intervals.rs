use crate::pitch::interval::Interval;
use core::{cmp::Ordering, fmt};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModeIntervals {
    pub tones: &'static [Interval],
    pub steps: &'static [Interval],
    pub intervals: &'static [Interval],
}

impl fmt::Debug for ModeIntervals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ModeIntervals({:?})", self.steps)
    }
}

impl fmt::Display for ModeIntervals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(self.intervals.iter().map(|step| step.as_ratio()))
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RoundingStrategy {
    Down,
    Up,
    #[default]
    NearestDown,
    NearestUp,
    TowardsZero,
    AwayFromZero,
    Reject,
    Pass,
}

impl ModeIntervals {
    pub const fn len(&self) -> usize {
        self.intervals.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.intervals.is_empty()
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
        round_interval(self.tones, interval, rounding_strategy)
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
        round_interval(self.intervals, interval, rounding_strategy)
    }
}

fn round_interval(
    intervals: &[Interval],
    interval: Interval,
    rounding_strategy: RoundingStrategy,
) -> Option<Interval> {
    use RoundingStrategy::*;

    let scaled = (interval * intervals.len()).as_ratio();
    let scaled = match rounding_strategy {
        _ if scaled.is_whole() => scaled.whole(),
        Down => scaled.floor().whole(),
        Up => scaled.ceil().whole(),
        TowardsZero => scaled.truncate().whole(),
        AwayFromZero => scaled.round().whole(),
        Pass => return Some(interval),
        Reject => return None,
        NearestDown | NearestUp => {
            let lower = get_scaled_interval(intervals, scaled.floor().whole());
            let upper = get_scaled_interval(intervals, scaled.ceil().whole());
            return match lower.cmp(&upper) {
                Ordering::Equal if rounding_strategy == NearestDown => Some(lower),
                Ordering::Equal => Some(upper),
                Ordering::Greater => Some(upper),
                Ordering::Less => Some(lower),
            };
        }
    };

    Some(get_scaled_interval(intervals, scaled))
}

fn get_scaled_interval(intervals: &[Interval], scaled: i64) -> Interval {
    let len = intervals.len();

    if scaled < 0 {
        let index = (len - (scaled.unsigned_abs() as usize % len)) % len;
        let octave = (scaled.abs() - 1) as usize / len;
        let value = -(Interval(1, 1) - intervals[index]);
        value - Interval(1, 1) * octave
    } else {
        let index = scaled as usize % len;
        let octave = scaled as usize / len;
        intervals[index] + Interval(1, 1) * octave
    }
}

impl core::ops::Mul<Interval> for ModeIntervals {
    type Output = Interval;

    fn mul(self, interval: Interval) -> Self::Output {
        self.expand(interval, Default::default())
    }
}

impl core::ops::Div<ModeIntervals> for Interval {
    type Output = Interval;

    fn div(self, mode: ModeIntervals) -> Self::Output {
        mode.collapse(self, Default::default())
    }
}

#[test]
fn interval_mode_bounds_test() {
    use super::heptatonic::MAJOR;

    for i in -10000..10000 {
        let _ = MAJOR.expand(i.into(), Default::default());
    }
}
