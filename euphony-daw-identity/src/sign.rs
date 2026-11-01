use crate::Digest;
use core::fmt;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

/// A key that signs revisions and branch references for one author.
pub struct AuthorKey {
    key: SigningKey,
}

impl AuthorKey {
    /// The key whose secret is `seed`.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(&seed),
        }
    }

    pub fn author(&self) -> AuthorId {
        AuthorId(self.key.verifying_key().to_bytes())
    }

    /// Signs the digest of `message` in the domain `domain`.
    pub fn sign(&self, domain: &str, message: &[u8]) -> Signature {
        let digest = Digest::of(domain, message);
        Signature(self.key.sign(digest.as_bytes()).to_bytes())
    }
}

impl fmt::Debug for AuthorKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthorKey")
            .field("author", &self.author())
            .finish_non_exhaustive()
    }
}

/// The public identity of an author: their verifying key.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorId([u8; 32]);

impl AuthorId {
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Checks that `signature` signs `message` in the domain `domain` with
    /// this author's key.
    pub fn verify(
        &self,
        domain: &str,
        message: &[u8],
        signature: &Signature,
    ) -> Result<(), SignatureError> {
        let key = VerifyingKey::from_bytes(&self.0);
        let key = key.map_err(|_| SignatureError::InvalidKey)?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature.0);
        let digest = Digest::of(domain, message);
        let verified = key.verify(digest.as_bytes(), &signature);
        verified.map_err(|_| SignatureError::Mismatch)
    }
}

impl fmt::Debug for AuthorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthorId({})", Digest::from_bytes(self.0))
    }
}

/// An author's signature over a domain-separated digest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Signature([u8; 64]);

impl Signature {
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

/// Why a signature didn't verify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignatureError {
    /// The author's identity isn't a valid public key.
    InvalidKey,
    /// The signature doesn't sign the message with the author's key.
    Mismatch,
}

impl fmt::Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey => f.write_str("the author's identity isn't a valid key"),
            Self::Mismatch => f.write_str("the signature doesn't match the author's key"),
        }
    }
}

impl std::error::Error for SignatureError {}

#[cfg(test)]
mod tests {
    use super::*;

    const DOMAIN: &str = "euphony test signature";

    #[test]
    fn verifies_its_own_signature() {
        let key = AuthorKey::from_seed([1; 32]);
        let signature = key.sign(DOMAIN, b"revision");
        let verified = key.author().verify(DOMAIN, b"revision", &signature);
        assert_eq!(verified, Ok(()));
    }

    #[test]
    fn refuses_another_message_domain_or_author() {
        let key = AuthorKey::from_seed([1; 32]);
        let other = AuthorKey::from_seed([2; 32]);
        let signature = key.sign(DOMAIN, b"revision");
        let author = key.author();
        let mismatch = Err(SignatureError::Mismatch);
        let message = author.verify(DOMAIN, b"revisioN", &signature);
        assert_eq!(message, mismatch);
        let domain = author.verify("euphony other domain", b"revision", &signature);
        assert_eq!(domain, mismatch);
        let author = other.author().verify(DOMAIN, b"revision", &signature);
        assert_eq!(author, mismatch);
    }
}
