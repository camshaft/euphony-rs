use crate::{sample::Sample, sink::SinkMap, Entry, Hash};
use blake3::Hasher;
use std::collections::{btree_set, hash_map, BTreeSet, HashMap};

pub type GroupMap = HashMap<u64, Group>;

#[derive(Debug, Default)]
pub struct Group {
    pub name: String,
    pub hash: Hash,
    pub sinks: BTreeSet<(Sample, u64)>,
}

impl Group {
    #[inline]
    pub fn update_hash(&mut self, sinks: &SinkMap) {
        let mut hasher = Hasher::new();
        for (sample, sink) in &self.sinks {
            hasher.update(&sample.to_bytes());
            hasher.update(&sinks[sink].hash);
        }
        self.hash = *hasher.finalize().as_bytes();
    }
}

pub struct Iter<'a> {
    pub iter: hash_map::Iter<'a, u64, Group>,
    pub sinks: &'a SinkMap,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (u64, &'a Group, Entries<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, group) = self.iter.next()?;

            // if the group is empty, no need to return it
            if group.sinks.is_empty() {
                continue;
            }

            let entries = Entries {
                iter: group.sinks.iter(),
                sinks: self.sinks,
            };

            return Some((*id, group, entries));
        }
    }
}

pub struct Entries<'a> {
    iter: btree_set::Iter<'a, (Sample, u64)>,
    sinks: &'a SinkMap,
}

impl<'a> Iterator for Entries<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let (sample, sink) = self.iter.next()?;
        let sample_offset = (*sample).into();
        let hash = self.sinks[sink].hash;
        Some(Entry {
            sample_offset,
            hash,
        })
    }
}
