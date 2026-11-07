//! Immutable content-addressed objects and durable journals.
//!
//! An object is named by the domain-separated digest of its bytes, is
//! published whole or not at all, and is checked against its digest on every
//! read. A journal holds the ordered records an authority must not lose.

mod dir;
mod journal;
mod mem;

pub use dir::DirStore;
pub use journal::{Journal, JournalError};
pub use mem::MemStore;

use core::fmt;
use euphony_daw_identity::Digest;
use std::io;

/// The kind of an object, which sets the domain its digest is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectKind {
    /// Canonical binary32 working audio.
    WorkingAudio,
    /// A render manifest.
    Manifest,
}

impl ObjectKind {
    pub const ALL: [Self; 2] = [Self::WorkingAudio, Self::Manifest];

    pub fn domain(self) -> &'static str {
        match self {
            Self::WorkingAudio => euphony_daw_sample::WorkingAudio::DOMAIN,
            Self::Manifest => "euphony render manifest v1",
        }
    }

    /// The digest an object of this kind with these bytes has.
    pub fn digest(self, bytes: &[u8]) -> Digest {
        Digest::of(self.domain(), bytes)
    }

    fn dir(self) -> &'static str {
        match self {
            Self::WorkingAudio => "audio",
            Self::Manifest => "manifest",
        }
    }
}

/// Why a store operation failed.
#[derive(Debug)]
pub enum StoreError {
    /// The store holds no object of this kind with this digest.
    Absent(Digest),
    /// The stored bytes don't match their digest.
    Corrupt(Digest),
    /// The bytes given to publish don't have the digest expected of them.
    Mismatch { expected: Digest, actual: Digest },
    /// The storage refused the operation.
    Io(io::Error),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absent(digest) => write!(f, "object {digest} is absent"),
            Self::Corrupt(digest) => write!(f, "object {digest} doesn't match its digest"),
            Self::Mismatch { expected, actual } => {
                write!(f, "expected object {expected}, received {actual}")
            }
            Self::Io(error) => write!(f, "storage refused the operation: {error}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<io::Error> for StoreError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// A store of immutable objects.
pub trait Store {
    /// Publishes `bytes` as one object, atomically: it is readable whole once
    /// this returns, and not at all before. Publishing bytes the store holds
    /// already returns the same digest.
    fn publish(&self, kind: ObjectKind, bytes: &[u8]) -> Result<Digest, StoreError>;

    /// Reads an object and checks its bytes against its digest.
    fn get(&self, kind: ObjectKind, digest: &Digest) -> Result<Vec<u8>, StoreError>;

    fn contains(&self, kind: ObjectKind, digest: &Digest) -> bool;

    /// Publishes `bytes` only where they have the digest `expected`.
    fn publish_expected(
        &self,
        kind: ObjectKind,
        bytes: &[u8],
        expected: &Digest,
    ) -> Result<Digest, StoreError> {
        let actual = kind.digest(bytes);
        if actual != *expected {
            return Err(StoreError::Mismatch {
                expected: *expected,
                actual,
            });
        }
        self.publish(kind, bytes)
    }
}
