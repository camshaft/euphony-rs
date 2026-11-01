use euphony_daw_identity::{
    AuthorId, AuthorKey, DecodeError, Decoder, Digest, Encoder, Signature, SignatureError,
};

const REVISION_DOMAIN: &str = "euphony revision v1";
const REVISION_SIGNATURE: &str = "euphony revision signature v1";
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
        let signature = key.sign(REVISION_SIGNATURE, self.id().0.as_bytes());
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
        author.verify(REVISION_SIGNATURE, id.0.as_bytes(), &self.signature)?;
        Ok(id)
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = self.revision.canonical_bytes();
        bytes.extend_from_slice(self.signature.as_bytes());
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut decoder = Decoder::new(bytes);
        let count = decoder.count(32)?;
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

/// A branch: its owner's reference to a revision head.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BranchRef {
    pub owner: AuthorId,
    pub name: String,
    pub head: Option<RevisionId>,
    /// Rises with each move of the head, so an older reference can't replace
    /// a newer one.
    pub sequence: u64,
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
        encoder.finish()
    }

    /// Signs the reference, where `key` is its owner's.
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

/// A branch reference with its owner's signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedBranch {
    pub branch: BranchRef,
    pub signature: Signature,
}

impl SignedBranch {
    /// Checks that the branch's owner signed this reference.
    pub fn verify(&self) -> Result<(), SignatureError> {
        let bytes = self.branch.canonical_bytes();
        let owner = self.branch.owner;
        owner.verify(BRANCH_SIGNATURE, &bytes, &self.signature)
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

    #[test]
    fn signed_revisions_round_trip_and_verify() {
        let key = AuthorKey::from_seed([3; 32]);
        let signed = revision(key.author()).sign(&key).unwrap();
        let decoded = SignedRevision::decode(&signed.canonical_bytes()).unwrap();
        assert_eq!(decoded, signed);
        assert_eq!(decoded.verify(), Ok(signed.revision.id()));
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
        let owner = AuthorKey::from_seed([5; 32]);
        let other = AuthorKey::from_seed([6; 32]);
        let branch = BranchRef {
            owner: owner.author(),
            name: "main".to_string(),
            head: None,
            sequence: 0,
        };
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
        moved.branch.head = Some(RevisionId(Digest::of("euphony test", b"head")));
        assert_eq!(moved.verify(), Err(SignatureError::Mismatch));
    }
}
