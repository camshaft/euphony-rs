use crate::Digest;
use core::{convert::Infallible, fmt, marker::PhantomData};
use etude_buffer::{
    reader::{self, Infallible as _},
    writer,
};

/// Writes the canonical encoding of a record into an etude writer.
///
/// Integers are little-endian at a fixed width, and every variable-length
/// field carries its length first, so one encoding never reads as another.
/// A write past the writer's capacity is refused: the encoder stops writing,
/// and [`Encoder::into_writer`] answers [`EncodeError::Capacity`].
#[derive(Clone, Debug, Default)]
pub struct Encoder<W = Vec<u8>> {
    out: W,
    overflowed: bool,
}

impl Encoder {
    /// An encoder that writes into a growing vector.
    pub fn new() -> Self {
        Self::default()
    }

    /// The digest of the bytes written so far, in the domain `domain`.
    pub fn digest_in(&self, domain: &str) -> Digest {
        Digest::of(domain, &self.out)
    }

    pub fn finish(self) -> Vec<u8> {
        debug_assert!(!self.overflowed, "a vector holds every write");
        self.out
    }
}

impl<W: writer::Buffer> Encoder<W> {
    /// An encoder that writes into `out`.
    pub fn with_writer(out: W) -> Self {
        Self {
            out,
            overflowed: false,
        }
    }

    pub fn u8(&mut self, value: u8) -> &mut Self {
        self.raw(&[value])
    }

    pub fn u16(&mut self, value: u16) -> &mut Self {
        self.raw(&value.to_le_bytes())
    }

    pub fn u32(&mut self, value: u32) -> &mut Self {
        self.raw(&value.to_le_bytes())
    }

    pub fn u64(&mut self, value: u64) -> &mut Self {
        self.raw(&value.to_le_bytes())
    }

    pub fn bool(&mut self, value: bool) -> &mut Self {
        self.u8(u8::from(value))
    }

    /// Writes `value` after its length.
    pub fn bytes(&mut self, value: &[u8]) -> &mut Self {
        self.u64(value.len() as u64);
        self.raw(value)
    }

    pub fn str(&mut self, value: &str) -> &mut Self {
        self.bytes(value.as_bytes())
    }

    pub fn digest(&mut self, value: &Digest) -> &mut Self {
        self.raw(value.as_bytes())
    }

    /// Writes `value` as it is, without its length, for fields of a fixed size.
    pub fn raw(&mut self, value: &[u8]) -> &mut Self {
        if self.overflowed || value.len() > self.out.remaining_capacity() {
            self.overflowed = true;
        } else {
            self.out.put_slice(value);
        }
        self
    }

    /// Ends the write, answering the writer, or the refusal of a write that
    /// didn't fit it.
    pub fn into_writer(self) -> Result<W, EncodeError> {
        match self.overflowed {
            true => Err(EncodeError::Capacity),
            false => Ok(self.out),
        }
    }
}

/// Why a canonical encoding wasn't written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodeError {
    /// The writer didn't have room for the whole record.
    Capacity,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capacity => f.write_str("the writer didn't have room for the encoding"),
        }
    }
}

impl std::error::Error for EncodeError {}

/// Why a canonical encoding didn't decode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The bytes ended before the record did.
    Truncated,
    /// Bytes remained after the record ended.
    Trailing,
    /// A field held a value its record doesn't allow.
    Invalid(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => f.write_str("the encoding ended early"),
            Self::Trailing => f.write_str("bytes followed the end of the encoding"),
            Self::Invalid(field) => write!(f, "the encoding holds an invalid {field}"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Reads a canonical encoding from an etude reader.
///
/// Each length read from the bytes is checked against the bytes the reader
/// holds before anything is allocated, so a claimed size can't force an
/// allocation.
#[derive(Clone, Debug)]
pub struct Decoder<'a, R = &'a [u8]> {
    reader: R,
    bytes: PhantomData<&'a [u8]>,
}

impl<'a> Decoder<'a> {
    /// A decoder that reads `bytes`.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self::with_reader(bytes)
    }
}

impl<R> Decoder<'_, R>
where
    R: reader::Buffer<Error = Infallible>,
{
    /// A decoder that reads the bytes `reader` holds.
    pub fn with_reader(reader: R) -> Self {
        Self {
            reader,
            bytes: PhantomData,
        }
    }

    /// Moves the next `len` bytes into `dest`.
    pub fn take_into<W>(&mut self, len: usize, dest: &mut W) -> Result<(), DecodeError>
    where
        W: writer::Buffer + ?Sized,
    {
        if self.reader.buffered_len() < len {
            return Err(DecodeError::Truncated);
        }
        if dest.remaining_capacity() < len {
            return Err(DecodeError::Invalid("length"));
        }
        let mut left = len;
        while left > 0 {
            let chunk = self.reader.infallible_read_chunk(left);
            if chunk.is_empty() {
                return Err(DecodeError::Truncated);
            }
            left -= chunk.len();
            dest.put_chunk(chunk);
        }
        Ok(())
    }

    /// Reads a field of a fixed size.
    pub fn array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        let mut out = [0; N];
        self.take_into(N, &mut &mut out[..])?;
        Ok(out)
    }

    pub fn u8(&mut self) -> Result<u8, DecodeError> {
        Ok(self.array::<1>()?[0])
    }

    pub fn u16(&mut self) -> Result<u16, DecodeError> {
        Ok(u16::from_le_bytes(self.array()?))
    }

    pub fn u32(&mut self) -> Result<u32, DecodeError> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    pub fn u64(&mut self) -> Result<u64, DecodeError> {
        Ok(u64::from_le_bytes(self.array()?))
    }

    pub fn bool(&mut self) -> Result<bool, DecodeError> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::Invalid("bool")),
        }
    }

    /// Reads a field written with its length.
    pub fn bytes(&mut self) -> Result<Vec<u8>, DecodeError> {
        let len = self.u64()?;
        let len = usize::try_from(len).map_err(|_| DecodeError::Truncated)?;
        if len > self.remaining() {
            return Err(DecodeError::Truncated);
        }
        let mut out = Vec::with_capacity(len);
        self.take_into(len, &mut out)?;
        Ok(out)
    }

    pub fn str(&mut self) -> Result<String, DecodeError> {
        let bytes = self.bytes()?;
        String::from_utf8(bytes).map_err(|_| DecodeError::Invalid("text"))
    }

    pub fn digest(&mut self) -> Result<Digest, DecodeError> {
        Ok(Digest::from_bytes(self.array()?))
    }

    /// Reads a count of items, refusing a count that the remaining bytes can't
    /// hold at `min_item_len` bytes an item.
    pub fn count(&mut self, min_item_len: usize) -> Result<usize, DecodeError> {
        let count = self.u64()?;
        let count = usize::try_from(count).map_err(|_| DecodeError::Truncated)?;
        match count.checked_mul(min_item_len.max(1)) {
            Some(len) if len <= self.remaining() => Ok(count),
            _ => Err(DecodeError::Truncated),
        }
    }

    pub fn remaining(&self) -> usize {
        self.reader.buffered_len()
    }

    /// Ends the read, refusing bytes after the record.
    pub fn finish(self) -> Result<(), DecodeError> {
        match self.reader.buffer_is_empty() {
            true => Ok(()),
            false => Err(DecodeError::Trailing),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use etude_buffer::reader::Buffer as _;

    fn sample() -> Vec<u8> {
        let digest = Digest::of("euphony test", b"x");
        let mut encoder = Encoder::new();
        encoder
            .u8(7)
            .u16(0x1234)
            .u32(0xdead_beef)
            .u64(u64::MAX)
            .bool(true)
            .str("track")
            .digest(&digest);
        encoder.finish()
    }

    fn check<R>(mut decoder: Decoder<'_, R>)
    where
        R: reader::Buffer<Error = Infallible>,
    {
        let digest = Digest::of("euphony test", b"x");
        assert_eq!(decoder.u8(), Ok(7));
        assert_eq!(decoder.u16(), Ok(0x1234));
        assert_eq!(decoder.u32(), Ok(0xdead_beef));
        assert_eq!(decoder.u64(), Ok(u64::MAX));
        assert_eq!(decoder.bool(), Ok(true));
        assert_eq!(decoder.str(), Ok("track".to_string()));
        assert_eq!(decoder.digest(), Ok(digest));
        assert_eq!(decoder.finish(), Ok(()));
    }

    #[test]
    fn round_trip() {
        let bytes = sample();
        check(Decoder::new(&bytes));
    }

    #[test]
    fn round_trip_across_reader_chunks() {
        // Each split puts a field's bytes in two chunks of a chained reader.
        let bytes = sample();
        for split in 0..=bytes.len() {
            let (head, tail) = bytes.split_at(split);
            check(Decoder::with_reader(head.chain(tail)));
        }
    }

    #[test]
    fn writes_into_a_bounded_writer() {
        let bytes = sample();
        let mut out = vec![0; bytes.len()];
        let mut encoder = Encoder::with_writer(&mut out[..]);
        let digest = Digest::of("euphony test", b"x");
        encoder.u8(7).u16(0x1234).u32(0xdead_beef).u64(u64::MAX);
        encoder.bool(true).str("track").digest(&digest);
        let rest = encoder.into_writer().unwrap();
        assert!(rest.is_empty());
        assert_eq!(out, bytes);
    }

    #[test]
    fn refuses_a_write_past_the_writer() {
        let mut out = [0; 3];
        let mut encoder = Encoder::with_writer(&mut out[..]);
        encoder.u32(1).u8(2);
        assert_eq!(encoder.into_writer().err(), Some(EncodeError::Capacity));
        assert_eq!(out, [0; 3]);
    }

    #[test]
    fn refuses_truncated_and_trailing_bytes() {
        let mut encoder = Encoder::new();
        encoder.u32(1);
        let bytes = encoder.finish();
        let truncated = Decoder::new(&bytes[..3]).u32();
        assert_eq!(truncated, Err(DecodeError::Truncated));

        let mut decoder = Decoder::new(&bytes);
        assert_eq!(decoder.u8(), Ok(1));
        assert_eq!(decoder.remaining(), 3);
        assert_eq!(decoder.finish(), Err(DecodeError::Trailing));
    }

    #[test]
    fn a_claimed_length_beyond_the_bytes_is_refused() {
        let mut encoder = Encoder::new();
        encoder.u64(u64::MAX);
        let bytes = encoder.finish();
        assert_eq!(Decoder::new(&bytes).bytes(), Err(DecodeError::Truncated));
        assert_eq!(Decoder::new(&bytes).count(1), Err(DecodeError::Truncated));
    }

    #[test]
    fn refuses_invalid_bool_and_text() {
        let invalid = Decoder::new(&[2]).bool();
        assert_eq!(invalid, Err(DecodeError::Invalid("bool")));

        let mut encoder = Encoder::new();
        encoder.bytes(&[0xff]);
        let bytes = encoder.finish();
        let invalid = Decoder::new(&bytes).str();
        assert_eq!(invalid, Err(DecodeError::Invalid("text")));
    }
}
