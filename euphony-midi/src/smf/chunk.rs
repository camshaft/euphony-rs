use crate::{
    codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue},
    integer::u32be,
};

pub enum ChunkHeader {
    Header(usize),
    Track(usize),
    Undefined([u8; 4], usize),
}

impl ChunkHeader {
    pub fn len(&self) -> usize {
        match self {
            Self::Header(len) => *len,
            Self::Track(len) => *len,
            Self::Undefined(_, len) => *len,
        }
    }
}

const HEADER_CHUNK: &[u8; 4] = b"MThd";
const TRACK_CHUNK: &[u8; 4] = b"MTrk";

impl MIDIValue for ChunkHeader {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let id: [u8; 4] = buffer.decode()?;
        let len = (*buffer.decode::<u32be>()?) as usize;
        match &id {
            HEADER_CHUNK => Ok(Self::Header(len)),
            TRACK_CHUNK => Ok(Self::Track(len)),
            _ => Ok(Self::Undefined(id, len)),
        }
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        let (id, len) = match self {
            ChunkHeader::Header(len) => (HEADER_CHUNK, *len),
            ChunkHeader::Track(len) => (TRACK_CHUNK, *len),
            ChunkHeader::Undefined(id, len) => (id, *len),
        };
        buffer.write_bytes(id)?;
        buffer.encode(&u32be::new_lossy(len as u32))?;
        Ok(())
    }

    fn encoding_len(&self) -> usize {
        4 + u32be::new_lossy(self.len() as u32).encoding_len()
    }
}
