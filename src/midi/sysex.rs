use crate::midi::codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]

pub struct SysExPayload(Vec<u8>);

// TODO implement conversion?

impl MIDIValue for SysExPayload {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let mut payload = Vec::new();
        loop {
            match buffer.next_byte() {
                Ok(b @ 0b0000_0000..=0b0111_1111) => payload.push(b),
                Ok(0b1111_0111) => return Ok(Self(payload)),
                Ok(b @ 0b1000_0000..=0b1111_1111) => {
                    return Err(DecoderError::UnexpectedStatusParam(b))
                }
                Err(DecoderError::EOF) => return Ok(Self(payload)),
                Err(err) => return Err(err),
            }
        }
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.write_bytes(&self.0)?;
        Ok(())
    }

    fn encoding_len(&self) -> usize {
        self.0.len()
    }
}
