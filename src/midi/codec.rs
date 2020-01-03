use crate::{
    midi::integer::{u14le, u24be},
    time::{beat::Beat, tempo::Tempo, time_signature::TimeSignature},
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DecoderError {
    EOF,
    UnexpectedStatusParam(u8),
    InvariantViolation(&'static str),
}

pub trait DecoderBuffer: Sized {
    fn next_byte(&mut self) -> Result<u8, DecoderError>;

    fn skip(&mut self, len: usize) -> Result<(), DecoderError>;

    fn decode<T: MIDIValue>(&mut self) -> Result<T, DecoderError> {
        T::decode(self)
    }
}

impl DecoderBuffer for &[u8] {
    fn skip(&mut self, len: usize) -> Result<(), DecoderError> {
        if self.len() < len {
            let (_, remaining) = self.split_at(len);
            *self = remaining;
            Ok(())
        } else {
            Err(DecoderError::EOF)
        }
    }

    fn next_byte(&mut self) -> Result<u8, DecoderError> {
        if self.is_empty() {
            Err(DecoderError::EOF)
        } else {
            let (value, remaining) = self.split_at(1);
            *self = remaining;
            Ok(value[0])
        }
    }
}

pub trait EncoderBuffer: Sized {
    type Error;

    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Error>;
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;

    fn encode<T: MIDIValue>(&mut self, value: &T) -> Result<(), Self::Error> {
        T::encode(value, self)
    }
}

#[cfg(not(feature = "std"))]
impl EncoderBuffer for alloc::vec::Vec<u8> {
    type Error = ();

    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.push(byte);
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> EncoderBuffer for W {
    type Error = std::io::Error;

    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.write_bytes(&[byte])
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write_all(bytes)?;
        Ok(())
    }
}

pub trait MIDIValue: Sized {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError>;
    fn encode<B: EncoderBuffer>(&self, bytes: &mut B) -> Result<(), B::Error>;
    fn encoding_len(&self) -> usize;
}

macro_rules! impl_array {
    ([]) => {
        impl_array!(@impl,);
    };
    ([$i:expr $(, $rest:expr)*]) => {
        impl_array!(@impl, $i $(,$rest)*);
        impl_array!([$($rest),*]);
    };
    (@impl, $($i:expr),*) => {
        impl<T: MIDIValue> MIDIValue for [T; 0 $(+ $i)*] {
            fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
                let value = [
                    $(
                        {
                            $i;
                            buffer.decode()?
                        },
                    )*
                ];
                let _ = buffer;
                Ok(value)
            }

            fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
                for v in self.iter() {
                    buffer.encode(v)?;
                }
                Ok(())
            }

            fn encoding_len(&self) -> usize {
                self.iter().map(T::encoding_len).sum()
            }
        }
    };
}

impl_array!([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

impl MIDIValue for Beat {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let count = *buffer.decode::<u14le>()?;
        Ok(Beat(1, 16) * count)
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        let count = (*self / Beat(1, 16)).whole() as u16;
        buffer.encode(&u14le::new_lossy(count))
    }

    fn encoding_len(&self) -> usize {
        unimplemented!()
    }
}

impl TimeSignature {
    fn midi_denominator(&self) -> u8 {
        match self.1 {
            1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            16 => 4,
            _ => panic!("invalid TimeSignature for midi {:?}", self),
        }
    }
}

impl MIDIValue for TimeSignature {
    fn decode<B: DecoderBuffer>(_buffer: &mut B) -> Result<Self, DecoderError> {
        unimplemented!()
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_byte(self.0 as u8)?;
        buffer.write_byte(self.midi_denominator())?;
        Ok(())
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

impl Tempo {
    fn as_midi_u24(&self) -> Option<u24be> {
        u24be::new(self.as_beat_duration().as_micros())
    }
}

impl MIDIValue for Tempo {
    fn decode<B: DecoderBuffer>(_buffer: &mut B) -> Result<Self, DecoderError> {
        unimplemented!()
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.encode(&self.as_midi_u24().expect("Tempo value does not fix"))
    }

    fn encoding_len(&self) -> usize {
        self.as_midi_u24()
            .expect("Tempo value does not fix")
            .encoding_len()
    }
}
