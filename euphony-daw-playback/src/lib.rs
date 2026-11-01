//! Prepared playback through a device adapter.
//!
//! Playback reads a revision's render manifest, verifies every result it
//! pins, and hands a device a preallocated stream whose callback doesn't
//! allocate, lock or block.

mod device;
#[cfg(feature = "native")]
pub mod native;
mod prepare;

pub use device::{Device, DeviceError, DeviceFormat, SimulatedDevice};
pub use prepare::{prepare, PlaybackError, PreparedAudio, Stream};
