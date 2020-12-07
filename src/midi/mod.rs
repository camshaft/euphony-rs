macro_rules! midi_value {
    ($name:ident, $ty:ident) => {
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name($ty);

        impl $name {
            pub const MAX: Self = Self($ty::MAX);

            pub fn new<T: core::convert::TryInto<$ty>>(value: T) -> Option<Self> {
                Some(Self(T::try_into(value).ok()?))
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
            ) -> Result<(), B::Error> {
                self.0.encode(buffer)
            }

            fn encoding_len(&self) -> usize {
                self.0.encoding_len()
            }
        }
    };
}

pub mod channel;
pub mod codec;
pub mod controller;
pub mod integer;
pub mod key;
pub mod message;
pub mod pitch_bend;
pub mod pressure;
pub mod program;
pub mod smf;
pub mod song;
pub mod sysex;
pub mod timecode;
pub mod velocity;
