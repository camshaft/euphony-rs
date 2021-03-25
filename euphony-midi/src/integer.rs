use crate::codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue};
use core::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ConversionError;

macro_rules! impl_integer {
    ($name:ident, $bitsize:expr, $inner:ident) => {
        #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[allow(non_camel_case_types)]
        pub struct $name($inner);

        impl $name {
            pub const MAX: Self = Self(((1u64 << $bitsize) - 1) as $inner);

            pub fn new<T: TryInto<$inner>>(value: T) -> Option<Self> {
                let value: $inner = value.try_into().ok()?;
                value.try_into().ok()
            }

            pub fn new_lossy(value: $inner) -> Self {
                Self(value & Self::MAX.0)
            }
        }

        impl Into<$inner> for $name {
            fn into(self) -> $inner {
                self.0
            }
        }

        impl TryFrom<$inner> for $name {
            type Error = ConversionError;

            fn try_from(raw: $inner) -> Result<Self, Self::Error> {
                let trunc = raw & Self::MAX.0;
                if trunc == raw {
                    Ok($name(trunc))
                } else {
                    Err(ConversionError)
                }
            }
        }

        impl core::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

impl_integer!(u2, 2, u8);
impl_integer!(u4, 4, u8);

impl_integer!(u7, 7, u8);

impl MIDIValue for u7 {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        match buffer.next_byte()?.try_into() {
            Ok(value) => Ok(value),
            Err(_) => Err(DecoderError::InvariantViolation("Integer overflow")),
        }
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_byte(self.0)
    }

    fn encoding_len(&self) -> usize {
        1
    }
}

impl MIDIValue for u8 {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        buffer.next_byte()
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_byte(*self)
    }

    fn encoding_len(&self) -> usize {
        1
    }
}

impl_integer!(u14le, 14, u16);

impl MIDIValue for u14le {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let lsb = (*buffer.decode::<u7>()?) as u16;
        let msb = (*buffer.decode::<u7>()? as u16) << 7;
        Ok(Self(msb + lsb))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.encode(&u7::new_lossy(self.0 as u8))?;
        buffer.encode(&u7::new_lossy((self.0 >> 7) as u8))?;
        Ok(())
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

#[test]
fn u14le_round_trip_test() {
    let mut buffer = vec![];
    for expected in 0u16..(*u14le::MAX) {
        let expected = u14le::new(expected).unwrap();
        expected.encode(&mut buffer).unwrap();
        let actual = u14le::decode(&mut &buffer[..]);
        assert_eq!(Ok(expected), actual);
        buffer.clear();
    }
}

impl_integer!(u15le, 15, u16);

impl MIDIValue for u15le {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        Ok(Self(u16::from_le_bytes(buffer.decode()?)))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_bytes(&self.0.to_le_bytes())
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

impl_integer!(u15be, 15, u16);

impl MIDIValue for u15be {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        Ok(Self(u16::from_be_bytes(buffer.decode()?)))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_bytes(&self.0.to_be_bytes())
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

impl_integer!(u16le, 16, u16);

impl MIDIValue for u16le {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        Ok(Self(u16::from_le_bytes(buffer.decode()?)))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_bytes(&self.0.to_le_bytes())
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

impl_integer!(u16be, 16, u16);

impl MIDIValue for u16be {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        Ok(Self(u16::from_be_bytes(buffer.decode()?)))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_bytes(&self.0.to_be_bytes())
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

impl_integer!(u24be, 24, u32);

impl MIDIValue for u24be {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let [a, b, c]: [u8; 3] = buffer.decode()?;
        Ok(Self(u32::from_be_bytes([0, a, b, c])))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_bytes(&self.0.to_be_bytes()[1..])
    }

    fn encoding_len(&self) -> usize {
        3
    }
}

impl_integer!(u28varint, 28, u32);

impl u28varint {
    fn bytes(&self) -> impl Iterator<Item = u8> {
        let value: u32 = **self;
        let mut has_value = false;
        [0x0, 0x80, 0x80, 0x80]
            .iter()
            .enumerate()
            .rev()
            .filter_map(move |(i, mask)| {
                let byte = ((value >> (i * 7)) as u8) & 0x7F;
                if has_value || byte != 0 {
                    has_value = true;
                    Some(byte | mask)
                } else {
                    None
                }
            })
    }
}

impl MIDIValue for u28varint {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            let byte = buffer.next_byte()?;
            value <<= 7;
            value |= *u7::new_lossy(byte) as u32;
            // check the most significant bit
            if byte & 0x80 == 0 {
                return Ok(u28varint(value));
            }
        }
        Err(DecoderError::InvariantViolation("varint value overflow"))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        if self.0 == 0 {
            return buffer.write_byte(0);
        }

        for byte in self.bytes() {
            buffer.write_byte(byte)?;
        }

        Ok(())
    }

    fn encoding_len(&self) -> usize {
        core::cmp::max(self.bytes().count(), 1)
    }
}

#[test]
fn u28varint_test() {
    let tests = [
        (&[0x00][..], 0x0000_0000),
        (&[0x67][..], 0x0000_0067),
        (&[0x7F][..], 0x0000_007F),
        (&[0x81, 0x00][..], 0x0000_0080),
        (&[0xC6, 0x45][..], 0x0000_2345),
        (&[0xFF, 0x7F][..], 0x0000_3FFF),
        (&[0x81, 0x80, 0x00][..], 0x0000_4000),
        (&[0xC8, 0xE8, 0x56][..], 0x0012_3456),
        (&[0xFF, 0xFF, 0x7F][..], 0x001F_FFFF),
        (&[0x81, 0x80, 0x80, 0x00][..], 0x0020_0000),
        (&[0xC4, 0xEA, 0xF9, 0x5E][..], 0x089A_BCDE),
        (&[0xFF, 0xFF, 0xFF, 0x7F][..], 0x0FFF_FFFF),
    ];

    let mut out = vec![];

    for (buffer, value) in tests.iter() {
        let value = u28varint::new(*value).expect("value overflow");
        assert_eq!((&buffer[..]).decode::<u28varint>().unwrap(), value);

        out.clear();
        out.encode(&value).unwrap();
        assert_eq!(&out[..], &buffer[..]);
    }
}

impl_integer!(u32be, 32, u32);

impl MIDIValue for u32be {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        Ok(Self(u32::from_be_bytes(buffer.decode()?)))
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_bytes(&self.0.to_be_bytes())
    }

    fn encoding_len(&self) -> usize {
        4
    }
}
