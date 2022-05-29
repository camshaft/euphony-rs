use crate::pitch::{
    frequency::{BaseFrequency, Frequency, FrequencyRatio},
    interval::Interval,
};
use core::ops::Mul;
use euphony_macros::cents;

pub mod western {
    pub static ET12: super::Tuning = super::Tuning {
        base: super::BaseFrequency(440, 1),
        system: &super::ET12,
    };
}

#[derive(Clone, Copy)]
pub struct Tuning {
    pub base: BaseFrequency,
    pub system: &'static dyn System,
}

impl Mul<Tuning> for Interval {
    type Output = Frequency;

    fn mul(self, rhs: Tuning) -> Self::Output {
        rhs.system.to_frequency(rhs.base, self)
    }
}

impl Mul<Interval> for Tuning {
    type Output = Frequency;

    fn mul(self, rhs: Interval) -> Self::Output {
        self.system.to_frequency(self.base, rhs)
    }
}

pub trait System: 'static + Send + Sync {
    fn to_frequency(&self, base: BaseFrequency, interval: Interval) -> Frequency;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Octave<T>(&'static [T])
where
    T: 'static + Copy + core::ops::Mul<BaseFrequency, Output = Frequency>,
    &'static [T]: Send + Sync;

impl<T> System for Octave<T>
where
    T: 'static + Copy + core::ops::Mul<BaseFrequency, Output = Frequency>,
    &'static [T]: Send + Sync,
{
    fn to_frequency(&self, base: BaseFrequency, interval: Interval) -> Frequency {
        let mut octaves = interval.whole();
        let mut fraction = interval.fraction();

        if fraction.0 < 0 {
            fraction += Interval(1, 1);
            octaves -= 1;
        }

        let base = if octaves < 0 {
            base / 2u64.pow(octaves.unsigned_abs() as u32)
        } else {
            base * 2u64.pow(octaves as u32)
        };

        if fraction.0 == 0 {
            return base.into();
        }

        let index = (fraction * (self.0.len() + 1)).whole();
        assert!(
            (1..=self.0.len() as i64).contains(&index),
            "interval is out of range for scale: {:?}",
            interval,
        );
        let mul = self.0[index as usize - 1];
        mul * base
    }
}

macro_rules! et {
    ($($cent:literal),* $(,)?) => {
        Octave(&[$(cents!($cent)),*])
    };
}

pub static ET12: Octave<f64> = et!(100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100);

macro_rules! just {
    ($($n:literal / $d:literal),* $(,)?) => {
        Octave(&[$(FrequencyRatio($n, $d)),*])
    };
}

pub static JUST12: Octave<FrequencyRatio> = just!(
    16 / 15,
    9 / 8,
    6 / 5,
    5 / 4,
    4 / 3,
    64 / 45,
    3 / 2,
    8 / 5,
    5 / 3,
    16 / 9,
    15 / 8
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn et12_test() {
        let tuning = Tuning {
            base: 440.into(),
            system: &ET12,
        };

        let mut results = vec![];

        eprintln!("interval,frequency,idx");
        for octave in [-3, -2, -1, 0, 1, 2, 3] {
            for interval in 0..12 {
                let interval = Interval(interval, 12) + octave;
                let freq = tuning * interval;
                eprintln!("{:?},{},{}", interval, freq.0, results.len());
                results.push(freq);
            }
        }
    }

    #[test]
    fn just12_test() {
        let tuning = Tuning {
            base: 440.into(),
            system: &JUST12,
        };

        let mut results = vec![];

        eprintln!("interval,frequency,idx");
        for octave in [-3, -2, -1, 0, 1, 2, 3] {
            for interval in 0..12 {
                let interval = Interval(interval, 12) + octave;
                let freq = tuning * interval;
                eprintln!("{:?},{},{}", interval, freq.0, results.len());
                results.push(freq);
            }
        }
    }
}
