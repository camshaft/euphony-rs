use core::fmt;
use euphony_daw_identity::Digest;
use euphony_daw_render::RenderManifest;
use euphony_daw_sample::{ChannelLayout, FormatError, WorkingAudio};
use euphony_daw_store::{ObjectKind, Store, StoreError};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// Why a manifest's audio isn't ready to play.
#[derive(Debug)]
pub enum PlaybackError {
    /// A segment's result couldn't be read.
    Missing { descriptor: Digest, error: StoreError },
    /// A segment's result isn't working audio.
    Malformed { descriptor: Digest, error: FormatError },
    /// A segment's result differs from its segment or the manifest's format.
    Mismatch { descriptor: Digest },
    /// The segments leave a gap or overlap at a timeline frame.
    Gap { at: u64 },
}

impl fmt::Display for PlaybackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { descriptor, error } => write!(f, "segment {descriptor}: {error}"),
            Self::Malformed { descriptor, error } => write!(f, "segment {descriptor}: {error}"),
            Self::Mismatch { descriptor } => write!(f, "segment {descriptor} has another format"),
            Self::Gap { at } => write!(f, "the segments don't continue at frame {at}"),
        }
    }
}

impl std::error::Error for PlaybackError {}

/// The prepared audio of one manifest, verified and joined in timeline
/// order.
#[derive(Clone, Debug)]
pub struct PreparedAudio {
    sample_rate: u32,
    layout: ChannelLayout,
    samples: Arc<[f32]>,
}

impl PreparedAudio {
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn layout(&self) -> ChannelLayout {
        self.layout
    }

    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    /// A stream that plays the audio from its start.
    pub fn stream(&self) -> Stream {
        Stream {
            samples: self.samples.clone(),
            position: Arc::new(AtomicUsize::new(0)),
        }
    }
}

/// Reads and verifies every result a manifest pins.
///
/// A result that is missing, malformed or of another format refuses the
/// preparation: playback never presents silence as a segment's audio.
pub fn prepare(manifest: &RenderManifest, store: &dyn Store) -> Result<PreparedAudio, PlaybackError> {
    let mut samples = Vec::new();
    let mut next = 0;
    for segment in &manifest.segments {
        let descriptor = segment.descriptor;
        if segment.start != next {
            return Err(PlaybackError::Gap { at: next });
        }
        let bytes = store.get(ObjectKind::WorkingAudio, &segment.result);
        let bytes = bytes.map_err(|error| PlaybackError::Missing { descriptor, error })?;
        let audio = WorkingAudio::from_canonical_bytes(&bytes);
        let audio = audio.map_err(|error| PlaybackError::Malformed { descriptor, error })?;
        let header = audio.header();
        if header.sample_rate != manifest.sample_rate
            || header.layout != manifest.layout
            || header.frames != segment.frames
        {
            return Err(PlaybackError::Mismatch { descriptor });
        }
        samples.extend_from_slice(audio.samples());
        next += segment.frames;
    }
    Ok(PreparedAudio {
        sample_rate: manifest.sample_rate,
        layout: manifest.layout,
        samples: samples.into(),
    })
}

/// Prepared audio as one device callback reads it.
#[derive(Clone, Debug)]
pub struct Stream {
    samples: Arc<[f32]>,
    position: Arc<AtomicUsize>,
}

impl Stream {
    /// Copies the next samples into `out`, fills the rest of it with silence
    /// once the audio ends, and returns how many samples came from the audio.
    ///
    /// It doesn't allocate, lock or block, so a device callback may call it.
    pub fn fill(&self, out: &mut [f32]) -> usize {
        let start = self.position.load(Ordering::Acquire);
        let start = start.min(self.samples.len());
        let available = &self.samples[start..];
        let count = available.len().min(out.len());
        out[..count].copy_from_slice(&available[..count]);
        out[count..].fill(0.0);
        self.position.store(start + count, Ordering::Release);
        count
    }

    /// Whether every sample has been read.
    pub fn finished(&self) -> bool {
        self.position.load(Ordering::Acquire) >= self.samples.len()
    }
}
