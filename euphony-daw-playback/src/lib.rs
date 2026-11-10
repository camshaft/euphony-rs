//! Prepared playback through a device adapter.
//!
//! Playback reads a revision's render manifest, verifies every result it
//! pins, and plays it through a ring: a reader thread fills the ring, and
//! the device callback pops from it without allocating, locking or
//! blocking.

mod device;
#[cfg(any(feature = "native", test))]
pub mod native;
mod prepare;
pub mod ring;

pub use device::{Device, DeviceError, DeviceFormat, SimulatedDevice};
pub use prepare::{prepare, PlaybackError, PreparedAudio, Stream};
pub use ring::{PlaybackEvent, TransportError};
