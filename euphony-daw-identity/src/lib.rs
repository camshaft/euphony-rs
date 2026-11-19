//! Content digests, canonical encoding and signed identities.
//!
//! Every identity in the workstation is a domain-separated digest of a
//! canonical encoding: two records have one identity exactly when their
//! canonical bytes are equal, and a digest of one kind of record can't be
//! taken for a digest of another kind. Each encoded digest names the hash
//! function that computed it in its first byte, so a later hash function
//! doesn't change what an earlier identity means.

mod canonical;
mod digest;
mod sign;

pub use canonical::{DecodeError, Decoder, EncodeError, Encoder};
pub use digest::{Algorithm, Digest};
pub use sign::{AuthorId, AuthorKey, Signature, SignatureError};
