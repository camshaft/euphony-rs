use core::fmt;
use euphony_daw_identity::Digest;

/// The order of the channels in interleaved frames.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChannelLayout {
    /// One channel.
    Mono,
    /// Left, then right.
    Stereo,
    /// The given number of channels, in their declared order.
    Discrete(u16),
}

impl ChannelLayout {
    pub fn channels(self) -> u16 {
        match self {
            Self::Mono => 1,
            Self::Stereo => 2,
            Self::Discrete(count) => count,
        }
    }

    pub fn tag(self) -> u8 {
        match self {
            Self::Mono => 0,
            Self::Stereo => 1,
            Self::Discrete(_) => 2,
        }
    }

    /// The layout a header's tag and channel count declare.
    pub fn from_tag(tag: u8, channels: u16) -> Result<Self, FormatError> {
        let layout = match tag {
            0 => Self::Mono,
            1 => Self::Stereo,
            2 => Self::Discrete(channels),
            _ => return Err(FormatError::UnknownLayout(tag)),
        };
        if channels == 0 || layout.channels() != channels {
            return Err(FormatError::ChannelCount(channels));
        }
        Ok(layout)
    }
}

/// The canonical header of working audio.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    pub sample_rate: u32,
    pub layout: ChannelLayout,
    pub frames: u64,
}

impl Header {
    /// The bytes every canonical header starts with.
    pub const MAGIC: [u8; 4] = *b"EUWA";
    /// The header's version.
    pub const VERSION: u16 = 1;
    /// The sample format tag of IEEE 754 binary32.
    pub const BINARY32: u8 = 1;
    /// The header's length in bytes.
    pub const LEN: usize = 22;

    pub fn channels(&self) -> u16 {
        self.layout.channels()
    }

    /// The number of interleaved samples the header declares, where it fits
    /// in memory.
    pub fn samples(&self) -> Result<usize, FormatError> {
        let frames = usize::try_from(self.frames).ok();
        let channels = usize::from(self.channels());
        let samples = frames.and_then(|frames| frames.checked_mul(channels));
        samples.ok_or(FormatError::TooLarge)
    }

    pub fn encode(&self) -> [u8; Self::LEN] {
        let mut out = [0; Self::LEN];
        out[0..4].copy_from_slice(&Self::MAGIC);
        out[4..6].copy_from_slice(&Self::VERSION.to_le_bytes());
        out[6] = Self::BINARY32;
        out[7] = self.layout.tag();
        out[8..10].copy_from_slice(&self.channels().to_le_bytes());
        out[10..14].copy_from_slice(&self.sample_rate.to_le_bytes());
        out[14..22].copy_from_slice(&self.frames.to_le_bytes());
        out
    }

    /// Reads a header from the start of `bytes`.
    pub fn decode(bytes: &[u8]) -> Result<Self, FormatError> {
        let Some(head) = bytes.get(..Self::LEN) else {
            return Err(FormatError::Truncated);
        };
        if head[0..4] != Self::MAGIC {
            return Err(FormatError::Magic);
        }
        let version = u16::from_le_bytes([head[4], head[5]]);
        if version != Self::VERSION {
            return Err(FormatError::Version(version));
        }
        if head[6] != Self::BINARY32 {
            return Err(FormatError::SampleFormat(head[6]));
        }
        let channels = u16::from_le_bytes([head[8], head[9]]);
        let layout = ChannelLayout::from_tag(head[7], channels)?;
        let sample_rate = u32::from_le_bytes([head[10], head[11], head[12], head[13]]);
        if sample_rate == 0 {
            return Err(FormatError::SampleRate);
        }
        let mut frames = [0; 8];
        frames.copy_from_slice(&head[14..22]);
        Ok(Self {
            sample_rate,
            layout,
            frames: u64::from_le_bytes(frames),
        })
    }
}

/// Why bytes or samples aren't valid working audio.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatError {
    /// The bytes end inside the header.
    Truncated,
    /// The bytes don't start with the header's magic.
    Magic,
    /// The header has a version this build doesn't read.
    Version(u16),
    /// The header declares a sample format other than binary32.
    SampleFormat(u8),
    /// The header declares a channel layout this build doesn't know.
    UnknownLayout(u8),
    /// The channel count is zero or doesn't match the layout.
    ChannelCount(u16),
    /// The sample rate is zero.
    SampleRate,
    /// The samples don't fill a whole number of frames.
    PartialFrame,
    /// The sample bytes don't hold exactly the frames the header declares.
    Length { expected: u64, actual: u64 },
    /// The header declares more samples than this computer addresses.
    TooLarge,
    /// A sample isn't finite, at its index among the interleaved samples.
    NonFinite { index: usize },
    /// Integer samples declare a number of valid bits binary32 doesn't hold
    /// exactly.
    IntegerBits(u8),
    /// An integer sample lies outside its declared valid bits.
    IntegerRange { index: usize },
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => f.write_str("the bytes end inside the header"),
            Self::Magic => f.write_str("the bytes aren't working audio"),
            Self::Version(version) => write!(f, "header version {version} isn't supported"),
            Self::SampleFormat(tag) => write!(f, "sample format {tag} isn't binary32"),
            Self::UnknownLayout(tag) => write!(f, "channel layout {tag} isn't known"),
            Self::ChannelCount(count) => write!(f, "{count} channels don't fit the layout"),
            Self::SampleRate => f.write_str("the sample rate is zero"),
            Self::PartialFrame => f.write_str("the samples don't fill whole frames"),
            Self::Length { expected, actual } => {
                write!(f, "expected {expected} sample bytes, found {actual}")
            }
            Self::TooLarge => f.write_str("the header declares more samples than fit in memory"),
            Self::NonFinite { index } => write!(f, "sample {index} isn't finite"),
            Self::IntegerBits(bits) => write!(f, "{bits} valid bits don't convert exactly"),
            Self::IntegerRange { index } => write!(f, "integer sample {index} is out of range"),
        }
    }
}

impl std::error::Error for FormatError {}

/// Interleaved binary32 working audio with its canonical header.
#[derive(Clone, Debug)]
pub struct WorkingAudio {
    header: Header,
    samples: Vec<f32>,
}

impl WorkingAudio {
    /// The digest domain of canonical working-audio bytes.
    pub const DOMAIN: &'static str = "euphony working audio v1";

    pub fn new(
        sample_rate: u32,
        layout: ChannelLayout,
        samples: Vec<f32>,
    ) -> Result<Self, FormatError> {
        if sample_rate == 0 {
            return Err(FormatError::SampleRate);
        }
        let channels = usize::from(layout.channels());
        if channels == 0 {
            return Err(FormatError::ChannelCount(0));
        }
        let frames = samples.len() / channels;
        if frames * channels != samples.len() {
            return Err(FormatError::PartialFrame);
        }
        let header = Header {
            sample_rate,
            layout,
            frames: frames as u64,
        };
        Ok(Self { header, samples })
    }

    /// Converts signed integer samples of up to 24 valid bits exactly, by
    /// division by 2^(bits - 1).
    pub fn from_integer(
        sample_rate: u32,
        layout: ChannelLayout,
        valid_bits: u8,
        samples: &[i32],
    ) -> Result<Self, FormatError> {
        if !(2..=24).contains(&valid_bits) {
            return Err(FormatError::IntegerBits(valid_bits));
        }
        let limit = 1i32 << (valid_bits - 1);
        let scale = 1.0 / limit as f32;
        let mut out = Vec::with_capacity(samples.len());
        for (index, &sample) in samples.iter().enumerate() {
            if !(-limit..limit).contains(&sample) {
                return Err(FormatError::IntegerRange { index });
            }
            out.push(sample as f32 * scale);
        }
        Self::new(sample_rate, layout, out)
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    pub fn frames(&self) -> u64 {
        self.header.frames
    }

    /// Whether `other` holds the same header and the same sample bits.
    pub fn bits_eq(&self, other: &Self) -> bool {
        self.to_canonical_bytes() == other.to_canonical_bytes()
    }

    /// The canonical bytes: the header, then each sample's bits in
    /// little-endian order.
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Header::LEN + self.samples.len() * 4);
        out.extend_from_slice(&self.header.encode());
        for sample in &self.samples {
            out.extend_from_slice(&sample.to_bits().to_le_bytes());
        }
        out
    }

    /// Reads canonical bytes, keeping every sample's bits as they are.
    ///
    /// The header's frame count is checked against the bytes given before
    /// anything is allocated, so a header can't claim more memory than its
    /// bytes hold.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        let header = Header::decode(bytes)?;
        let body = &bytes[Header::LEN..];
        let expected = header.samples()?.checked_mul(4);
        let expected = expected.ok_or(FormatError::TooLarge)?;
        if body.len() != expected {
            return Err(FormatError::Length {
                expected: expected as u64,
                actual: body.len() as u64,
            });
        }
        let samples = body.chunks_exact(4).map(sample_from_le).collect();
        Ok(Self { header, samples })
    }

    /// The semantic digest of the canonical bytes, which names the audio
    /// whatever encoding stores or carries it.
    pub fn digest(&self) -> Digest {
        Digest::of(Self::DOMAIN, &self.to_canonical_bytes())
    }

    /// Checks that every sample is finite, as a render must be before it is
    /// published.
    pub fn check_publishable(&self) -> Result<(), FormatError> {
        match self.samples.iter().position(|sample| !sample.is_finite()) {
            Some(index) => Err(FormatError::NonFinite { index }),
            None => Ok(()),
        }
    }
}

fn sample_from_le(bits: &[u8]) -> f32 {
    let mut word = [0; 4];
    word.copy_from_slice(bits);
    f32::from_bits(u32::from_le_bytes(word))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(layout: ChannelLayout, samples: Vec<f32>) -> WorkingAudio {
        let audio = WorkingAudio::new(48_000, layout, samples);
        audio.unwrap()
    }

    fn decode(bytes: &[u8]) -> Result<WorkingAudio, FormatError> {
        WorkingAudio::from_canonical_bytes(bytes)
    }

    fn edge_samples() -> Vec<f32> {
        vec![
            0.0,
            -0.0,
            f32::from_bits(1),
            -f32::from_bits(1),
            f32::MIN_POSITIVE / 2.0,
            f32::MIN_POSITIVE,
            1.0,
            -1.0,
            4.0,
            f32::MAX,
            f32::from_bits(0x7fc0_1234),
            f32::INFINITY,
        ]
    }

    #[test]
    fn round_trip_preserves_every_admitted_bit() {
        let audio = make(ChannelLayout::Stereo, edge_samples());
        let bytes = audio.to_canonical_bytes();
        let decoded = decode(&bytes).unwrap();
        assert_eq!(decoded.header(), audio.header());
        for (a, b) in audio.samples().iter().zip(decoded.samples()) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
        assert_eq!(decoded.to_canonical_bytes(), bytes);
        assert_eq!(decoded.digest(), audio.digest());
    }

    #[test]
    fn samples_are_little_endian_after_the_header() {
        let bytes = make(ChannelLayout::Mono, vec![1.0]).to_canonical_bytes();
        assert_eq!(&bytes[..4], b"EUWA");
        assert_eq!(&bytes[Header::LEN..], &[0x00, 0x00, 0x80, 0x3f]);
    }

    #[test]
    fn a_changed_bit_changes_the_digest() {
        let positive = make(ChannelLayout::Mono, vec![0.0]);
        let negative = make(ChannelLayout::Mono, vec![-0.0]);
        assert_ne!(positive.digest(), negative.digest());
        assert!(!positive.bits_eq(&negative));
    }

    #[test]
    fn header_round_trips_each_layout() {
        let layouts = [
            ChannelLayout::Mono,
            ChannelLayout::Stereo,
            ChannelLayout::Discrete(6),
        ];
        for layout in layouts {
            let header = Header {
                sample_rate: 96_000,
                layout,
                frames: 1 << 40,
            };
            assert_eq!(Header::decode(&header.encode()), Ok(header));
        }
    }

    #[test]
    fn header_refuses_invalid_fields() {
        let header = Header {
            sample_rate: 48_000,
            layout: ChannelLayout::Stereo,
            frames: 1,
        };
        let good = header.encode();

        assert_eq!(Header::decode(&good[..10]), Err(FormatError::Truncated));

        let mut bytes = good;
        bytes[0] = b'X';
        assert_eq!(Header::decode(&bytes), Err(FormatError::Magic));

        let mut bytes = good;
        bytes[4] = 2;
        assert_eq!(Header::decode(&bytes), Err(FormatError::Version(2)));

        let mut bytes = good;
        bytes[6] = 2;
        assert_eq!(Header::decode(&bytes), Err(FormatError::SampleFormat(2)));

        let mut bytes = good;
        bytes[8] = 3;
        assert_eq!(Header::decode(&bytes), Err(FormatError::ChannelCount(3)));

        let mut bytes = good;
        bytes[10..14].copy_from_slice(&[0; 4]);
        assert_eq!(Header::decode(&bytes), Err(FormatError::SampleRate));
    }

    #[test]
    fn body_length_must_match_the_header() {
        let audio = make(ChannelLayout::Stereo, vec![0.5; 4]);
        let bytes = audio.to_canonical_bytes();
        let cut = decode(&bytes[..bytes.len() - 1]).unwrap_err();
        let expected = FormatError::Length {
            expected: 16,
            actual: 15,
        };
        assert_eq!(cut, expected);

        let header = Header {
            sample_rate: 48_000,
            layout: ChannelLayout::Stereo,
            frames: u64::MAX,
        };
        let claim = decode(&header.encode()).unwrap_err();
        let refused = matches!(claim, FormatError::TooLarge | FormatError::Length { .. });
        assert!(refused);
    }

    #[test]
    fn publication_refuses_non_finite_samples() {
        let audio = make(ChannelLayout::Stereo, edge_samples());
        let refused = FormatError::NonFinite { index: 10 };
        assert_eq!(audio.check_publishable(), Err(refused));
        let finite = edge_samples()[..10].to_vec();
        let audio = make(ChannelLayout::Stereo, finite);
        assert_eq!(audio.check_publishable(), Ok(()));
    }

    #[test]
    fn partial_frames_are_refused() {
        let result = WorkingAudio::new(48_000, ChannelLayout::Stereo, vec![0.0; 3]);
        assert_eq!(result.unwrap_err(), FormatError::PartialFrame);
    }

    #[test]
    fn integer_extrema_convert_exactly() {
        let mono = ChannelLayout::Mono;
        for bits in [16u8, 24] {
            let limit = 1i32 << (bits - 1);
            let input = [-limit, -1, 0, 1, limit - 1];
            let audio = WorkingAudio::from_integer(48_000, mono, bits, &input).unwrap();
            assert_eq!(audio.samples()[0], -1.0);
            for (sample, expected) in audio.samples().iter().zip(input) {
                assert_eq!((*sample * limit as f32) as i32, expected);
            }
        }
        let refused = WorkingAudio::from_integer(48_000, mono, 25, &[0]);
        assert_eq!(refused.unwrap_err(), FormatError::IntegerBits(25));
        let refused = WorkingAudio::from_integer(48_000, mono, 16, &[1 << 15]);
        let expected = FormatError::IntegerRange { index: 0 };
        assert_eq!(refused.unwrap_err(), expected);
    }

    #[test]
    fn over_level_samples_survive_until_attenuated() {
        let loud = make(ChannelLayout::Mono, vec![8.0, -6.5]);
        let decoded = decode(&loud.to_canonical_bytes()).unwrap();
        let attenuated: Vec<f32> = decoded.samples().iter().map(|s| s * 0.125).collect();
        assert_eq!(attenuated, vec![1.0, -0.8125]);
    }
}
