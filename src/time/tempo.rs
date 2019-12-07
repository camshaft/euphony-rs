use crate::{ratio::div_duration, time::beat::Beat};
use core::{ops::Mul, time::Duration};

new_ratio!(Tempo, u64);

impl Default for Tempo {
    fn default() -> Self {
        Self::new(120)
    }
}

const MINUTE: Duration = Duration::from_secs(60);

impl Tempo {
    pub(crate) fn as_duration(self) -> Duration {
        div_duration(MINUTE, self.as_ratio())
    }
}

impl Mul<Beat> for Tempo {
    type Output = Duration;

    fn mul(self, beat: Beat) -> Self::Output {
        div_duration(MINUTE, self.as_ratio() * beat.as_ratio())
    }
}
