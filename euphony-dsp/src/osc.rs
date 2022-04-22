use crate::sample::{DefaultRate as Rate, Rate as _};
use core::f64::consts::TAU;
use euphony_node::{Input, Node};
use fastapprox::{fast, faster};

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
            unsafe_assert!(!value.is_nan());
            unsafe_assert!(value.is_finite());
            unsafe_assert!((0.0..=1.0).contains(&value));
        }
        self.0 = Rate::PERIOD.mul_add(freq, value).fract();
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
        #[derive(Default, Node)]
        #[node(id = $id, module = "osc")]
        #[input(frequency, default = 440.0)]
        #[input(phase, trigger = set_phase)]
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
                        for (frame, freq) in output.iter_mut().zip(freq.iter()) {
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
    |phase| ((0.5 - phase).abs() - 0.25) * 4.0
);

#[derive(Default, Node)]
#[node(id = 106, module = "osc")]
pub struct Silence;

impl Silence {
    #[inline]
    pub fn render(&mut self, _output: &mut [f64]) {}
}
