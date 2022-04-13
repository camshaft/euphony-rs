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

    pub trait Rate {
        const PERIOD: f32;
        const VALUE: f32;
    }

    pub struct Rate44100;

    impl Rate for Rate44100 {
        const PERIOD: f32 = 1.0f32 / 44100.0;
        const VALUE: f32 = 44100.0;
    }

    pub struct Rate48000;

    impl Rate for Rate48000 {
        const PERIOD: f32 = 1.0f32 / 48000.0;
        const VALUE: f32 = 48000.0;
    }
}
pub mod buffer;
pub use buffer::Buffer;
pub mod signal;
pub use signal::Signal;
