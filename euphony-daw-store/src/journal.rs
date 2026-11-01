use core::fmt;
use euphony_daw_identity::{Decoder, Digest, Encoder};
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

const DOMAIN: &str = "euphony journal record v1";
const LEN_BYTES: usize = 8;
const CHECK_BYTES: usize = 32;

/// Why a journal didn't open or append.
#[derive(Debug)]
pub enum JournalError {
    /// A record other than the last doesn't match its checksum.
    Damaged { record: usize },
    /// The storage refused the operation.
    Io(io::Error),
}

impl fmt::Display for JournalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Damaged { record } => write!(f, "journal record {record} is damaged"),
            Self::Io(error) => write!(f, "storage refused the journal: {error}"),
        }
    }
}

impl std::error::Error for JournalError {}

impl From<io::Error> for JournalError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// An append-only file of checksummed records, each synced before
/// [`Journal::append`] returns.
#[derive(Debug)]
pub struct Journal {
    file: File,
}

impl Journal {
    /// Opens the journal at `path`, creating it where it doesn't exist, and
    /// returns its committed records in order.
    ///
    /// A torn final record, whose append was interrupted, was never committed:
    /// opening removes it. A damaged record before the last refuses the open.
    pub fn open(path: &Path) -> Result<(Self, Vec<Vec<u8>>), JournalError> {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let mut records = Vec::new();
        let mut offset = 0;
        while offset < bytes.len() {
            let rest = &bytes[offset..];
            let Some(record) = read_record(rest) else {
                break;
            };
            match record {
                Ok((payload, len)) => {
                    records.push(payload);
                    offset += len;
                }
                Err(len) if offset + len == bytes.len() => break,
                Err(_) => {
                    return Err(JournalError::Damaged {
                        record: records.len(),
                    })
                }
            }
        }
        if offset < bytes.len() {
            file.set_len(offset as u64)?;
            file.sync_all()?;
        }
        Ok((Self { file }, records))
    }

    /// Appends one record and syncs it.
    pub fn append(&mut self, payload: &[u8]) -> Result<(), JournalError> {
        let len = LEN_BYTES + payload.len() + CHECK_BYTES;
        let mut record = Encoder::with_writer(Vec::with_capacity(len));
        record.bytes(payload).digest(&Digest::of(DOMAIN, payload));
        let record = record.into_writer();
        let record = record.expect("a vector holds the record");
        self.file.write_all(&record)?;
        self.file.sync_data()?;
        Ok(())
    }

    /// Writes part of a record without syncing it, as an append interrupted
    /// by a crash leaves it.
    pub fn append_torn(&mut self, payload: &[u8]) -> Result<(), JournalError> {
        let mut record = Encoder::new();
        record.u64(payload.len() as u64);
        record.raw(&payload[..payload.len() / 2]);
        self.file.write_all(&record.finish())?;
        Ok(())
    }
}

/// Reads one record from the start of `bytes`: `None` where the bytes end
/// inside it, `Err` with its length where its checksum doesn't match.
fn read_record(bytes: &[u8]) -> Option<Result<(Vec<u8>, usize), usize>> {
    let mut decoder = Decoder::new(bytes);
    let payload = decoder.bytes().ok()?;
    let check = decoder.digest().ok()?;
    let total = bytes.len() - decoder.remaining();
    if check == Digest::of(DOMAIN, &payload) {
        Some(Ok((payload, total)))
    } else {
        Some(Err(total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reopens_with_every_committed_record() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal");
        {
            let (mut journal, records) = Journal::open(&path).unwrap();
            assert!(records.is_empty());
            journal.append(b"one").unwrap();
            journal.append(b"two").unwrap();
        }
        let (_, records) = Journal::open(&path).unwrap();
        assert_eq!(records, vec![b"one".to_vec(), b"two".to_vec()]);
    }

    #[test]
    fn a_record_keeps_its_byte_layout() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal");
        {
            let (mut journal, _) = Journal::open(&path).unwrap();
            journal.append(b"one").unwrap();
        }
        let mut expected = Vec::new();
        expected.extend_from_slice(&3u64.to_le_bytes());
        expected.extend_from_slice(b"one");
        expected.extend_from_slice(Digest::of(DOMAIN, b"one").as_bytes());
        assert_eq!(expected.len(), LEN_BYTES + 3 + CHECK_BYTES);
        assert_eq!(std::fs::read(&path).unwrap(), expected);
    }

    #[test]
    fn a_torn_last_record_is_dropped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal");
        {
            let (mut journal, _) = Journal::open(&path).unwrap();
            journal.append(b"one").unwrap();
            journal.append_torn(b"two that never finished").unwrap();
        }
        let (mut journal, records) = Journal::open(&path).unwrap();
        assert_eq!(records, vec![b"one".to_vec()]);
        journal.append(b"three").unwrap();
        drop(journal);
        let (_, records) = Journal::open(&path).unwrap();
        assert_eq!(records, vec![b"one".to_vec(), b"three".to_vec()]);
    }

    #[test]
    fn a_damaged_earlier_record_refuses_the_open() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal");
        {
            let (mut journal, _) = Journal::open(&path).unwrap();
            journal.append(b"one").unwrap();
            journal.append(b"two").unwrap();
        }
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[LEN_BYTES] ^= 1;
        std::fs::write(&path, bytes).unwrap();
        let refused = Journal::open(&path);
        assert!(matches!(refused, Err(JournalError::Damaged { record: 0 })));
    }
}
