#![cfg_attr(not(any(feature = "std", test)), no_std)]

extern crate alloc;

#[macro_use]
mod ratio;

pub mod midi;
pub mod pitch;
pub mod time;

#[cfg(feature = "runtime")]
pub mod runtime;
