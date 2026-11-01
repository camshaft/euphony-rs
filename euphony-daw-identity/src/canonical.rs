use crate::Digest;
use core::fmt;

/// Writes the canonical encoding of a record.
///
/// Integers are little-endian at a fixed width, and every variable-length
/// field carries its length first, so one encoding never reads as another.
#[derive(Clone, Debug, Default)]
pub struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn u8(&mut self, value: u8) -> &mut Self {
        self.bytes.push(value);
        self
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
        self.bytes.extend_from_slice(value);
        self
    }

    /// The digest of the bytes written so far, in the domain `domain`.
    pub fn digest_in(&self, domain: &str) -> Digest {
        Digest::of(domain, &self.bytes)
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

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

/// Reads a canonical encoding.
///
/// Each length read from the bytes is checked against the bytes that remain
/// before anything is allocated, so a claimed size can't force an allocation.
#[derive(Clone, Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// Reads the next `len` bytes as they are.
    pub fn take(&mut self, len: usize) -> Result<&'a [u8], DecodeError> {
        let bytes = self.bytes;
        if bytes.len() < len {
            return Err(DecodeError::Truncated);
        }
        let (head, tail) = bytes.split_at(len);
        self.bytes = tail;
        Ok(head)
    }

    /// Reads a field of a fixed size.
    pub fn array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        let mut out = [0; N];
        out.copy_from_slice(self.take(N)?);
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
    pub fn bytes(&mut self) -> Result<&'a [u8], DecodeError> {
        let len = self.u64()?;
        let len = usize::try_from(len).map_err(|_| DecodeError::Truncated)?;
        self.take(len)
    }

    pub fn str(&mut self) -> Result<&'a str, DecodeError> {
        let bytes = self.bytes()?;
        match core::str::from_utf8(bytes) {
            Ok(text) => Ok(text),
            Err(_) => Err(DecodeError::Invalid("text")),
        }
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
            Some(len) if len <= self.bytes.len() => Ok(count),
            _ => Err(DecodeError::Truncated),
        }
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len()
    }

    /// Ends the read, refusing bytes after the record.
    pub fn finish(self) -> Result<(), DecodeError> {
        match self.bytes.is_empty() {
            true => Ok(()),
            false => Err(DecodeError::Trailing),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
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
        let bytes = encoder.finish();

        let mut decoder = Decoder::new(&bytes);
        assert_eq!(decoder.u8(), Ok(7));
        assert_eq!(decoder.u16(), Ok(0x1234));
        assert_eq!(decoder.u32(), Ok(0xdead_beef));
        assert_eq!(decoder.u64(), Ok(u64::MAX));
        assert_eq!(decoder.bool(), Ok(true));
        assert_eq!(decoder.str(), Ok("track"));
        assert_eq!(decoder.digest(), Ok(digest));
        assert_eq!(decoder.finish(), Ok(()));
    }

    #[test]
    fn refuses_truncated_and_trailing_bytes() {
        let mut encoder = Encoder::new();
        encoder.u32(1);
        let bytes = encoder.finish();
        let truncated = Decoder::new(&bytes[..3]).u32();
        assert_eq!(truncated, Err(DecodeError::Truncated));

        let mut decoder = Decoder::new(&bytes);
        decoder.u8().unwrap();
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
    fn refuses_invalid_bool() {
        let invalid = Decoder::new(&[2]).bool();
        assert_eq!(invalid, Err(DecodeError::Invalid("bool")));
    }
}
