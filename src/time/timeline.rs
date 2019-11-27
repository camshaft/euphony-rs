use super::timecode::Timecode;

pub struct TimelineEvent {
    target: Timecode,
    source: Timecode,
}
