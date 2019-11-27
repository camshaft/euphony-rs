use super::{beat::Beat, measure::Measure, time_context::TimeContext, timestamp::Timestamp};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Timecode {
    timestamp: Timestamp,
    context: TimeContext,
    measure: Measure,
    beat: Beat,
}
