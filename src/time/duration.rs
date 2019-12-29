use crate::ratio::Ratio;
use core::ops::{Div, Mul};
pub use core::time::Duration;

impl Mul<Ratio<u64>> for Duration {
    type Output = Duration;

    fn mul(self, ratio: Ratio<u64>) -> Self::Output {
        let whole_duration = self * ratio.whole() as u32;
        let (numer, denom) = ratio.fraction().into();
        let fract_duration = self / (denom as u32) * (numer as u32);
        whole_duration + fract_duration
    }
}

impl Div<Ratio<u64>> for Duration {
    type Output = Duration;

    fn div(self, ratio: Ratio<u64>) -> Self::Output {
        let (numer, denom) = ratio.into();
        if numer == 0 {
            return Duration::from_secs(0);
        }
        self / (numer as u32) * (denom as u32)
    }
}

#[test]
fn div_duration_test() {
    assert_eq!(
        Duration::from_secs(1) / Ratio(4, 3),
        Duration::from_millis(750)
    );
}
