use crate::{
    BranchRef, CommandBatch, CommandError, IdempotencyKey, ProjectId, ProjectState, Revision,
    RevisionId, SignedBranch, SignedRevision,
};
use core::fmt;
use euphony_daw_identity::{AuthorKey, DecodeError, Decoder, Digest, Encoder, SignatureError};
use euphony_daw_store::{Journal, JournalError};
use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};

const OPENED: u8 = 0;
const COMMIT: u8 = 1;
const PIN: u8 = 2;
const BATCH_DOMAIN: &str = "euphony batch v1";

/// A committed batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Committed {
    pub revision: RevisionId,
    /// Whether an earlier submission of the batch's key committed it.
    pub replayed: bool,
}

/// Why a batch wasn't committed.
#[derive(Debug)]
pub enum SubmitError {
    /// The batch names another project.
    WrongProject,
    /// The batch expected another revision than the head.
    Conflict { current: Option<RevisionId> },
    /// The batch's key committed a different batch.
    KeyReused { revision: RevisionId },
    /// A command doesn't apply; the batch changed nothing.
    Invalid { index: usize, error: CommandError },
    /// The journal refused the commit; the batch changed nothing.
    Unavailable(JournalError),
}

impl fmt::Display for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongProject => f.write_str("the batch names another project"),
            Self::Conflict { .. } => f.write_str("the batch expected another revision"),
            Self::KeyReused { .. } => f.write_str("the key committed another batch"),
            Self::Invalid { index, error } => write!(f, "command {index}: {error}"),
            Self::Unavailable(error) => write!(f, "the journal is unavailable: {error}"),
        }
    }
}

impl std::error::Error for SubmitError {}

/// Why the authority's journal didn't open or record.
#[derive(Debug)]
pub enum AuthorityError {
    Journal(JournalError),
    /// A record didn't decode.
    Decode(DecodeError),
    /// A recorded revision's signature didn't verify.
    Signature(SignatureError),
    /// A recorded revision doesn't follow from its batch and its parent.
    Diverged { record: usize },
    /// A pin names a revision the journal doesn't hold.
    UnknownRevision(RevisionId),
}

impl fmt::Display for AuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Journal(error) => write!(f, "{error}"),
            Self::Decode(error) => write!(f, "a journal record didn't decode: {error}"),
            Self::Signature(error) => write!(f, "a journal revision didn't verify: {error}"),
            Self::Diverged { record } => write!(f, "journal record {record} doesn't replay"),
            Self::UnknownRevision(revision) => write!(f, "revision {} isn't held", revision.0),
        }
    }
}

impl std::error::Error for AuthorityError {}

impl From<JournalError> for AuthorityError {
    fn from(error: JournalError) -> Self {
        Self::Journal(error)
    }
}

impl From<DecodeError> for AuthorityError {
    fn from(error: DecodeError) -> Self {
        Self::Decode(error)
    }
}

/// The authority of one working branch: it orders the branch's edits and
/// keeps them, with the render manifests pinned to them, in its journal.
#[derive(Debug)]
pub struct BranchAuthority {
    journal: Journal,
    key: AuthorKey,
    project: ProjectId,
    name: String,
    epoch: u64,
    state: ProjectState,
    head: Option<RevisionId>,
    sequence: u64,
    revisions: BTreeMap<RevisionId, SignedRevision>,
    keys: HashMap<IdempotencyKey, (Digest, RevisionId)>,
    pins: BTreeMap<RevisionId, Digest>,
}

impl BranchAuthority {
    /// Opens the branch's journal at `path`, replays and verifies every
    /// record, and starts a new authority epoch.
    pub fn open(
        path: &Path,
        key: AuthorKey,
        project: ProjectId,
        name: &str,
    ) -> Result<Self, AuthorityError> {
        let (journal, records) = Journal::open(path)?;
        let mut authority = Self {
            journal,
            key,
            project,
            name: name.to_string(),
            epoch: 0,
            state: ProjectState::default(),
            head: None,
            sequence: 0,
            revisions: BTreeMap::new(),
            keys: HashMap::new(),
            pins: BTreeMap::new(),
        };
        for (index, record) in records.iter().enumerate() {
            authority.replay(index, record)?;
        }
        authority.epoch += 1;
        authority.journal.append(&[OPENED])?;
        Ok(authority)
    }

    fn replay(&mut self, index: usize, record: &[u8]) -> Result<(), AuthorityError> {
        let mut decoder = Decoder::new(record);
        match decoder.u8()? {
            OPENED => self.epoch += 1,
            COMMIT => {
                let batch_bytes = decoder.bytes()?;
                let batch = CommandBatch::decode(&batch_bytes)?;
                let signed = SignedRevision::decode(&decoder.bytes()?)?;
                let id = signed.verify().map_err(AuthorityError::Signature)?;
                let diverged = AuthorityError::Diverged { record: index };
                let mut state = self.state.clone();
                for command in &batch.commands {
                    if state.apply(command).is_err() {
                        return Err(diverged);
                    }
                }
                let parents: Vec<_> = self.head.into_iter().collect();
                if signed.revision.parents != parents || signed.revision.root != state.root() {
                    return Err(diverged);
                }
                let batch_digest = Digest::of(BATCH_DOMAIN, &batch_bytes);
                self.accept(state, id, signed, batch.key, batch_digest);
            }
            PIN => {
                let revision = RevisionId(decoder.digest()?);
                let manifest = decoder.digest()?;
                if !self.revisions.contains_key(&revision) {
                    return Err(AuthorityError::UnknownRevision(revision));
                }
                self.pins.insert(revision, manifest);
            }
            _ => return Err(DecodeError::Invalid("journal record").into()),
        }
        decoder.finish()?;
        Ok(())
    }

    fn accept(
        &mut self,
        state: ProjectState,
        id: RevisionId,
        signed: SignedRevision,
        key: IdempotencyKey,
        batch: Digest,
    ) {
        self.state = state;
        self.head = Some(id);
        self.sequence += 1;
        self.revisions.insert(id, signed);
        self.keys.insert(key, (batch, id));
    }

    /// Validates and commits a batch, appending it to the journal before
    /// answering.
    pub fn submit(&mut self, batch: &CommandBatch) -> Result<Committed, SubmitError> {
        if batch.project != self.project {
            return Err(SubmitError::WrongProject);
        }
        let batch_bytes = batch.canonical_bytes();
        let batch_digest = Digest::of(BATCH_DOMAIN, &batch_bytes);
        if let Some((digest, revision)) = self.keys.get(&batch.key) {
            if *digest != batch_digest {
                return Err(SubmitError::KeyReused {
                    revision: *revision,
                });
            }
            return Ok(Committed {
                revision: *revision,
                replayed: true,
            });
        }
        if batch.expected != self.head {
            return Err(SubmitError::Conflict { current: self.head });
        }
        let mut state = self.state.clone();
        for (index, command) in batch.commands.iter().enumerate() {
            if let Err(error) = state.apply(command) {
                return Err(SubmitError::Invalid { index, error });
            }
        }
        let revision = Revision {
            parents: self.head.into_iter().collect(),
            root: state.root(),
            schema_version: Revision::SCHEMA_VERSION,
            author: self.key.author(),
            message: batch.message.clone(),
        };
        let id = revision.id();
        let signed = revision.sign(&self.key);
        let signed = signed.expect("the authority signs its own revisions");
        let mut record = Encoder::new();
        record.u8(COMMIT).bytes(&batch_bytes);
        record.bytes(&signed.canonical_bytes());
        let appended = self.journal.append(&record.finish());
        appended.map_err(SubmitError::Unavailable)?;
        self.accept(state, id, signed, batch.key, batch_digest);
        Ok(Committed {
            revision: id,
            replayed: false,
        })
    }

    /// Pins a render manifest to a revision this branch holds.
    pub fn pin(&mut self, revision: RevisionId, manifest: Digest) -> Result<(), AuthorityError> {
        if !self.revisions.contains_key(&revision) {
            return Err(AuthorityError::UnknownRevision(revision));
        }
        let mut record = Encoder::new();
        record.u8(PIN).digest(&revision.0).digest(&manifest);
        self.journal.append(&record.finish())?;
        self.pins.insert(revision, manifest);
        Ok(())
    }

    pub fn pinned(&self, revision: &RevisionId) -> Option<Digest> {
        self.pins.get(revision).copied()
    }

    pub fn head(&self) -> Option<RevisionId> {
        self.head
    }

    pub fn state(&self) -> &ProjectState {
        &self.state
    }

    pub fn revision(&self, id: &RevisionId) -> Option<&SignedRevision> {
        self.revisions.get(id)
    }

    /// The authority epoch: it rises with each open, so a result an earlier
    /// epoch assigned is recognized as stale.
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn project(&self) -> ProjectId {
        self.project
    }

    /// The branch reference to the head, signed by the branch's owner.
    pub fn branch(&self) -> SignedBranch {
        let reference = BranchRef {
            owner: self.key.author(),
            name: self.name.clone(),
            head: self.head,
            sequence: self.sequence,
        };
        let signed = reference.sign(&self.key);
        signed.expect("the authority signs its own branch")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Command, RegionId, TrackId};
    use euphony_daw_identity::AuthorId;

    fn key() -> AuthorKey {
        AuthorKey::from_seed([9; 32])
    }

    fn project() -> ProjectId {
        ProjectId(Digest::of("euphony test", b"project"))
    }

    fn open(path: &Path) -> BranchAuthority {
        BranchAuthority::open(path, key(), project(), "main").unwrap()
    }

    fn batch(
        expected: Option<RevisionId>,
        key: u8,
        actor: AuthorId,
        commands: Vec<Command>,
    ) -> CommandBatch {
        CommandBatch {
            project: project(),
            expected,
            key: IdempotencyKey([key; 16]),
            actor,
            message: format!("batch {key}"),
            commands,
        }
    }

    fn add_track(id: u64) -> Command {
        Command::AddTrack {
            track: TrackId(id),
            name: format!("track {id}"),
        }
    }

    #[test]
    fn commits_in_order_and_reopens_to_the_same_head() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("branch");
        let actor = key().author();
        let (head, root, manifest) = {
            let mut authority = open(&path);
            assert_eq!(authority.epoch(), 1);
            let first = authority.submit(&batch(None, 1, actor, vec![add_track(1)]));
            let first = first.unwrap();
            let region = Command::AddRegion {
                region: RegionId(1),
                track: TrackId(1),
                asset: Digest::of("euphony test", b"asset"),
                start: 0,
            };
            let second = batch(Some(first.revision), 2, actor, vec![region]);
            let second = authority.submit(&second).unwrap();
            let manifest = Digest::of("euphony test", b"manifest");
            authority.pin(second.revision, manifest).unwrap();
            (second.revision, authority.state().root(), manifest)
        };

        let authority = open(&path);
        assert_eq!(authority.epoch(), 2);
        assert_eq!(authority.head(), Some(head));
        assert_eq!(authority.state().root(), root);
        assert_eq!(authority.pinned(&head), Some(manifest));
        let revision = authority.revision(&head).unwrap();
        assert_eq!(revision.verify(), Ok(head));
        assert_eq!(authority.branch().verify(), Ok(()));
        assert_eq!(authority.branch().branch.head, Some(head));
    }

    #[test]
    fn a_repeated_key_commits_once() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("branch");
        let actor = key().author();
        let mut authority = open(&path);
        let submitted = batch(None, 1, actor, vec![add_track(1)]);
        let first = authority.submit(&submitted).unwrap();
        let again = authority.submit(&submitted).unwrap();
        assert!(!first.replayed);
        assert!(again.replayed);
        assert_eq!(first.revision, again.revision);

        let reused = batch(Some(first.revision), 1, actor, vec![add_track(2)]);
        let reused = authority.submit(&reused);
        assert!(matches!(reused, Err(SubmitError::KeyReused { .. })));
    }

    #[test]
    fn a_stale_expected_revision_is_a_conflict() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("branch");
        let actor = key().author();
        let mut authority = open(&path);
        let first = authority.submit(&batch(None, 1, actor, vec![add_track(1)]));
        let first = first.unwrap();
        let stale = authority.submit(&batch(None, 2, actor, vec![add_track(2)]));
        match stale {
            Err(SubmitError::Conflict { current }) => assert_eq!(current, Some(first.revision)),
            other => panic!("expected a conflict, found {other:?}"),
        }
    }

    #[test]
    fn an_invalid_command_changes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("branch");
        let actor = key().author();
        let mut authority = open(&path);
        let commands = vec![add_track(1), add_track(1)];
        let result = authority.submit(&batch(None, 1, actor, commands));
        match result {
            Err(SubmitError::Invalid { index, error }) => {
                assert_eq!(index, 1);
                assert_eq!(error, CommandError::TrackExists(TrackId(1)));
            }
            other => panic!("expected an invalid command, found {other:?}"),
        }
        assert_eq!(authority.head(), None);
        assert!(authority.state().tracks.is_empty());
    }

    #[test]
    fn a_damaged_record_refuses_the_open() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("branch");
        let actor = key().author();
        {
            let mut authority = open(&path);
            let first = batch(None, 1, actor, vec![add_track(1)]);
            authority.submit(&first).unwrap();
        }
        // The first record is the open marker: its one payload byte follows
        // its eight length bytes.
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[8] ^= 1;
        std::fs::write(&path, bytes).unwrap();
        let reopened = BranchAuthority::open(&path, key(), project(), "main");
        assert!(reopened.is_err());
    }
}
