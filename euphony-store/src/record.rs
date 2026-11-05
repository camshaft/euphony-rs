//! The typed records the store keeps on disk.
//!
//! Each file under the store's directory holds one record, written as Cadenza's canonical binary
//! tree with `cadenza_ast_serde::to_bytes` and read with `cadenza_ast_serde::from_bytes`. A stream
//! of samples or coordinates is a record whose body is a byte string of its values in little-endian
//! order, since a tree value for each sample would multiply its size.
//!
//! A stream's file is named by the content hash of its record's bytes. A group index, a sink index
//! and a buffer list are named by the hash of what produced them, which the compiler gives.

use core::{fmt, ops};
use euphony_compiler::Hash;
use euphony_units::coordinates::Cartesian;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::io;

/// Encodes `value` as its canonical bytes.
pub fn encode<T: ?Sized + Serialize>(value: &T) -> io::Result<Vec<u8>> {
    cadenza_ast_serde::to_bytes(value)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
}

/// Decodes a record of type `T`. Bytes that aren't such a record are an `InvalidData` error that
/// holds the decoder's own error, which says why.
pub fn decode<T: de::DeserializeOwned>(bytes: &[u8]) -> io::Result<T> {
    cadenza_ast_serde::from_bytes(bytes)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

/// The content hash of a record's canonical bytes.
pub fn content_hash(bytes: &[u8]) -> Hash {
    *blake3::hash(bytes).as_bytes()
}

/// A blake3 digest, written as a byte string.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digest(pub Hash);

impl ops::Deref for Digest {
    type Target = Hash;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Hash> for Digest {
    #[inline]
    fn from(hash: Hash) -> Self {
        Self(hash)
    }
}

impl Serialize for Digest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Digest;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a 32-byte digest")
            }

            fn visit_bytes<E: de::Error>(self, bytes: &[u8]) -> Result<Digest, E> {
                Hash::try_from(bytes)
                    .map(Digest)
                    .map_err(|_| E::invalid_length(bytes.len(), &self))
            }
        }

        deserializer.deserialize_bytes(Visitor)
    }
}

/// A stream's body, written as one byte string rather than as a sequence of numbers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Body(pub Vec<u8>);

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Serialize for Body {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Body {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Body;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a byte string")
            }

            fn visit_bytes<E: de::Error>(self, bytes: &[u8]) -> Result<Body, E> {
                Ok(Body(bytes.to_vec()))
            }

            fn visit_byte_buf<E: de::Error>(self, bytes: Vec<u8>) -> Result<Body, E> {
                Ok(Body(bytes))
            }
        }

        deserializer.deserialize_bytes(Visitor)
    }
}

/// The entries of a group, in the order they start.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupIndex {
    pub entries: Vec<GroupEntry>,
}

/// A sink of a group and the sample it starts at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupEntry {
    pub sample_offset: u64,
    pub sink: Digest,
}

/// A sink's two streams, by the content hashes of their records.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SinkIndex {
    pub samples: Digest,
    pub coordinates: Digest,
}

/// The sample streams a buffer's source converted to, one for each channel, by the content hashes
/// of their records.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferList {
    pub channels: Vec<Digest>,
}

/// The two kinds of stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamKind {
    Samples,
    Coordinates,
}

impl StreamKind {
    /// The bytes of one value of the stream's body.
    #[inline]
    pub fn value_len(self) -> usize {
        match self {
            Self::Samples => SAMPLE,
            Self::Coordinates => COORDINATE_RUN,
        }
    }
}

/// The bytes of one sample of a sample stream: an `f64` in little-endian order.
pub const SAMPLE: usize = 8;

/// The bytes of one run of a coordinate stream, in little-endian order: the count of samples
/// after the first that share the coordinate, as a `u32`, then x, y and z, each as an `f32`.
pub const COORDINATE_RUN: usize = 16;

/// A stream of samples or of coordinate runs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stream {
    Samples { samples: Body },
    Coordinates { runs: Body },
}

impl Stream {
    #[inline]
    pub fn new(kind: StreamKind, body: Vec<u8>) -> Self {
        let body = Body(body);
        match kind {
            StreamKind::Samples => Self::Samples { samples: body },
            StreamKind::Coordinates => Self::Coordinates { runs: body },
        }
    }

    #[inline]
    pub fn kind(&self) -> StreamKind {
        match self {
            Self::Samples { .. } => StreamKind::Samples,
            Self::Coordinates { .. } => StreamKind::Coordinates,
        }
    }

    /// The stream's body, where the stream is of `kind` and its body holds whole values.
    pub fn into_body(self, kind: StreamKind) -> io::Result<Vec<u8>> {
        let found = self.kind();
        let body = match self {
            Self::Samples { samples } => samples,
            Self::Coordinates { runs } => runs,
        }
        .0;

        if found != kind {
            return Err(invalid(format!("expected a {kind:?} stream, found a {found:?} stream")));
        }

        if body.len() % kind.value_len() != 0 {
            return Err(invalid(format!(
                "a {kind:?} stream of {} bytes ends inside a value",
                body.len()
            )));
        }

        Ok(body)
    }
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

/// The body of a sample stream that holds `samples`.
pub fn samples_body(samples: &[f64]) -> Vec<u8> {
    let mut body = Vec::with_capacity(samples.len() * SAMPLE);
    for sample in samples {
        body.extend_from_slice(&sample.to_le_bytes());
    }
    body
}

/// The samples of a sample stream's body.
pub fn samples_from_body(body: &[u8]) -> impl Iterator<Item = f64> + '_ {
    body.chunks_exact(SAMPLE)
        .map(|sample| f64::from_le_bytes(sample.try_into().unwrap()))
}

/// One run of a coordinate stream's body.
pub fn coordinate_run(count: u32, coord: Cartesian<f64>) -> [u8; COORDINATE_RUN] {
    let mut bytes = [0u8; COORDINATE_RUN];
    bytes[..4].copy_from_slice(&count.to_le_bytes());
    bytes[4..8].copy_from_slice(&(coord.x as f32).to_le_bytes());
    bytes[8..12].copy_from_slice(&(coord.y as f32).to_le_bytes());
    bytes[12..].copy_from_slice(&(coord.z as f32).to_le_bytes());
    bytes
}

/// The count and the coordinate of one run of a coordinate stream's body, `COORDINATE_RUN` bytes.
pub fn read_coordinate_run(run: &[u8]) -> (u32, Cartesian<f64>) {
    let word = |at: usize| -> [u8; 4] { run[at..at + 4].try_into().unwrap() };
    let count = u32::from_le_bytes(word(0));
    let coord = Cartesian {
        x: f32::from_le_bytes(word(4)) as f64,
        y: f32::from_le_bytes(word(8)) as f64,
        z: f32::from_le_bytes(word(12)) as f64,
    };
    (count, coord)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timeline::{Group, Timeline};

    fn round_trip<T: Serialize + de::DeserializeOwned + PartialEq + fmt::Debug>(value: &T) {
        let bytes = encode(value).unwrap();
        let read: T = decode(&bytes).unwrap();
        assert_eq!(&read, value);
        // the bytes of the value read are the bytes written: one canonical form
        assert_eq!(encode(&read).unwrap(), bytes);
    }

    #[test]
    fn each_record_round_trips_to_its_canonical_bytes() {
        round_trip(&GroupIndex {
            entries: vec![
                GroupEntry {
                    sample_offset: 0,
                    sink: Digest([1; 32]),
                },
                GroupEntry {
                    sample_offset: 48_000,
                    sink: Digest([2; 32]),
                },
            ],
        });
        round_trip(&SinkIndex {
            samples: Digest([3; 32]),
            coordinates: Digest([4; 32]),
        });
        round_trip(&Stream::new(
            StreamKind::Samples,
            samples_body(&[0.5, -1.0, f64::MIN_POSITIVE]),
        ));
        round_trip(&Stream::new(
            StreamKind::Coordinates,
            coordinate_run(
                2,
                Cartesian {
                    x: 1.0,
                    y: -2.0,
                    z: 0.5,
                },
            )
            .to_vec(),
        ));
        round_trip(&BufferList {
            channels: vec![Digest([5; 32]), Digest([6; 32])],
        });
        round_trip(&Timeline {
            sample_rate: 48_000,
            groups: vec![
                Group {
                    name: "lead".into(),
                    entries: Digest([7; 32]),
                    midi: Some(Digest([8; 32])),
                },
                Group {
                    name: "bass".into(),
                    entries: Digest([9; 32]),
                    midi: None,
                },
            ],
        });
    }

    #[test]
    fn a_digest_is_written_as_a_byte_string() {
        let hash = [0xab; 32];
        assert_eq!(
            encode(&Digest(hash)).unwrap(),
            encode(&Body(hash.to_vec())).unwrap()
        );
    }

    #[test]
    fn a_stream_body_is_a_byte_string_and_reads_back() {
        for bytes in [vec![], vec![0x01], samples_body(&[1.5, -0.25])] {
            let encoded = encode(&Body(bytes.clone())).unwrap();
            if !bytes.is_empty() {
                // a byte string, not a sequence of numbers
                assert_ne!(encoded, encode(&bytes).unwrap());
            }
            assert_eq!(decode::<Body>(&encoded).unwrap(), Body(bytes));
        }

        // a 32-byte body is read by the same byte-string visitor as a digest
        let encoded = encode(&Body(vec![0xcd; 32])).unwrap();
        assert_eq!(decode::<Digest>(&encoded).unwrap(), Digest([0xcd; 32]));
    }

    #[test]
    fn stream_bodies_hold_little_endian_values() {
        let body = samples_body(&[1.5, -0.25]);
        let mut expected = 1.5f64.to_le_bytes().to_vec();
        expected.extend_from_slice(&(-0.25f64).to_le_bytes());
        assert_eq!(body, expected);
        assert_eq!(
            samples_from_body(&body).collect::<Vec<_>>(),
            vec![1.5, -0.25]
        );

        let run = coordinate_run(
            7,
            Cartesian {
                x: 1.0,
                y: 2.0,
                z: -3.0,
            },
        );
        assert_eq!(&run[..4], &7u32.to_le_bytes());
        assert_eq!(&run[4..8], &1.0f32.to_le_bytes());
        assert_eq!(&run[8..12], &2.0f32.to_le_bytes());
        assert_eq!(&run[12..], &(-3.0f32).to_le_bytes());
        let (count, coord) = read_coordinate_run(&run);
        assert_eq!(count, 7);
        assert_eq!((coord.x, coord.y, coord.z), (1.0, 2.0, -3.0));
    }

    fn decoder_error(err: &io::Error) -> &cadenza_ast_serde::Error {
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        err.get_ref()
            .and_then(|inner| inner.downcast_ref::<cadenza_ast_serde::Error>())
            .expect("the decoder's error")
    }

    #[test]
    fn damaged_bytes_are_refused_with_the_decoders_reason() {
        let bytes = encode(&SinkIndex {
            samples: Digest([1; 32]),
            coordinates: Digest([2; 32]),
        })
        .unwrap();
        let err = decode::<SinkIndex>(&bytes[..bytes.len() / 2]).unwrap_err();
        assert!(
            matches!(
                decoder_error(&err),
                cadenza_ast_serde::Error::MalformedBinaryAst(_)
            ),
            "{err}"
        );
    }

    #[test]
    fn a_record_of_another_type_is_refused() {
        let bytes = encode(&BufferList {
            channels: vec![Digest([1; 32])],
        })
        .unwrap();
        let err = decode::<SinkIndex>(&bytes).unwrap_err();
        decoder_error(&err);
    }

    #[test]
    fn a_digest_of_another_length_is_refused() {
        let bytes = encode(&Body(vec![1; 31])).unwrap();
        let err = decode::<Digest>(&bytes).unwrap_err();
        decoder_error(&err);
    }

    #[test]
    fn a_stream_is_read_only_as_its_own_kind_and_whole_values() {
        let samples = Stream::new(StreamKind::Samples, samples_body(&[1.0]));
        assert_eq!(
            samples
                .clone()
                .into_body(StreamKind::Coordinates)
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            samples.into_body(StreamKind::Samples).unwrap(),
            1.0f64.to_le_bytes()
        );

        let partial = Stream::new(StreamKind::Coordinates, vec![0; COORDINATE_RUN + 1]);
        assert_eq!(
            partial
                .into_body(StreamKind::Coordinates)
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidData
        );
    }
}
