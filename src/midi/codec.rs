use crate::time::beat::Beat;

pub enum DecoderError {
    EOF,
    UnexpectedStatusParam(u8),
}

pub trait DecoderBuffer: Sized {
    fn next_byte(&mut self) -> Result<u8, DecoderError>;

    fn decode<T: MIDIValue>(&mut self) -> Result<T, DecoderError> {
        T::decode(self)
    }
}

impl DecoderBuffer for &[u8] {
    fn next_byte(&mut self) -> Result<u8, DecoderError> {
        if self.is_empty() {
            Err(DecoderError::EOF)
        } else {
            let (value, remainging) = self.split_at(1);
            *self = remainging;
            Ok(value[0])
        }
    }
}

pub type EncoderError = (); // TODO

pub trait EncoderBuffer: Sized {
    fn write_byte(&mut self, byte: u8) -> Result<usize, EncoderError>;

    fn encode<T: MIDIValue>(&mut self, value: &T) -> Result<usize, EncoderError> {
        T::encode(value, self)
    }
}

impl EncoderBuffer for Vec<u8> {
    fn write_byte(&mut self, byte: u8) -> Result<usize, EncoderError> {
        self.push(byte);
        Ok(1)
    }
}

pub trait MIDIValue: Sized {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError>;
    fn encode<B: EncoderBuffer>(&self, bytes: &mut B) -> Result<usize, EncoderError>;
}

impl MIDIValue for u8 {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let value = buffer.next_byte()?;
        if value < 128 {
            Ok(value)
        } else {
            Err(DecoderError::UnexpectedStatusParam(value))
        }
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<usize, EncoderError> {
        buffer.write_byte(self & 127)
    }
}

impl MIDIValue for u16 {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let lsb = buffer.decode::<u8>()? as u16;
        let msb = (buffer.decode::<u8>()? as u16) << 7;
        Ok(msb + lsb)
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<usize, EncoderError> {
        Ok(buffer.encode(&(*self as u8))? + buffer.encode(&((self >> 7) as u8))?)
    }
}

impl MIDIValue for Beat {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let count = buffer.decode::<u16>()?;
        Ok(Beat(1, 16) * count)
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<usize, EncoderError> {
        let count = (*self / Beat(1, 16)).to_integer() as u16;
        buffer.encode(&count)
    }
}
