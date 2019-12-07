use crate::pitch::interval::Interval;
use core::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModeIntervals {
    pub size: usize,
    pub steps: &'static [Interval],
    pub intervals: &'static [Interval],
}

#[macro_export]
macro_rules! mode_intervals {
    ([$($step:expr),* $(,)?]) => {
        $crate::mode_intervals!([$($step),*], 0 $(+ $step)*)
    };
    ([$($step:expr),* $(,)?], $size:expr) => {
        $crate::pitch::mode::intervals::ModeIntervals {
            size: $size,
            steps: &[$(
                $crate::pitch::interval::Interval($step, $size)),*
            ],
            intervals: $crate::mode_intervals!(
                @intervals,
                [$($step,)*],
                0,
                $size,
                [$crate::pitch::interval::Interval(0, $size),]
            )
        }
    };
    (@intervals, [$step:expr, ], $acc:expr, $size:expr, [$($intervals:expr,)*]) => {
        &[$($intervals),*]
    };
    (@intervals, [$step:expr, $($steps:expr,)*], $acc:expr, $size:expr, [$($intervals:expr,)*]) => {
        $crate::mode_intervals!(
            @intervals,
            [$($steps,)*],
            $step + $acc,
            $size,
            [
                $($intervals,)*
                $crate::pitch::interval::Interval($step + $acc, $size),
            ]
        )
    };
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RoundingStrategy {
    Down,
    Up,
    NearestDown,
    NearestUp,
    TowardsZero,
    AwayFromZero,
    Reject,
}

impl Default for RoundingStrategy {
    fn default() -> Self {
        RoundingStrategy::NearestDown
    }
}

impl ModeIntervals {
    pub fn apply(&self, interval: Interval, rounding_strategy: RoundingStrategy) -> Interval {
        self.try_apply(interval, rounding_strategy)
            .expect("Interval could not be applied")
    }

    pub fn try_apply(
        &self,
        interval: Interval,
        rounding_strategy: RoundingStrategy,
    ) -> Option<Interval> {
        use RoundingStrategy::*;

        let scaled = (interval * self.intervals.len()).as_ratio();
        let scaled = match rounding_strategy {
            _ if scaled.is_integer() => scaled.to_integer(),
            Down => scaled.floor().to_integer(),
            Up => scaled.ceil().to_integer(),
            TowardsZero => scaled.trunc().to_integer(),
            AwayFromZero => scaled.round().to_integer(),
            Reject => return None,
            NearestDown | NearestUp => {
                let lower = self.get(scaled.floor().to_integer());
                let upper = self.get(scaled.ceil().to_integer());
                return match lower.cmp(&upper) {
                    Ordering::Equal if rounding_strategy == NearestDown => Some(lower),
                    Ordering::Equal => Some(upper),
                    Ordering::Greater => Some(upper),
                    Ordering::Less => Some(lower),
                };
            }
        };

        Some(self.get(scaled))
    }

    fn get(&self, scaled: i64) -> Interval {
        let len = self.intervals.len();

        // TODO support multi-octave modes

        if scaled < 0 {
            let index = (len - (scaled.abs() as usize % len)) % len;
            let octave = (scaled.abs() - 1) as usize / len;
            let value = -(Interval(1, 1) - self.intervals[index]);
            value - Interval(1, 1) * octave
        } else {
            let index = scaled as usize % len;
            let octave = scaled as usize / len;
            self.intervals[index] + Interval(1, 1) * octave
        }
    }
}

impl core::ops::Mul<Interval> for ModeIntervals {
    type Output = Interval;

    fn mul(self, interval: Interval) -> Self::Output {
        self.apply(interval, Default::default())
    }
}

#[test]
fn interval_mode_bounds_test() {
    use super::heptatonic::MAJOR;

    for i in -10000..10000 {
        let _ = MAJOR.get(i);
    }
}

#[test]
fn interval_mode_test() {
    use super::heptatonic::MAJOR;

    assert_eq!(MAJOR * Interval(0, 7), Interval(0, 12));
    assert_eq!(MAJOR * Interval(1, 7), Interval(2, 12));
    assert_eq!(MAJOR * Interval(2, 7), Interval(4, 12));
    assert_eq!(MAJOR * Interval(3, 7), Interval(5, 12));
    assert_eq!(MAJOR * Interval(4, 7), Interval(7, 12));
    assert_eq!(MAJOR * Interval(5, 7), Interval(9, 12));
    assert_eq!(MAJOR * Interval(6, 7), Interval(11, 12));
    assert_eq!(MAJOR * Interval(7, 7), Interval(12, 12));
    assert_eq!(MAJOR * Interval(8, 7), Interval(14, 12));
    assert_eq!(MAJOR * Interval(9, 7), Interval(16, 12));
    assert_eq!(MAJOR * Interval(14, 7), Interval(24, 12));

    assert_eq!(MAJOR * Interval(-1, 7), Interval(-1, 12));
    assert_eq!(MAJOR * Interval(-2, 7), Interval(-3, 12));
    assert_eq!(MAJOR * Interval(-7, 7), Interval(-12, 12));
    assert_eq!(MAJOR * Interval(-8, 7), Interval(-13, 12));
    assert_eq!(MAJOR * Interval(-9, 7), Interval(-15, 12));
    assert_eq!(MAJOR * Interval(-14, 7), Interval(-24, 12));
}
