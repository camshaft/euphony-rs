use euphony_compiler::{DefaultSampleRate, SampleRate, Writer};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timeline {
    pub sample_rate: u32,
    pub samples: u64,
    pub groups: Vec<Group>,
}

impl Default for Timeline {
    #[inline]
    fn default() -> Self {
        Self {
            sample_rate: DefaultSampleRate::COUNT as _,
            samples: 0,
            groups: Default::default(),
        }
    }
}

impl Timeline {
    #[inline]
    pub fn to_json<W: io::Write>(&self, w: W) -> io::Result<()> {
        serde_json::to_writer(w, self)?;
        Ok(())
    }

    #[inline]
    pub fn reset(&mut self) {
        self.sample_rate = DefaultSampleRate::COUNT as _;
        self.samples = 0;
        self.groups.clear();
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Entry {
    pub sample_offset: u64,
    pub hash: Hash,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Hash(euphony_compiler::Hash);

impl Writer for Timeline {
    #[inline]
    fn is_cached(&self, _: &[u8; 32]) -> bool {
        unimplemented!()
    }

    #[inline]
    fn sink(&mut self, _hash: euphony_compiler::Hash) -> euphony_node::BoxProcessor {
        unimplemented!()
    }

    #[inline]
    fn group<I: Iterator<Item = euphony_compiler::Entry>>(
        &mut self,
        _id: u64,
        name: &str,
        entries: I,
    ) {
        let group = Group {
            name: name.to_string(),
            entries: entries
                .map(|e| Entry {
                    sample_offset: e.sample_offset,
                    hash: Hash(e.hash),
                })
                .collect(),
        };

        self.groups.push(group);
    }
}
