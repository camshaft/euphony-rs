use euphony_daw_identity::{DecodeError, Decoder, Digest, Encoder};
use euphony_daw_sample::{ChannelLayout, ProcessorId};

/// One input of a render: an asset, where it plays and its gain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Input {
    /// The digest of the asset's canonical working audio.
    pub asset: Digest,
    /// The timeline frame the asset's first frame plays at.
    pub position: u64,
    pub gain: f32,
}

/// The identity of a requested calculation: everything its result depends
/// on, and nothing it doesn't, such as the worker or the time.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderDescriptor {
    pub sample_rate: u32,
    pub layout: ChannelLayout,
    /// The first timeline frame of the range.
    pub start: u64,
    pub frames: u64,
    /// The processor and numerical profile that render it.
    pub processor: ProcessorId,
    pub inputs: Vec<Input>,
}

impl RenderDescriptor {
    /// The descriptor schema's version.
    pub const SCHEMA_VERSION: u32 = 1;
    const DOMAIN: &'static str = "euphony render descriptor v1";

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder
            .u32(Self::SCHEMA_VERSION)
            .u32(self.sample_rate)
            .u8(self.layout.tag())
            .u16(self.layout.channels())
            .u64(self.start)
            .u64(self.frames)
            .digest(&self.processor.0)
            .u64(self.inputs.len() as u64);
        for input in &self.inputs {
            encoder
                .digest(&input.asset)
                .u64(input.position)
                .u32(input.gain.to_bits());
        }
        encoder.finish()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.u32()? != Self::SCHEMA_VERSION {
            return Err(DecodeError::Invalid("descriptor schema version"));
        }
        let sample_rate = decoder.u32()?;
        let tag = decoder.u8()?;
        let channels = decoder.u16()?;
        let layout = ChannelLayout::from_tag(tag, channels);
        let layout = layout.map_err(|_| DecodeError::Invalid("channel layout"))?;
        let start = decoder.u64()?;
        let frames = decoder.u64()?;
        let processor = ProcessorId(decoder.digest()?);
        let count = decoder.count(44)?;
        let mut inputs = Vec::with_capacity(count);
        for _ in 0..count {
            inputs.push(Input {
                asset: decoder.digest()?,
                position: decoder.u64()?,
                gain: f32::from_bits(decoder.u32()?),
            });
        }
        decoder.finish()?;
        Ok(Self {
            sample_rate,
            layout,
            start,
            frames,
            processor,
            inputs,
        })
    }

    /// The descriptor's identity.
    pub fn id(&self) -> Digest {
        Digest::of(Self::DOMAIN, &self.canonical_bytes())
    }
}
