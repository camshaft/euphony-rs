use core::fmt;
use euphony_daw_identity::{
    AuthorId, AuthorKey, DecodeError, Decoder, Digest, Encoder, Signature, SignatureError,
};
use std::collections::BTreeMap;

const REVISION_DOMAIN: &str = "euphony revision v1";
const REVISION_SIGNATURE: &str = "euphony revision signature v1";
const BRANCH_DOMAIN: &str = "euphony branch head v1";
const BRANCH_SIGNATURE: &str = "euphony branch signature v1";

/// The identity of a revision: the digest of its canonical unsigned fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RevisionId(pub Digest);

/// An immutable revision of a project.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Revision {
    pub parents: Vec<RevisionId>,
    /// The digest of the project's canonical state at this revision.
    pub root: Digest,
    pub schema_version: u32,
    pub author: AuthorId,
    pub message: String,
}

impl Revision {
    /// The schema version of the project state this build writes.
    pub const SCHEMA_VERSION: u32 = 1;

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder.u64(self.parents.len() as u64);
        for parent in &self.parents {
            encoder.digest(&parent.0);
        }
        encoder
            .digest(&self.root)
            .u32(self.schema_version)
            .raw(self.author.as_bytes())
            .str(&self.message);
        encoder.finish()
    }

    pub fn id(&self) -> RevisionId {
        RevisionId(Digest::of(REVISION_DOMAIN, &self.canonical_bytes()))
    }

    /// Signs the revision, where `key` is its author's.
    pub fn sign(self, key: &AuthorKey) -> Option<SignedRevision> {
        if key.author() != self.author {
            return None;
        }
        let signature = key.sign(REVISION_SIGNATURE, &self.id().0.encoded());
        Some(SignedRevision {
            revision: self,
            signature,
        })
    }
}

/// A revision with its author's signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedRevision {
    pub revision: Revision,
    pub signature: Signature,
}

impl SignedRevision {
    /// Checks the author's signature and returns the revision's identity.
    pub fn verify(&self) -> Result<RevisionId, SignatureError> {
        let id = self.revision.id();
        let author = self.revision.author;
        author.verify(REVISION_SIGNATURE, &id.0.encoded(), &self.signature)?;
        Ok(id)
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = self.revision.canonical_bytes();
        bytes.extend_from_slice(self.signature.as_bytes());
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut decoder = Decoder::new(bytes);
        let count = decoder.count(Digest::ENCODED_LEN)?;
        let mut parents = Vec::with_capacity(count);
        for _ in 0..count {
            parents.push(RevisionId(decoder.digest()?));
        }
        let root = decoder.digest()?;
        let schema_version = decoder.u32()?;
        let author = AuthorId::from_bytes(decoder.array()?);
        let message = decoder.str()?.to_string();
        let signature = Signature::from_bytes(decoder.array()?);
        decoder.finish()?;
        let revision = Revision {
            parents,
            root,
            schema_version,
            author,
            message,
        };
        Ok(Self {
            revision,
            signature,
        })
    }
}

/// The identity of a branch head record: the digest of its canonical
/// unsigned fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BranchHeadId(pub Digest);

/// A branch head record: its owner's reference to a revision head.
///
/// Each record names the record it replaces, so the records of one branch
/// form a chain that a replayed or skipped record can't join.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BranchRef {
    pub owner: AuthorId,
    pub name: String,
    pub head: Option<RevisionId>,
    /// Rises with each move of the head, so an older record can't replace
    /// a newer one.
    pub sequence: u64,
    /// The record this one replaces, or `None` for the branch's first record.
    pub previous: Option<BranchHeadId>,
}

impl BranchRef {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder.raw(self.owner.as_bytes()).str(&self.name);
        match &self.head {
            Some(head) => encoder.bool(true).digest(&head.0),
            None => encoder.bool(false),
        };
        encoder.u64(self.sequence);
        match &self.previous {
            Some(previous) => encoder.bool(true).digest(&previous.0),
            None => encoder.bool(false),
        };
        encoder.finish()
    }

    pub fn id(&self) -> BranchHeadId {
        BranchHeadId(Digest::of(BRANCH_DOMAIN, &self.canonical_bytes()))
    }

    /// Signs the record, where `key` is its owner's.
    pub fn sign(self, key: &AuthorKey) -> Option<SignedBranch> {
        if key.author() != self.owner {
            return None;
        }
        let signature = key.sign(BRANCH_SIGNATURE, &self.canonical_bytes());
        Some(SignedBranch {
            branch: self,
            signature,
        })
    }
}

/// A branch head record with its owner's signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedBranch {
    pub branch: BranchRef,
    pub signature: Signature,
}

impl SignedBranch {
    /// Checks that the branch's owner signed this record.
    pub fn verify(&self) -> Result<(), SignatureError> {
        let bytes = self.branch.canonical_bytes();
        let owner = self.branch.owner;
        owner.verify(BRANCH_SIGNATURE, &bytes, &self.signature)
    }

    pub fn id(&self) -> BranchHeadId {
        self.branch.id()
    }
}

/// Two records of one branch that its owner signed at one sequence number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BranchConflict {
    pub sequence: u64,
    /// The record accepted at that sequence number.
    pub accepted: SignedBranch,
    /// The other record offered at it.
    pub offered: SignedBranch,
}

/// Why a branch head record wasn't accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BranchError {
    /// The owner's signature doesn't verify.
    Signature(SignatureError),
    /// The record is of another owner's branch, or of another name.
    WrongBranch,
    /// The record was accepted already, or is older than the head.
    Replayed { sequence: u64, current: u64 },
    /// The record's sequence number is higher, but it doesn't name the
    /// current record as the one it replaces.
    Unlinked {
        expected: BranchHeadId,
        named: Option<BranchHeadId>,
    },
    /// The owner signed another record at the same sequence number; the
    /// branch keeps both as a conflict.
    Conflict(Box<BranchConflict>),
}

impl fmt::Display for BranchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signature(error) => write!(f, "the branch record didn't verify: {error}"),
            Self::WrongBranch => f.write_str("the record is of another branch"),
            Self::Replayed { sequence, current } => {
                write!(f, "record {sequence} isn't newer than the head at {current}")
            }
            Self::Unlinked { .. } => f.write_str("the record doesn't name the current head"),
            Self::Conflict(conflict) => write!(
                f,
                "the owner signed two records at sequence {}",
                conflict.sequence
            ),
        }
    }
}

impl std::error::Error for BranchError {}

/// The head records of one branch that a reader has accepted.
///
/// A record replaces the head only where its sequence number is higher and
/// it names the head as the record it replaces. A second record at an
/// accepted sequence number is kept as a [`BranchConflict`].
#[derive(Clone, Debug)]
pub struct BranchHeads {
    current: SignedBranch,
    accepted: BTreeMap<u64, SignedBranch>,
    conflicts: Vec<BranchConflict>,
}

impl BranchHeads {
    /// Starts from `record`, the first record of the branch this reader
    /// takes, once its owner's signature verifies.
    pub fn new(record: SignedBranch) -> Result<Self, BranchError> {
        record.verify().map_err(BranchError::Signature)?;
        let mut accepted = BTreeMap::new();
        accepted.insert(record.branch.sequence, record.clone());
        Ok(Self {
            current: record,
            accepted,
            conflicts: Vec::new(),
        })
    }

    /// The head record.
    pub fn current(&self) -> &SignedBranch {
        &self.current
    }

    /// Each pair of records the owner signed at one sequence number.
    pub fn conflicts(&self) -> &[BranchConflict] {
        &self.conflicts
    }

    /// Offers a record of the branch, and makes it the head where it follows
    /// the head.
    pub fn offer(&mut self, record: SignedBranch) -> Result<(), BranchError> {
        let head = &self.current.branch;
        let offered = &record.branch;
        if offered.owner != head.owner || offered.name != head.name {
            return Err(BranchError::WrongBranch);
        }
        record.verify().map_err(BranchError::Signature)?;
        let sequence = offered.sequence;
        let current = head.sequence;
        if let Some(accepted) = self.accepted.get(&sequence) {
            if accepted.id() == record.id() {
                return Err(BranchError::Replayed { sequence, current });
            }
            let conflict = BranchConflict {
                sequence,
                accepted: accepted.clone(),
                offered: record,
            };
            if !self.conflicts.contains(&conflict) {
                self.conflicts.push(conflict.clone());
            }
            return Err(BranchError::Conflict(Box::new(conflict)));
        }
        if sequence <= current {
            return Err(BranchError::Replayed { sequence, current });
        }
        let expected = self.current.id();
        if offered.previous != Some(expected) {
            return Err(BranchError::Unlinked {
                expected,
                named: offered.previous,
            });
        }
        self.accepted.insert(sequence, record.clone());
        self.current = record;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn revision(author: AuthorId) -> Revision {
        Revision {
            parents: vec![RevisionId(Digest::of("euphony test", b"parent"))],
            root: Digest::of("euphony test", b"root"),
            schema_version: Revision::SCHEMA_VERSION,
            author,
            message: "add a track".to_string(),
        }
    }

    fn owner() -> AuthorKey {
        AuthorKey::from_seed([5; 32])
    }

    fn head(label: &[u8]) -> Option<RevisionId> {
        Some(RevisionId(Digest::of("euphony test", label)))
    }

    /// The record after `previous`, at the next sequence number.
    fn next(previous: &SignedBranch, label: &[u8]) -> SignedBranch {
        let record = BranchRef {
            head: head(label),
            sequence: previous.branch.sequence + 1,
            previous: Some(previous.id()),
            ..previous.branch.clone()
        };
        record.sign(&owner()).unwrap()
    }

    fn first() -> SignedBranch {
        let record = BranchRef {
            owner: owner().author(),
            name: "main".to_string(),
            head: None,
            sequence: 0,
            previous: None,
        };
        record.sign(&owner()).unwrap()
    }

    #[test]
    fn signed_revisions_round_trip_and_verify() {
        let key = AuthorKey::from_seed([3; 32]);
        let signed = revision(key.author()).sign(&key).unwrap();
        let decoded = SignedRevision::decode(&signed.canonical_bytes()).unwrap();
        assert_eq!(decoded, signed);
        assert_eq!(decoded.verify(), Ok(signed.revision.id()));
    }

    #[test]
    fn a_revision_with_an_unknown_digest_algorithm_doesnt_decode() {
        let key = AuthorKey::from_seed([3; 32]);
        let signed = revision(key.author()).sign(&key).unwrap();
        let mut bytes = signed.canonical_bytes();
        // The parent count's eight bytes come first, then the first parent's
        // algorithm byte.
        assert_eq!(bytes[8], 1);
        bytes[8] = 2;
        let refused = SignedRevision::decode(&bytes);
        assert_eq!(refused, Err(DecodeError::Invalid("digest algorithm")));
    }

    #[test]
    fn a_changed_field_breaks_the_signature() {
        let key = AuthorKey::from_seed([3; 32]);
        let mut signed = revision(key.author()).sign(&key).unwrap();
        signed.revision.root = Digest::of("euphony test", b"other root");
        assert_eq!(signed.verify(), Err(SignatureError::Mismatch));
    }

    #[test]
    fn only_the_author_signs_a_revision() {
        let key = AuthorKey::from_seed([3; 32]);
        let other = AuthorKey::from_seed([4; 32]);
        assert!(revision(key.author()).sign(&other).is_none());
    }

    #[test]
    fn an_equal_branch_name_doesnt_give_control_of_another_owners_branch() {
        let owner = owner();
        let other = AuthorKey::from_seed([6; 32]);
        let branch = first().branch;
        assert!(branch.clone().sign(&other).is_none());

        let signed = branch.clone().sign(&owner).unwrap();
        assert_eq!(signed.verify(), Ok(()));

        let forged = SignedBranch {
            branch: BranchRef {
                owner: owner.author(),
                ..branch.clone()
            },
            signature: other.sign(BRANCH_SIGNATURE, &branch.canonical_bytes()),
        };
        assert_eq!(forged.verify(), Err(SignatureError::Mismatch));

        let mut moved = signed;
        moved.branch.head = head(b"head");
        assert_eq!(moved.verify(), Err(SignatureError::Mismatch));
    }

    #[test]
    fn the_record_a_branch_head_replaces_is_part_of_its_identity() {
        let one = next(&first(), b"one");
        let mut relinked = one.branch.clone();
        relinked.previous = None;
        assert_ne!(relinked.id(), one.id());
        let mut moved = one.clone();
        moved.branch.previous = Some(BranchHeadId(Digest::of("euphony test", b"other")));
        assert_eq!(moved.verify(), Err(SignatureError::Mismatch));
    }

    #[test]
    fn a_record_that_follows_the_head_replaces_it() {
        let first = first();
        let mut heads = BranchHeads::new(first.clone()).unwrap();
        let one = next(&first, b"one");
        let two = next(&one, b"two");
        assert_eq!(heads.offer(one.clone()), Ok(()));
        assert_eq!(heads.offer(two.clone()), Ok(()));
        assert_eq!(heads.current(), &two);
        assert!(heads.conflicts().is_empty());
    }

    #[test]
    fn a_replayed_record_is_refused() {
        let first = first();
        let one = next(&first, b"one");
        let two = next(&one, b"two");
        let mut heads = BranchHeads::new(first.clone()).unwrap();
        heads.offer(one.clone()).unwrap();
        heads.offer(two.clone()).unwrap();
        for (old, sequence) in [(two.clone(), 2), (one.clone(), 1), (first, 0)] {
            let refused = heads.offer(old);
            let expected = BranchError::Replayed {
                sequence,
                current: 2,
            };
            assert_eq!(refused, Err(expected));
        }
        assert_eq!(heads.current(), &two);
        assert!(heads.conflicts().is_empty());

        // A record older than the first one this reader took is refused too.
        let mut late = BranchHeads::new(two.clone()).unwrap();
        let refused = late.offer(one);
        let expected = BranchError::Replayed {
            sequence: 1,
            current: 2,
        };
        assert_eq!(refused, Err(expected));
        assert_eq!(late.current(), &two);
    }

    #[test]
    fn a_record_that_skips_the_previous_one_is_refused() {
        let first = first();
        let one = next(&first, b"one");
        let two = next(&one, b"two");
        let mut heads = BranchHeads::new(first.clone()).unwrap();
        let refused = heads.offer(two);
        let expected = BranchError::Unlinked {
            expected: first.id(),
            named: Some(one.id()),
        };
        assert_eq!(refused, Err(expected));

        let unlinked = BranchRef {
            previous: None,
            ..one.branch.clone()
        };
        let refused = heads.offer(unlinked.sign(&owner()).unwrap());
        let expected = BranchError::Unlinked {
            expected: first.id(),
            named: None,
        };
        assert_eq!(refused, Err(expected));
        assert_eq!(heads.current(), &first);

        assert_eq!(heads.offer(one.clone()), Ok(()));
        assert_eq!(heads.current(), &one);
    }

    #[test]
    fn two_records_at_one_sequence_number_are_a_conflict() {
        let first = first();
        let one = next(&first, b"one");
        let other = next(&first, b"other one");
        assert_eq!(one.branch.sequence, other.branch.sequence);
        let mut heads = BranchHeads::new(first.clone()).unwrap();
        heads.offer(one.clone()).unwrap();

        let refused = heads.offer(other.clone());
        let conflict = BranchConflict {
            sequence: 1,
            accepted: one.clone(),
            offered: other.clone(),
        };
        assert_eq!(refused, Err(BranchError::Conflict(Box::new(conflict.clone()))));
        assert_eq!(heads.conflicts(), &[conflict.clone()]);
        assert_eq!(heads.current(), &one);

        // Offering the same pair again keeps one conflict.
        let again = heads.offer(other);
        assert!(matches!(again, Err(BranchError::Conflict(_))));
        assert_eq!(heads.conflicts().len(), 1);

        // A second record at an older accepted number is a conflict too.
        let two = next(&one, b"two");
        heads.offer(two).unwrap();
        let rewritten = BranchRef {
            head: head(b"rewritten"),
            ..first.branch.clone()
        };
        let refused = heads.offer(rewritten.sign(&owner()).unwrap());
        assert!(matches!(refused, Err(BranchError::Conflict(_))));
        assert_eq!(heads.conflicts().len(), 2);
        assert_eq!(heads.conflicts()[1].sequence, 0);
    }

    #[test]
    fn another_branchs_or_a_forged_record_is_refused() {
        let first = first();
        let mut heads = BranchHeads::new(first.clone()).unwrap();
        let renamed = BranchRef {
            name: "other".to_string(),
            ..next(&first, b"one").branch
        };
        let refused = heads.offer(renamed.sign(&owner()).unwrap());
        assert_eq!(refused, Err(BranchError::WrongBranch));

        let mut forged = next(&first, b"one");
        forged.branch.head = head(b"forged");
        let refused = heads.offer(forged.clone());
        let expected = BranchError::Signature(SignatureError::Mismatch);
        assert_eq!(refused, Err(expected.clone()));
        assert_eq!(BranchHeads::new(forged).err(), Some(expected));
        assert_eq!(heads.current(), &first);
    }
}
