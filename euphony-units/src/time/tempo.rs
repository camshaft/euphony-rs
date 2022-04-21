use crate::time::{beat::Beat, duration::Duration, unquantized_beat::UnquantizedBeat};
use core::ops::{Div, Mul};

new_ratio!(Tempo, u64);

const MINUTE: Duration = Duration::from_secs(60);

impl Tempo {
    pub const DEFAULT: Self = Self(120, 1);

    pub fn as_beat_duration(self) -> Duration {
        MINUTE / self.as_ratio()
    }

    pub fn from_beat_duration(duration: Duration) -> Self {
        Tempo(MINUTE.as_nanos() as u64, duration.as_nanos() as u64)
            .reduce()
            .simplify(64)
    }
}

impl Mul<Beat> for Tempo {
    type Output = Duration;

    fn mul(self, beat: Beat) -> Self::Output {
        self.as_beat_duration() * beat.as_ratio()
    }
}

impl Div<Tempo> for Duration {
    type Output = UnquantizedBeat;

    fn div(self, tempo: Tempo) -> Self::Output {
        let a = self.as_nanos();
        let b = tempo.as_beat_duration().as_nanos();

        let whole = (a / b) as u64;
        let fract = (a % b) as u64;

        UnquantizedBeat(whole, Beat(fract, b as u64).reduce())
    }
}

#[test]
fn beat_round_trip_test() {
    let tempos = 60..320;
    let beats = [
        Beat(1, 64),
        Beat(1, 48),
        Beat(1, 32),
        Beat(1, 24),
        Beat(1, 16),
        Beat(1, 12),
        Beat(1, 8),
        Beat(1, 6),
        Beat(1, 4),
        Beat(1, 3),
        Beat(1, 2),
        Beat(1, 1),
    ];
    let counts = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 1000, 10_000, 100_000, 1_000_000,
    ];
    for tempo in tempos {
        let tempo = Tempo(tempo, 1);
        assert_eq!(tempo, Tempo::from_beat_duration(tempo.as_beat_duration()));

        for count in counts.iter() {
            for beat in beats.iter() {
                let expected = *beat * *count;
                let actual: UnquantizedBeat = (tempo * expected) / tempo;
                assert_eq!(
                    actual.quantize(Beat(1, 192)),
                    expected,
                    "{:?} {:?}",
                    tempo,
                    beat
                );
            }
        }
    }
}
