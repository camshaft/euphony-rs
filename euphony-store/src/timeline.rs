use crate::record::{self, Digest};
use euphony_compiler::{
    midi,
    sample::{DefaultRate, Rate as _},
    Hash, Writer,
};
use serde::{Deserialize, Serialize};
use std::io;

/// A project's timeline: its sample rate and each group, written as a canonical typed record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timeline {
    pub sample_rate: u32,
    pub groups: Vec<Group>,
}

impl Default for Timeline {
    #[inline]
    fn default() -> Self {
        Self {
            sample_rate: DefaultRate::COUNT as _,
            groups: Default::default(),
        }
    }
}

impl Timeline {
    /// The timeline's canonical bytes.
    #[inline]
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        record::encode(self)
    }

    /// Reads a timeline from its bytes.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        record::decode(bytes)
    }

    /// Writes the timeline's canonical bytes to `w`.
    #[inline]
    pub fn write_to<W: io::Write>(&self, mut w: W) -> io::Result<()> {
        w.write_all(&self.to_bytes()?)
    }

    #[inline]
    pub fn reset(&mut self) {
        self.sample_rate = DefaultRate::COUNT as _;
        self.groups.clear();
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub entries: Digest,
    pub midi: Option<Digest>,
}

impl Writer for Timeline {
    #[inline]
    fn is_cached(&self, _: &Hash) -> bool {
        unimplemented!()
    }

    #[inline]
    fn sink(&mut self, _hash: &Hash) -> euphony_node::BoxProcessor {
        unimplemented!()
    }

    #[inline]
    fn group<I: Iterator<Item = euphony_compiler::Entry>>(
        &mut self,
        name: &str,
        hash: &Hash,
        _entries: I,
        midi: &midi::Writer,
    ) {
        self.groups.push(Group {
            name: name.to_string(),
            entries: Digest(*hash),
            midi: if midi.is_empty() {
                None
            } else {
                Some(Digest(*midi.hash()))
            },
        });
    }

    fn buffer<
        F: FnOnce(
            Box<dyn euphony_compiler::BufferReader>,
        ) -> euphony_compiler::Result<Vec<euphony_compiler::ConvertedBuffer>, E>,
        E,
    >(
        &self,
        _path: &str,
        _sample_rate: u64,
        _init: F,
    ) -> euphony_compiler::Result<Vec<euphony_compiler::CachedBuffer>, E> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_timeline_written_reads_back() {
        let timeline = Timeline {
            sample_rate: 44_100,
            groups: vec![Group {
                name: "drums".into(),
                entries: Digest([1; 32]),
                midi: None,
            }],
        };

        let mut bytes = vec![];
        timeline.write_to(&mut bytes).unwrap();
        assert_eq!(bytes, timeline.to_bytes().unwrap());
        assert_eq!(Timeline::from_bytes(&bytes).unwrap(), timeline);
        assert_eq!(
            Timeline::from_bytes(b"{\"sample_rate\":48000,\"groups\":[]}")
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidData
        );
    }
}
