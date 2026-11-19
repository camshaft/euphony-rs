//! Typed commands, signed revisions and the branch authority.
//!
//! A project changes only through typed command batches. The authority of a
//! working branch orders those batches, commits each one as a revision its
//! owner signs, and keeps them in a journal it replays on reopen.

mod authority;
mod command;
mod revision;
mod state;

pub use authority::{AuthorityError, BranchAuthority, Committed, SubmitError};
pub use command::{Command, CommandBatch, CommandError, IdempotencyKey};
pub use revision::{
    BranchConflict, BranchError, BranchHeadId, BranchHeads, BranchRef, Revision, RevisionId,
    SignedBranch, SignedRevision,
};
pub use state::{ProjectId, ProjectState, Region, RegionId, Track, TrackId};
