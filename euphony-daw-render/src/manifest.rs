use euphony_daw_identity::{DecodeError, Decoder, Digest, Encoder};
use euphony_daw_project::RevisionId;
use euphony_daw_sample::ChannelLayout;

/// One segment of a manifest: a descriptor and the result chosen for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Segment {
    pub descriptor: Digest,
    pub start: u64,
    pub frames: u64,
    /// The digest of the result's canonical working audio.
    pub result: Digest,
}

/// The prepared audio of one revision: one validated result for each of its
/// descriptors, in timeline order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderManifest {
    pub revision: RevisionId,
    pub sample_rate: u32,
    pub layout: ChannelLayout,
    pub segments: Vec<Segment>,
}

impl RenderManifest {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder
            .digest(&self.revision.0)
            .u32(self.sample_rate)
            .u8(self.layout.tag())
            .u16(self.layout.channels())
            .u64(self.segments.len() as u64);
        for segment in &self.segments {
            encoder
                .digest(&segment.descriptor)
                .u64(segment.start)
                .u64(segment.frames)
                .digest(&segment.result);
        }
        encoder.finish()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut decoder = Decoder::new(bytes);
        let revision = RevisionId(decoder.digest()?);
        let sample_rate = decoder.u32()?;
        let tag = decoder.u8()?;
        let channels = decoder.u16()?;
        let layout = ChannelLayout::from_tag(tag, channels);
        let layout = layout.map_err(|_| DecodeError::Invalid("channel layout"))?;
        // A segment is two digests, its start and its frame count.
        let count = decoder.count(2 * Digest::ENCODED_LEN + 16)?;
        let mut segments = Vec::with_capacity(count);
        for _ in 0..count {
            segments.push(Segment {
                descriptor: decoder.digest()?,
                start: decoder.u64()?,
                frames: decoder.u64()?,
                result: decoder.digest()?,
            });
        }
        decoder.finish()?;
        Ok(Self {
            revision,
            sample_rate,
            layout,
            segments,
        })
    }
}
