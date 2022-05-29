// #![cfg_attr(not(any(feature = "std", test)), no_std)]

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

mod fun;
mod prelude;

pub mod nodes;
pub mod sample;

mod binary;
mod buffer;
mod env;
mod filter;
mod osc;
mod tertiary;
mod unary;

#[test]
fn reflection() {
    euphony_node::reflect::generate_files(env!("CARGO_MANIFEST_DIR"));
}
