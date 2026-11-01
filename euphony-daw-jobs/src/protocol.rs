use euphony_daw_identity::{DecodeError, Decoder, Digest, Encoder};
use euphony_daw_render::RenderDescriptor;
use std::io::{self, Read, Write};

/// A worker's identity for one controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkerId(pub u64);

/// The identity of one attempt at a job, unique across the controller's
/// epochs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttemptId(pub u64);

/// An attempt a controller assigns a worker.
#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub attempt: AttemptId,
    /// The controller's authority epoch: the result counts only in it.
    pub epoch: u64,
    pub descriptor: RenderDescriptor,
}

/// A message from a controller to a worker.
#[derive(Clone, Debug, PartialEq)]
pub enum ToWorker {
    Assign(Assignment),
    Shutdown,
}

impl ToWorker {
    pub fn encode(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        match self {
            Self::Assign(assignment) => {
                let descriptor = assignment.descriptor.canonical_bytes();
                encoder.u8(0).u64(assignment.attempt.0).u64(assignment.epoch);
                encoder.bytes(&descriptor);
            }
            Self::Shutdown => {
                encoder.u8(1);
            }
        }
        encoder.finish()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut decoder = Decoder::new(bytes);
        let message = match decoder.u8()? {
            0 => {
                let attempt = AttemptId(decoder.u64()?);
                let epoch = decoder.u64()?;
                let descriptor = RenderDescriptor::decode(decoder.bytes()?)?;
                Self::Assign(Assignment {
                    attempt,
                    epoch,
                    descriptor,
                })
            }
            1 => Self::Shutdown,
            _ => return Err(DecodeError::Invalid("message to a worker")),
        };
        decoder.finish()?;
        Ok(message)
    }
}

/// A message from a worker to a controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FromWorker {
    /// The next part of an attempt's result bytes.
    Chunk { attempt: AttemptId, bytes: Vec<u8> },
    /// The attempt's result is whole: its descriptor and its bytes' digest.
    Finished {
        attempt: AttemptId,
        epoch: u64,
        descriptor: Digest,
        result: Digest,
    },
    /// The attempt failed.
    Failed { attempt: AttemptId, reason: String },
    /// The worker has no capacity for the attempt now.
    Busy { attempt: AttemptId },
}

impl FromWorker {
    pub fn encode(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        match self {
            Self::Chunk { attempt, bytes } => {
                encoder.u8(0).u64(attempt.0).bytes(bytes);
            }
            Self::Finished {
                attempt,
                epoch,
                descriptor,
                result,
            } => {
                encoder.u8(1).u64(attempt.0).u64(*epoch);
                encoder.digest(descriptor).digest(result);
            }
            Self::Failed { attempt, reason } => {
                encoder.u8(2).u64(attempt.0).str(reason);
            }
            Self::Busy { attempt } => {
                encoder.u8(3).u64(attempt.0);
            }
        }
        encoder.finish()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut decoder = Decoder::new(bytes);
        let tag = decoder.u8()?;
        let attempt = AttemptId(decoder.u64()?);
        let message = match tag {
            0 => Self::Chunk {
                attempt,
                bytes: decoder.bytes()?.to_vec(),
            },
            1 => Self::Finished {
                attempt,
                epoch: decoder.u64()?,
                descriptor: decoder.digest()?,
                result: decoder.digest()?,
            },
            2 => Self::Failed {
                attempt,
                reason: decoder.str()?.to_string(),
            },
            3 => Self::Busy { attempt },
            _ => return Err(DecodeError::Invalid("message from a worker")),
        };
        decoder.finish()?;
        Ok(message)
    }
}

/// Writes one frame: the payload's length, then the payload.
pub fn write_frame(writer: &mut impl Write, payload: &[u8]) -> io::Result<()> {
    writer.write_all(&(payload.len() as u64).to_le_bytes())?;
    writer.write_all(payload)?;
    writer.flush()
}

/// Reads one frame, or `None` where the stream ends between frames.
///
/// A frame's claimed length doesn't allocate: the payload grows only as its
/// bytes arrive.
pub fn read_frame(reader: &mut impl Read) -> io::Result<Option<Vec<u8>>> {
    let mut len = [0; 8];
    let mut read = 0;
    while read < len.len() {
        match reader.read(&mut len[read..])? {
            0 if read == 0 => return Ok(None),
            0 => return Err(io::ErrorKind::UnexpectedEof.into()),
            count => read += count,
        }
    }
    let len = u64::from_le_bytes(len);
    let mut payload = Vec::new();
    reader.by_ref().take(len).read_to_end(&mut payload)?;
    if payload.len() as u64 != len {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    Ok(Some(payload))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_round_trip_and_end_cleanly() {
        let mut stream = Vec::new();
        write_frame(&mut stream, b"one").unwrap();
        write_frame(&mut stream, b"").unwrap();
        let mut reader = &stream[..];
        assert_eq!(read_frame(&mut reader).unwrap(), Some(b"one".to_vec()));
        assert_eq!(read_frame(&mut reader).unwrap(), Some(Vec::new()));
        assert_eq!(read_frame(&mut reader).unwrap(), None);
    }

    #[test]
    fn a_cut_frame_is_an_error() {
        let mut stream = Vec::new();
        write_frame(&mut stream, b"payload").unwrap();
        let mut reader = &stream[..stream.len() - 1];
        assert!(read_frame(&mut reader).is_err());

        let claim = u64::MAX.to_le_bytes();
        let mut reader = &claim[..];
        assert!(read_frame(&mut reader).is_err());
    }

    #[test]
    fn worker_messages_round_trip() {
        let attempt = AttemptId(7);
        let messages = [
            FromWorker::Chunk {
                attempt,
                bytes: vec![1, 2, 3],
            },
            FromWorker::Finished {
                attempt,
                epoch: 2,
                descriptor: Digest::of("euphony test", b"descriptor"),
                result: Digest::of("euphony test", b"result"),
            },
            FromWorker::Failed {
                attempt,
                reason: "no input".to_string(),
            },
            FromWorker::Busy { attempt },
        ];
        for message in messages {
            assert_eq!(FromWorker::decode(&message.encode()), Ok(message));
        }
        let shutdown = ToWorker::Shutdown;
        assert_eq!(ToWorker::decode(&shutdown.encode()), Ok(shutdown));
    }
}
