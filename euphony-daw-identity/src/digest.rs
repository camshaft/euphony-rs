use core::fmt;

/// The hash function a [`Digest`] was computed with.
///
/// Every encoded digest carries its algorithm's code before its bytes, so a
/// later hash function gets a new code and never changes what an identity
/// written under an earlier one means.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Algorithm {
    /// BLAKE3 with a 256-bit output, in key-derivation mode for domain
    /// separation.
    Blake3,
}

impl Algorithm {
    /// The byte that names this algorithm in an encoded digest.
    pub const fn code(self) -> u8 {
        match self {
            Self::Blake3 => 1,
        }
    }

    /// The algorithm a code names, or `None` for a code this build doesn't
    /// know.
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Blake3),
            _ => None,
        }
    }
}

/// A 32-byte domain-separated content digest, with the algorithm that
/// computed it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Digest {
    algorithm: Algorithm,
    bytes: [u8; 32],
}

impl Digest {
    /// The length of an encoded digest: the algorithm's code, then the
    /// digest's bytes.
    pub const ENCODED_LEN: usize = 33;

    /// Digests `bytes` in the domain `domain`, with BLAKE3-256.
    ///
    /// The same bytes in two domains have unrelated digests.
    pub fn of(domain: &str, bytes: &[u8]) -> Self {
        let mut hasher = blake3::Hasher::new_derive_key(domain);
        hasher.update(bytes);
        Self::new(Algorithm::Blake3, *hasher.finalize().as_bytes())
    }

    pub const fn new(algorithm: Algorithm, bytes: [u8; 32]) -> Self {
        Self { algorithm, bytes }
    }

    /// A BLAKE3-256 digest with these bytes.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self::new(Algorithm::Blake3, bytes)
    }

    pub const fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }

    /// The digest as it is encoded: the algorithm's code, then its bytes.
    pub fn encoded(&self) -> [u8; Self::ENCODED_LEN] {
        let mut out = [0; Self::ENCODED_LEN];
        out[0] = self.algorithm.code();
        out[1..].copy_from_slice(&self.bytes);
        out
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest({self})")
    }
}

/// Lowercase hex of the encoded digest: the algorithm's code, then the
/// digest's bytes.
impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.encoded() {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domains_separate_equal_bytes() {
        let a = Digest::of("euphony test a", b"bytes");
        let b = Digest::of("euphony test b", b"bytes");
        assert_ne!(a, b);
        assert_eq!(a, Digest::of("euphony test a", b"bytes"));
    }

    #[test]
    fn display_is_lowercase_hex_after_the_algorithm() {
        let digest = Digest::from_bytes([0xab; 32]);
        assert_eq!(digest.to_string(), format!("01{}", "ab".repeat(32)));
    }

    #[test]
    fn digests_name_blake3_as_algorithm_one() {
        let digest = Digest::of("euphony test", b"bytes");
        assert_eq!(digest.algorithm(), Algorithm::Blake3);
        assert_eq!(Algorithm::Blake3.code(), 1);
        assert_eq!(Algorithm::from_code(1), Some(Algorithm::Blake3));
        assert_eq!(digest.encoded()[0], 1);
        assert_eq!(&digest.encoded()[1..], digest.as_bytes());
    }

    #[test]
    fn every_other_algorithm_code_is_unknown() {
        for code in (0..=u8::MAX).filter(|code| *code != 1) {
            assert_eq!(Algorithm::from_code(code), None, "code {code}");
        }
    }
}
