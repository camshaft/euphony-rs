use core::ops::Mul;

new_ratio!(BaseFrequency, u64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Frequency(pub f64);

impl From<BaseFrequency> for Frequency {
    #[inline]
    fn from(freq: BaseFrequency) -> Self {
        Self(freq.as_f64())
    }
}

impl Mul<BaseFrequency> for FrequencyRatio {
    type Output = Frequency;

    #[inline]
    fn mul(self, rhs: BaseFrequency) -> Self::Output {
        Frequency((self * rhs.as_ratio()).as_f64())
    }
}

impl Mul<FrequencyRatio> for BaseFrequency {
    type Output = Frequency;

    #[inline]
    fn mul(self, rhs: FrequencyRatio) -> Self::Output {
        Frequency((self * rhs.as_ratio()).as_f64())
    }
}

impl Mul<f64> for BaseFrequency {
    type Output = Frequency;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Frequency(self.as_f64() * rhs)
    }
}

impl Mul<BaseFrequency> for f64 {
    type Output = Frequency;

    #[inline]
    fn mul(self, rhs: BaseFrequency) -> Self::Output {
        Frequency(rhs.as_f64() * self)
    }
}

new_ratio!(FrequencyRatio, u64);
