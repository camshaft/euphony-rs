use crate::{Command, CommandError};
use euphony_daw_identity::{Digest, Encoder};
use std::collections::BTreeMap;

/// The identity of a project.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProjectId(pub Digest);

/// The stable identity of a track.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrackId(pub u64);

/// The stable identity of a region.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionId(pub u64);

/// A region: an immutable asset placed at a frame of the timeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    /// The digest of the asset's canonical working audio.
    pub asset: Digest,
    /// The timeline frame the asset's first frame plays at.
    pub start: u64,
}

/// A track: a gain and the regions it plays.
#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    pub name: String,
    pub gain: f32,
    pub regions: BTreeMap<RegionId, Region>,
}

/// The state of a project at one revision.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectState {
    pub tracks: BTreeMap<TrackId, Track>,
}

impl ProjectState {
    const DOMAIN: &'static str = "euphony project root v1";

    /// Applies one command, or changes nothing where it is invalid.
    pub fn apply(&mut self, command: &Command) -> Result<(), CommandError> {
        match command {
            Command::AddTrack { track, name } => {
                if self.tracks.contains_key(track) {
                    return Err(CommandError::TrackExists(*track));
                }
                let created = Track {
                    name: name.clone(),
                    gain: 1.0,
                    regions: BTreeMap::new(),
                };
                self.tracks.insert(*track, created);
            }
            Command::SetTrackGain { track, gain } => {
                if !gain.is_finite() {
                    return Err(CommandError::NonFiniteGain(*track));
                }
                let entry = self.tracks.get_mut(track);
                entry.ok_or(CommandError::UnknownTrack(*track))?.gain = *gain;
            }
            Command::AddRegion {
                region,
                track,
                asset,
                start,
            } => {
                if self.track_of(*region).is_some() {
                    return Err(CommandError::RegionExists(*region));
                }
                let entry = self.tracks.get_mut(track);
                let entry = entry.ok_or(CommandError::UnknownTrack(*track))?;
                let placed = Region {
                    asset: *asset,
                    start: *start,
                };
                entry.regions.insert(*region, placed);
            }
            Command::RemoveRegion { region } => {
                let track = self.track_of(*region);
                let track = track.ok_or(CommandError::UnknownRegion(*region))?;
                if let Some(entry) = self.tracks.get_mut(&track) {
                    entry.regions.remove(region);
                }
            }
        }
        Ok(())
    }

    fn track_of(&self, region: RegionId) -> Option<TrackId> {
        self.tracks
            .iter()
            .find(|(_, track)| track.regions.contains_key(&region))
            .map(|(id, _)| *id)
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder.u64(self.tracks.len() as u64);
        for (id, track) in &self.tracks {
            encoder
                .u64(id.0)
                .str(&track.name)
                .u32(track.gain.to_bits())
                .u64(track.regions.len() as u64);
            for (region, placed) in &track.regions {
                encoder.u64(region.0).digest(&placed.asset).u64(placed.start);
            }
        }
        encoder.finish()
    }

    /// The digest of the canonical state, which a revision names as its root.
    pub fn root(&self) -> Digest {
        Digest::of(Self::DOMAIN, &self.canonical_bytes())
    }
}
