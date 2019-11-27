use super::ratio::div_duration;
use core::time::Duration;

new_ratio!(Tempo);

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
