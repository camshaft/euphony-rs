use crate::time::{measure::Measure, time_signature::TimeSignature};

new_ratio!(Beat, u64);

impl Beat {
    pub const EIGHTH: Beat = Beat(1, 8);
    // TODO
    pub const EIGHTH_TRIPLET: Beat = Beat(1, 3);
    pub const HALF: Beat = Beat(1, 2);
    pub const QUARTER: Beat = Beat(1, 4);
    // TODO
    pub const QUARTER_TRIPLET: Beat = Beat(1, 3);
    pub const SIXTEENTH: Beat = Beat(1, 16);
    pub const SIXTY_FOURTH: Beat = Beat(1, 64);
    pub const THIRTY_SECOND: Beat = Beat(1, 32);
    pub const WHOLE: Beat = Beat(1, 1);
}

impl Default for Beat {
    fn default() -> Self {
        Beat(1, 4)
    }
}

impl core::ops::Div<TimeSignature> for Beat {
    type Output = Measure;

    fn div(self, time_signature: TimeSignature) -> Self::Output {
        let beat_count = self / time_signature.beat();
        (beat_count / time_signature.count()).into()
    }
}

#[test]
fn div_time_signature_test() {
    assert_eq!(Beat(1, 4) / TimeSignature(4, 4), Measure(1, 4));
    assert_eq!(Beat(2, 4) / TimeSignature(4, 4), Measure(1, 4) * 2);
    assert_eq!(Beat(1, 4) / TimeSignature(6, 8), Measure(1, 3));
    assert_eq!(Beat(5, 4) / TimeSignature(4, 4), Measure(5, 4));
}
