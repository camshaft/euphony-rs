macro_rules! midi_value {
    ($name:ident, $ty:ident) => {
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub struct $name($ty);

        impl $name {
            pub const MAX: Self = Self(
                (1 << (core::mem::size_of::<$ty>() as $ty * 8
                    - core::mem::size_of::<$ty>() as $ty))
                    - 1,
            );

            pub fn new(value: $ty) -> Option<Self> {
                if value <= Self::MAX.0 {
                    Some(Self(value))
                } else {
                    None
                }
            }
        }

        impl core::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl crate::midi::codec::MIDIValue for $name {
            fn decode<B: crate::midi::codec::DecoderBuffer>(
                buffer: &mut B,
            ) -> Result<Self, crate::midi::codec::DecoderError> {
                Ok(Self(buffer.decode::<$ty>()?))
            }

            fn encode<B: crate::midi::codec::EncoderBuffer>(
                &self,
                buffer: &mut B,
            ) -> Result<usize, crate::midi::codec::EncoderError> {
                self.0.encode(buffer)
            }
        }
    };
}

pub mod channel;
pub mod codec;
pub mod controller;
pub mod key;
pub mod message;
pub mod pitch_bend;
pub mod pressure;
pub mod program;
pub mod song;
pub mod sysex;
pub mod timecode;
pub mod velocity;
