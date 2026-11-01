//! Content digests, canonical encoding and signed identities.
//!
//! Every identity in the workstation is a domain-separated digest of a
//! canonical encoding: two records have one identity exactly when their
//! canonical bytes are equal, and a digest of one kind of record can't be
//! taken for a digest of another kind.

mod canonical;
mod digest;
mod sign;

pub use canonical::{DecodeError, Decoder, EncodeError, Encoder};
pub use digest::Digest;
pub use sign::{AuthorId, AuthorKey, Signature, SignatureError};
