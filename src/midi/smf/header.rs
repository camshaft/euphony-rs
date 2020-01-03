use crate::midi::{
    codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue},
    integer::u16be,
    smf::{format::Format, timing::Timing},
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Header {
    pub format: Format,
    pub track_count: u16,
    pub timing: Timing,
}

impl MIDIValue for Header {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        let format = buffer.decode()?;
        let track_count = *buffer.decode::<u16be>()?;
        let timing = buffer.decode()?;
        Ok(Self {
            format,
            track_count,
            timing,
        })
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.encode(&self.format)?;
        buffer.encode(&u16be::new_lossy(self.track_count))?;
        buffer.encode(&self.timing)?;
        Ok(())
    }

    fn encoding_len(&self) -> usize {
        self.format.encoding_len()
            + u16be::new_lossy(self.track_count).encoding_len()
            + self.timing.encoding_len()
    }
}
