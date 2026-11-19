//! The binary32 working-audio contract.
//!
//! Working audio is IEEE 754 binary32 at every processor interface, cache
//! boundary, stored object and peer transfer. Its canonical bytes are a
//! versioned header followed by interleaved little-endian sample bits, and
//! every encoding keeps each admitted sample's bits, signed zero and
//! subnormal values included.

mod audio;
mod profile;

pub use audio::{ChannelLayout, FormatError, Header, WorkingAudio};
pub use profile::{
    FusedOperations, NumericalProfile, Precision, ProcessorDefinition, ProcessorId, ProfileId,
    Registration, RegistrationError, Registry, Rounding, Simd, Subnormals,
};
