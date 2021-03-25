use crate::time::{
    beat::Beat, duration::Duration, measure::Measure, tempo::Tempo, time_signature::TimeSignature,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TimeContext {
    pub time_signature: TimeSignature,
    pub tempo: Tempo,
}

impl TimeContext {
    pub fn new<B: Into<Tempo>>(tempo: B) -> Self {
        Self::default().with_tempo(tempo)
    }

    pub fn with_time_signature<V: Into<TimeSignature>>(self, time_signature: V) -> Self {
        Self {
            time_signature: time_signature.into(),
            tempo: self.tempo,
        }
    }

    pub fn with_tempo<B: Into<Tempo>>(self, tempo: B) -> Self {
        Self {
            time_signature: self.time_signature,
            tempo: tempo.into(),
        }
    }
}

impl core::ops::Mul<Beat> for TimeContext {
    type Output = Duration;

    fn mul(self, beats: Beat) -> Self::Output {
        let beat_count = beats / self.time_signature.beat();
        self.tempo.as_beat_duration() * beat_count
    }
}

impl core::ops::Mul<Measure> for TimeContext {
    type Output = Duration;

    fn mul(self, measures: Measure) -> Self::Output {
        self.tempo.as_beat_duration() * measures.as_ratio()
    }
}

#[test]
fn mul_beat_test() {
    assert_eq!(
        TimeContext::new(120) * Beat(1, 4),
        Duration::from_millis(500)
    );
    assert_eq!(
        TimeContext::new(120) * Beat(1, 2),
        Duration::from_millis(1000)
    );
    assert_eq!(
        TimeContext::new(120).with_time_signature((2, 2)) * Beat(1, 2),
        Duration::from_millis(500)
    );

    assert_eq!(
        TimeContext::new(96) * Beat(1, 4),
        Duration::from_millis(625)
    );
    assert!(
        (TimeContext::new(95) * Beat(1, 4)) - Duration::from_millis(631) < Duration::from_millis(1)
    );
}
