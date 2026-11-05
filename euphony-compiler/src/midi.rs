use crate::{sample::Offset, Hash};
use core::ops::Deref;
use euphony_units::time::Beat;
use serde::{Deserialize, Serialize};
use std::io;

mod smf;

/// A group's MIDI events, in the order the composition sent them, as the canonical typed record
/// the store keeps.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Events {
    pub events: Vec<Event>,
}

/// One MIDI message and the sample and the beat it plays at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub sample: u64,
    /// The beat, as the two parts of its ratio.
    pub beat: (u64, u64),
    pub message: [u8; 3],
}

/// The same record as [`Events`], over events the writer holds.
#[derive(Serialize)]
struct EventsRef<'a> {
    events: &'a [Event],
}

#[derive(Debug, Default)]
pub struct Writer {
    events: Vec<Event>,
    bytes: Vec<u8>,
    hash: Hash,
}

impl Writer {
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub(crate) fn write(&mut self, sample: Offset, beat: Beat, data: [u8; 3]) {
        self.events.push(Event {
            sample: sample.into(),
            beat: (beat.0, beat.1),
            message: data,
        });
    }

    pub(crate) fn finish(&mut self) {
        if self.is_empty() {
            return;
        }

        let events = EventsRef {
            events: &self.events,
        };
        self.bytes = cadenza_ast_serde::to_bytes(&events).expect("MIDI events encode");
        self.hash = *blake3::hash(&self.bytes).as_bytes();
    }
}

impl Deref for Writer {
    type Target = [u8];

    /// The canonical bytes of the events' record, once the writer finishes.
    fn deref(&self) -> &Self::Target {
        &self.bytes[..]
    }
}

pub struct Reader<R: io::Read> {
    r: Option<R>,
    events: std::vec::IntoIter<Event>,
}

impl<R: io::Read> Reader<R> {
    pub fn new(r: R) -> Self {
        Self {
            r: Some(r),
            events: Vec::new().into_iter(),
        }
    }

    pub fn write_smf<W: io::Write + io::Seek>(&mut self, out: W) -> io::Result<()> {
        smf::write(self, out)
    }

    fn try_next(&mut self) -> io::Result<Option<(Offset, Beat, [u8; 3])>> {
        if let Some(mut r) = self.r.take() {
            let mut bytes = vec![];
            r.read_to_end(&mut bytes)?;
            let events: Events = cadenza_ast_serde::from_bytes(&bytes)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
            self.events = events.events.into_iter();
        }

        Ok(self.events.next().map(|event| {
            (
                Offset::new(event.sample),
                Beat(event.beat.0, event.beat.1),
                event.message,
            )
        }))
    }
}

impl<R: io::Read> Iterator for Reader<R> {
    type Item = io::Result<(Offset, Beat, [u8; 3])>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn writer() -> Writer {
        let mut writer = Writer::default();
        writer.write(Offset::new(0), Beat(0, 1), [0x90, 60, 100]);
        writer.write(Offset::new(0), Beat(0, 1), [0x90, 64, 100]);
        writer.write(Offset::new(24_000), Beat(1, 2), [0x80, 60, 0]);
        writer.finish();
        writer
    }

    #[test]
    fn the_events_are_a_canonical_typed_record() {
        let writer = writer();

        let events: Events = cadenza_ast_serde::from_bytes(&writer).unwrap();
        assert_eq!(
            events.events,
            vec![
                Event {
                    sample: 0,
                    beat: (0, 1),
                    message: [0x90, 60, 100],
                },
                Event {
                    sample: 0,
                    beat: (0, 1),
                    message: [0x90, 64, 100],
                },
                Event {
                    sample: 24_000,
                    beat: (1, 2),
                    message: [0x80, 60, 0],
                },
            ]
        );
        assert_eq!(cadenza_ast_serde::to_bytes(&events).unwrap(), &writer[..]);
        assert_eq!(writer.hash(), blake3::hash(&writer).as_bytes());
    }

    #[test]
    fn the_reader_reads_each_event_in_order() {
        let writer = writer();

        let read: Vec<_> = Reader::new(&writer[..])
            .map(|event| {
                let (sample, beat, message) = event.unwrap();
                (u64::from(sample), beat.0, beat.1, message)
            })
            .collect();
        assert_eq!(
            read,
            vec![
                (0, 0, 1, [0x90, 60, 100]),
                (0, 0, 1, [0x90, 64, 100]),
                (24_000, 1, 2, [0x80, 60, 0]),
            ]
        );
    }

    #[test]
    fn damaged_bytes_are_an_error() {
        let writer = writer();

        let mut reader = Reader::new(&writer[..writer.len() / 2]);
        assert!(matches!(
            reader.next(),
            Some(Err(err)) if err.kind() == io::ErrorKind::InvalidData
        ));
    }

    #[test]
    fn an_empty_writer_has_no_bytes() {
        let mut writer = Writer::default();
        writer.finish();
        assert!(writer.is_empty());
        assert!(writer[..].is_empty());
        assert_eq!(writer.hash(), &Hash::default());
    }
}
