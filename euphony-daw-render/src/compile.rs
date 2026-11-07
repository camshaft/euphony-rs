use crate::{Input, RenderDescriptor};
use core::fmt;
use euphony_daw_identity::Digest;
use euphony_daw_project::ProjectState;
use euphony_daw_sample::{ChannelLayout, FormatError, Header, ProcessorId};
use euphony_daw_store::{ObjectKind, Store, StoreError};
use std::collections::BTreeMap;

/// The format a project renders in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputFormat {
    pub sample_rate: u32,
    pub layout: ChannelLayout,
}

/// Why a project didn't compile.
#[derive(Debug)]
pub enum CompileError {
    /// A segment of zero frames was asked for.
    EmptySegment,
    /// An asset couldn't be read.
    Asset { asset: Digest, error: StoreError },
    /// An asset isn't valid working audio.
    Format { asset: Digest, error: FormatError },
    /// An asset's format differs from the project's.
    Incompatible { asset: Digest },
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySegment => f.write_str("segments need at least one frame"),
            Self::Asset { asset, error } => write!(f, "asset {asset}: {error}"),
            Self::Format { asset, error } => write!(f, "asset {asset}: {error}"),
            Self::Incompatible { asset } => write!(f, "asset {asset} has another format"),
        }
    }
}

impl std::error::Error for CompileError {}

struct Placed {
    asset: Digest,
    position: u64,
    end: u64,
    gain: f32,
}

impl Placed {
    fn overlaps(&self, start: u64, end: u64) -> bool {
        self.position < end && self.end > start
    }
}

/// Compiles a project state into one render descriptor for each segment of
/// `segment_frames` frames, the last one shorter where the project ends
/// inside it.
///
/// Each descriptor holds only the inputs that overlap its segment, so an edit
/// elsewhere leaves its identity, and the result cached for it, unchanged.
pub fn compile(
    state: &ProjectState,
    store: &dyn Store,
    format: OutputFormat,
    processor: ProcessorId,
    segment_frames: u64,
) -> Result<Vec<RenderDescriptor>, CompileError> {
    if segment_frames == 0 {
        return Err(CompileError::EmptySegment);
    }
    let mut lengths = BTreeMap::new();
    let mut placed = Vec::new();
    for track in state.tracks.values() {
        for region in track.regions.values() {
            let asset = region.asset;
            let frames = match lengths.get(&asset) {
                Some(frames) => *frames,
                None => {
                    let frames = asset_frames(store, asset, format)?;
                    lengths.insert(asset, frames);
                    frames
                }
            };
            placed.push(Placed {
                asset,
                position: region.start,
                end: region.start.saturating_add(frames),
                gain: track.gain,
            });
        }
    }

    let end = placed.iter().map(|placed| placed.end).max().unwrap_or(0);
    let mut descriptors = Vec::new();
    let mut start = 0;
    while start < end {
        let frames = segment_frames.min(end - start);
        let mut inputs = Vec::new();
        for placed in &placed {
            if placed.overlaps(start, start + frames) {
                inputs.push(Input {
                    asset: placed.asset,
                    position: placed.position,
                    gain: placed.gain,
                });
            }
        }
        descriptors.push(RenderDescriptor {
            sample_rate: format.sample_rate,
            layout: format.layout,
            start,
            frames,
            processor,
            inputs,
        });
        start += frames;
    }
    Ok(descriptors)
}

fn asset_frames(
    store: &dyn Store,
    asset: Digest,
    format: OutputFormat,
) -> Result<u64, CompileError> {
    let bytes = store.get(ObjectKind::WorkingAudio, &asset);
    let bytes = bytes.map_err(|error| CompileError::Asset { asset, error })?;
    let header = Header::decode(&bytes);
    let header = header.map_err(|error| CompileError::Format { asset, error })?;
    if header.sample_rate != format.sample_rate || header.layout != format.layout {
        return Err(CompileError::Incompatible { asset });
    }
    Ok(header.frames)
}
