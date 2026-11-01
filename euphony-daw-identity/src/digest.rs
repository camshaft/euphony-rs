use core::fmt;

/// A 32-byte domain-separated content digest.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Digest([u8; 32]);

impl Digest {
    /// Digests `bytes` in the domain `domain`.
    ///
    /// The same bytes in two domains have unrelated digests.
    pub fn of(domain: &str, bytes: &[u8]) -> Self {
        let mut hasher = blake3::Hasher::new_derive_key(domain);
        hasher.update(bytes);
        Self(*hasher.finalize().as_bytes())
    }

    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest({self})")
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
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
    fn display_is_lowercase_hex() {
        let digest = Digest::from_bytes([0xab; 32]);
        assert_eq!(digest.to_string(), "ab".repeat(32));
    }
}
