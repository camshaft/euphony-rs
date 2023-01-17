use crate::prelude::*;
use core::f64::consts::TAU;
use fastapprox::{fast, faster};

pub mod nes;
pub mod noise;
pub mod wave;

#[derive(Debug, Clone, Copy, Default, Node)]
#[node(id = 107, module = "osc")]
#[input(frequency, default = 440.0)]
#[input(phase, trigger = set)]
pub struct Phase(f64);

impl Phase {
    #[inline]
    pub fn set(&mut self, phase: f64) {
        self.0 = phase.fract().abs();
    }

    #[inline(always)]
    fn next(&mut self, freq: f64) -> f64 {
        let value = self.0;
        unsafe {
            unsafe_assert!(!value.is_nan(), "value: {:?}", value);
            unsafe_assert!(value.is_finite(), "value: {:?}", value);
            unsafe_assert!((0.0..=1.0).contains(&value), "value: {:?}", value);
        }
        self.0 = Rate::PERIOD.mul_add(freq, value).fract().abs();
        value
    }

    #[inline]
    pub fn render(&mut self, frequency: Input, output: &mut [f64]) {
        match frequency {
            Input::Constant(freq) => {
                for frame in output.iter_mut() {
                    *frame = self.next(freq);
                }
            }
            Input::Buffer(freq) => {
                for (frame, freq) in output.iter_mut().zip(freq.iter()) {
                    *frame = self.next(*freq);
                }
            }
        }
    }
}

macro_rules! phase_osc {
    ($(#[doc = $doc:literal])* $id:literal, $name:ident, | $phase:ident | $sample:expr) => {
        phase_osc!($(#[doc = $doc])* $id, $name, 0.0, |$phase| $sample);
    };
    ($(#[doc = $doc:literal])* $id:literal, $name:ident, $default_phase:literal, | $phase:ident | $sample:expr) => {
        #[derive(Default, Node)]
        #[node(id = $id, module = "osc")]
        #[input(frequency, default = 440.0)]
        #[input(phase, default = $default_phase, trigger = set_phase)]
        $(
            #[doc = $doc]
        )*
        ///
        /// # frequency
        ///
        /// # phase (trigger)
        pub struct $name {
            phase: Phase,
        }

        impl $name {
            #[inline]
            pub fn set_phase(&mut self, phase: f64) {
                self.phase.set(phase);
            }

            #[inline]
            pub fn render(&mut self, frequency: Input, output: &mut [f64]) {
                // TODO split the output into chunks
                match frequency {
                    Input::Constant(freq) => {
                        for frame in output.iter_mut() {
                            let $phase = self.phase.next(freq);
                            *frame = $sample;
                        }
                    }
                    Input::Buffer(freq) => {
                        for (freq, frame) in (freq, output.iter_mut()).zip() {
                            let $phase = self.phase.next(*freq);
                            *frame = $sample;
                        }
                    }
                }
            }
        }
    };
}

phase_osc!(
    /// Accurate (slow) sine oscillator
    100,
    Sine,
    |phase| (TAU * phase).sin()
);
phase_osc!(
    /// Mostly accurate, but faster sine oscillator
    101,
    SineFast,
    |phase| fast::sinfull((TAU * phase) as f32) as f64
);
phase_osc!(
    /// Less accurate, but fast sine oscillator
    102,
    SineFaster,
    |phase| faster::sinfull((TAU * phase) as f32) as f64
);
phase_osc!(
    /// A pulse (square) oscillator
    103,
    Pulse,
    |phase| (0.5 - phase).signum()
);
phase_osc!(
    /// A sawtooth oscillator
    104,
    Sawtooth,
    |phase| (0.5 - phase) * 2.0
);
phase_osc!(
    /// A triangle oscillator
    105,
    Triangle,
    0.75,
    |phase| ((0.5 - phase).abs() - 0.25) * 4.0
);

#[derive(Default, Node)]
#[node(id = 106, module = "osc")]
pub struct Silence;

impl Silence {
    #[inline]
    pub fn render(&mut self, output: &mut [f64]) {
        output.fill(0.0);
    }
}

#[derive(Debug, Node)]
#[node(id = 116, module = "osc")]
#[input(frequency, default = 440.0)]
#[input(phase, default = 0.0, trigger = set_phase)]
/// Single sample impulse generator
pub struct Impulse {
    phase: Sample,
    value: Sample,
    trigger: bool,
}

impl Default for Impulse {
    fn default() -> Self {
        Self {
            phase: 0.0,
            value: 1.0,
            trigger: true, // pulse on the first sample
        }
    }
}

impl Impulse {
    #[inline]
    pub fn set_phase(&mut self, phase: f64) {
        let phase = phase.fract().abs();

        if phase < self.phase {
            self.trigger = true;
        }

        self.phase = phase;
    }

    #[inline(always)]
    fn next(&mut self, freq: f64) -> f64 {
        let phase = self.phase;
        unsafe {
            unsafe_assert!(!phase.is_nan(), "value: {:?}", phase);
            unsafe_assert!(phase.is_finite(), "value: {:?}", phase);
            unsafe_assert!((0.0..=1.0).contains(&phase), "value: {:?}", phase);
        }

        let value = if core::mem::replace(&mut self.trigger, false) {
            let value = self.value;
            self.value *= -1.0;
            value
        } else {
            0.0
        };

        let mut next = Rate::PERIOD.mul_add(freq, phase).abs();
        if next >= 1.0 {
            self.trigger = true;
            next = next.fract();
        }
        self.phase = next;

        value
    }

    #[inline]
    pub fn render(&mut self, frequency: Input, output: &mut [f64]) {
        match frequency {
            Input::Constant(freq) => {
                for frame in output.iter_mut() {
                    *frame = self.next(freq);
                }
            }
            Input::Buffer(freq) => {
                for (freq, frame) in (freq, output.iter_mut()).zip() {
                    *frame = self.next(*freq);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_test() {
        let mut osc = Triangle::new();
        let mut out = [0.0; 500];
        osc.render(480.0.into(), &mut out);
        eprintln!("{:?}", out);
    }

    #[test]
    fn impulse_test() {
        let mut osc = Impulse::new();
        let mut out = [0.0; 512];
        osc.render(440.0.into(), &mut out);
        eprintln!("{:?}", out);
    }
}
