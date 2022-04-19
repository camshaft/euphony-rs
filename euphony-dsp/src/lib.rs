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

pub mod loader;
mod osc;

pub mod sample {
    pub use dasp_sample::*;

    pub type Default = f64;
    pub type DefaultRate = Rate48000;

    pub trait Rate: 'static + Send + Sync {
        const PERIOD: f64;
        const VALUE: f64;
    }

    pub struct Rate44100;

    impl Rate for Rate44100 {
        const PERIOD: f64 = 1.0f64 / 44100.0;
        const VALUE: f64 = 44100.0;
    }

    pub struct Rate48000;

    impl Rate for Rate48000 {
        const PERIOD: f64 = 1.0f64 / 48000.0;
        const VALUE: f64 = 48000.0;
    }
}

#[test]
fn reflection() {
    euphony_node::reflect::generate_files(env!("CARGO_MANIFEST_DIR"));
}
