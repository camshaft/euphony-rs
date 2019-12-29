use crate::time::{beat::Beat, duration::Duration};
use core::ops::Mul;

new_ratio!(Tempo, u64);

const MINUTE: Duration = Duration::from_secs(60);

impl Tempo {
    pub(crate) fn as_duration(self) -> Duration {
        MINUTE / self.as_ratio()
    }
}

impl Mul<Beat> for Tempo {
    type Output = Duration;

    fn mul(self, beat: Beat) -> Self::Output {
        self.as_duration() * beat.as_ratio()
    }
}
