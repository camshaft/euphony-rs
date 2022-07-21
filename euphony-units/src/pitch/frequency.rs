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

impl From<Frequency> for f64 {
    #[inline]
    fn from(freq: Frequency) -> Self {
        freq.0
    }
}

impl From<f64> for Frequency {
    #[inline]
    fn from(freq: f64) -> Self {
        Self(freq)
    }
}

impl From<u64> for Frequency {
    #[inline]
    fn from(freq: u64) -> Self {
        Self(freq as _)
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

impl Mul<f64> for Frequency {
    type Output = Frequency;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Frequency(self.0 * rhs)
    }
}

impl Mul<Frequency> for f64 {
    type Output = Frequency;

    #[inline]
    fn mul(self, rhs: Frequency) -> Self::Output {
        Frequency(self * rhs.0)
    }
}

impl Mul<Frequency> for Frequency {
    type Output = Frequency;

    #[inline]
    fn mul(self, rhs: Frequency) -> Self::Output {
        Frequency(self.0 * rhs.0)
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
