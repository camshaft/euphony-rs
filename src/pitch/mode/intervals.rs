use crate::pitch::interval::Interval;
use core::{cmp::Ordering, fmt};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModeIntervals {
    pub tones: usize,
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
            .entries(self.steps.iter().map(|step| step.0))
            .finish()
    }
}

#[macro_export]
macro_rules! mode_intervals {
    ([$($step:expr),* $(,)?]) => {
        $crate::mode_intervals!([$($step),*], 0 $(+ $step)*)
    };
    ([$($step:expr),* $(,)?], $size:expr) => {
        $crate::pitch::mode::intervals::ModeIntervals {
            tones: $size,
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
    Pass,
}

impl Default for RoundingStrategy {
    fn default() -> Self {
        RoundingStrategy::NearestDown
    }
}

impl ModeIntervals {
    pub fn expand(&self, interval: Interval, rounding_strategy: RoundingStrategy) -> Interval {
        self.checked_expand(interval, rounding_strategy)
            .expect("Interval could not be applied")
    }

    pub fn checked_expand(
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
            Pass => return Some(interval),
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
        self.expand(interval, Default::default())
    }
}

#[test]
fn interval_mode_bounds_test() {
    use super::heptatonic::MAJOR;

    for i in -10000..10000 {
        let _ = MAJOR.expand(i.into(), Default::default());
    }
}
