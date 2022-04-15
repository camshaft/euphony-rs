use crate::time::{
    beat::Beat, measure::Measure, tempo::Tempo, time_signature::TimeSignature, timestamp::Timestamp,
};
use core::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Timecode {
    pub(crate) timestamp: Timestamp,
    pub(crate) tempo: Tempo,
    pub(crate) time_signature: TimeSignature,
    pub(crate) measure: Measure,
    pub(crate) beat: Beat,
}

impl Timecode {
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn tempo(&self) -> Tempo {
        self.tempo
    }

    pub fn time_signature(&self) -> TimeSignature {
        self.time_signature
    }

    pub fn measure(&self) -> Measure {
        self.measure
    }

    pub fn beat(&self) -> Beat {
        self.beat
    }
}

macro_rules! compare {
    ($ty:ident, $field:ident) => {
        impl PartialEq<$ty> for Timecode {
            fn eq(&self, other: &$ty) -> bool {
                self.$field.eq(other)
            }
        }

        impl PartialOrd<$ty> for Timecode {
            fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                self.$field.partial_cmp(other)
            }
        }
    };
}

compare!(Timestamp, timestamp);
compare!(Tempo, tempo);
compare!(TimeSignature, time_signature);
compare!(Measure, measure);
compare!(Beat, beat);
