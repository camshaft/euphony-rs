use crate::{ProjectId, RegionId, RevisionId, TrackId};
use core::fmt;
use euphony_daw_identity::{AuthorId, DecodeError, Decoder, Digest, Encoder};

/// One typed change to a project.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    AddTrack { track: TrackId, name: String },
    SetTrackGain { track: TrackId, gain: f32 },
    AddRegion {
        region: RegionId,
        track: TrackId,
        asset: Digest,
        start: u64,
    },
    RemoveRegion { region: RegionId },
}

impl Command {
    pub fn encode(&self, encoder: &mut Encoder) {
        match self {
            Self::AddTrack { track, name } => {
                encoder.u8(0).u64(track.0).str(name);
            }
            Self::SetTrackGain { track, gain } => {
                encoder.u8(1).u64(track.0).u32(gain.to_bits());
            }
            Self::AddRegion {
                region,
                track,
                asset,
                start,
            } => {
                encoder.u8(2).u64(region.0).u64(track.0);
                encoder.digest(asset).u64(*start);
            }
            Self::RemoveRegion { region } => {
                encoder.u8(3).u64(region.0);
            }
        }
    }

    pub fn decode(decoder: &mut Decoder<'_>) -> Result<Self, DecodeError> {
        let command = match decoder.u8()? {
            0 => Self::AddTrack {
                track: TrackId(decoder.u64()?),
                name: decoder.str()?.to_string(),
            },
            1 => Self::SetTrackGain {
                track: TrackId(decoder.u64()?),
                gain: f32::from_bits(decoder.u32()?),
            },
            2 => Self::AddRegion {
                region: RegionId(decoder.u64()?),
                track: TrackId(decoder.u64()?),
                asset: decoder.digest()?,
                start: decoder.u64()?,
            },
            3 => Self::RemoveRegion {
                region: RegionId(decoder.u64()?),
            },
            _ => return Err(DecodeError::Invalid("command")),
        };
        Ok(command)
    }
}

/// Why a command doesn't apply to the project as it stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandError {
    TrackExists(TrackId),
    UnknownTrack(TrackId),
    RegionExists(RegionId),
    UnknownRegion(RegionId),
    NonFiniteGain(TrackId),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrackExists(track) => write!(f, "track {} exists already", track.0),
            Self::UnknownTrack(track) => write!(f, "track {} doesn't exist", track.0),
            Self::RegionExists(region) => write!(f, "region {} exists already", region.0),
            Self::UnknownRegion(region) => write!(f, "region {} doesn't exist", region.0),
            Self::NonFiniteGain(track) => write!(f, "track {} gain isn't finite", track.0),
        }
    }
}

impl std::error::Error for CommandError {}

/// A key that makes a repeated submission of one batch commit it once.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdempotencyKey(pub [u8; 16]);

/// A batch of commands an actor submits against an expected revision.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandBatch {
    pub project: ProjectId,
    /// The revision the actor saw, or `None` for an empty branch.
    pub expected: Option<RevisionId>,
    pub key: IdempotencyKey,
    pub actor: AuthorId,
    pub message: String,
    pub commands: Vec<Command>,
}

impl CommandBatch {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder.digest(&self.project.0);
        match &self.expected {
            Some(expected) => encoder.bool(true).digest(&expected.0),
            None => encoder.bool(false),
        };
        encoder
            .raw(&self.key.0)
            .raw(self.actor.as_bytes())
            .str(&self.message)
            .u64(self.commands.len() as u64);
        for command in &self.commands {
            command.encode(&mut encoder);
        }
        encoder.finish()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut decoder = Decoder::new(bytes);
        let project = ProjectId(decoder.digest()?);
        let expected = match decoder.bool()? {
            true => Some(RevisionId(decoder.digest()?)),
            false => None,
        };
        let key = IdempotencyKey(decoder.array()?);
        let actor = AuthorId::from_bytes(decoder.array()?);
        let message = decoder.str()?.to_string();
        let count = decoder.count(9)?;
        let mut commands = Vec::with_capacity(count);
        for _ in 0..count {
            commands.push(Command::decode(&mut decoder)?);
        }
        decoder.finish()?;
        Ok(Self {
            project,
            expected,
            key,
            actor,
            message,
            commands,
        })
    }
}
