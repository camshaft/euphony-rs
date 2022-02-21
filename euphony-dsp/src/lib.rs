#![cfg_attr(not(any(feature = "std", test)), no_std)]

/// Asserts that a boolean expression is true at runtime, only if debug_assertions are enabled.
///
/// Otherwise, the compiler is told to assume that the expression is always true and can perform
/// additional optimizations.
macro_rules! unsafe_assert {
    ($cond:expr) => {
        unsafe_assert!($cond, "assumption failed: {}", stringify!($cond));
    };
    ($cond:expr $(, $fmtarg:expr)* $(,)?) => {{
        let v = $cond;

        debug_assert!(v $(, $fmtarg)*);
        if cfg!(not(debug_assertions)) && !v {
            core::hint::unreachable_unchecked();
        }
    }};
}

pub use dasp_frame as frame;
pub use frame::Frame;
pub mod sample {
    pub use dasp_sample::*;
    pub const PERIOD_44100: u32 = unsafe { core::mem::transmute(1.0f32 / 44100.0f32) };
    pub const PERIOD_48000: u32 = unsafe { core::mem::transmute(1.0f32 / 48000.0f32) };
}
pub mod buffer;
pub use buffer::Buffer;
pub mod signal;
pub use signal::Signal;

pub fn sine_fm_test(freq: f32, mul: f32, add: f32, phase: f32, buffer: &mut buffer::Buffer<f32>) {
    use crate::signal::{generator::*, Signal as _, SignalExt as _};

    let carrier = sine_fast::<f32>(freq)
        .phase(phase)
        .mul_signal(mul)
        .add_signal(add);
    let mut sine = sine_fast(carrier);
    sine.fill::<buffer::ArrayBatch<2, { sample::PERIOD_48000 }>>(buffer);
}
