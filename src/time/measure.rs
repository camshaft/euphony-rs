use crate::time::{beat::Beat, time_signature::TimeSignature};

new_ratio!(Measure, u64);

impl Default for Measure {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Measure {
    pub fn count(self) -> u64 {
        self.as_ratio().to_integer()
    }

    pub fn beat(self) -> Beat {
        self.as_ratio().fract().into()
    }
}

impl core::ops::Mul<TimeSignature> for Measure {
    type Output = Beat;

    fn mul(self, time_signature: TimeSignature) -> Self::Output {
        time_signature * self
    }
}
