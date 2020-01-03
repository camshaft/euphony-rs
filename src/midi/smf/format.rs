use crate::midi::{
    codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue},
    integer::u16be,
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Format {
    Single,
    Parallel,
    Sequential,
}

impl MIDIValue for Format {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        Ok(match *buffer.decode::<u16be>()? {
            0 => Format::Single,
            1 => Format::Parallel,
            2 => Format::Sequential,
            _ => return Err(DecoderError::InvariantViolation("invalid format")),
        })
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        buffer.encode(&u16be::new_lossy(match self {
            Format::Single => 0u16,
            Format::Parallel => 1,
            Format::Sequential => 2,
        }))
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

#[test]
fn interop_test() {
    use midly::Format as MFormat;

    fn assert_interop(f: Format, m: MFormat) {
        let mut bytes = vec![];
        f.encode(&mut bytes).unwrap();
        assert_eq!(&m.encode()[..], &bytes[..], "{:?}", f);
        assert_eq!(Format::decode(&mut &bytes[..]).unwrap(), f);
    }

    assert_interop(Format::Single, MFormat::SingleTrack);
    assert_interop(Format::Parallel, MFormat::Parallel);
    assert_interop(Format::Sequential, MFormat::Sequential);
}
