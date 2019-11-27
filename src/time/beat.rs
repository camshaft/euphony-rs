use super::{measure::Measure, time_signature::TimeSignature};

new_ratio!(Beat);

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
