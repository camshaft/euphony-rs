use crate::{sample::Sample, sink::SinkMap, Entry};
use std::collections::{hash_map, HashMap};

pub type GroupMap = HashMap<u64, Group>;

#[derive(Debug, Default)]
pub struct Group {
    pub name: String,
    pub sinks: HashMap<u64, Sample>,
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
    iter: hash_map::Iter<'a, u64, Sample>,
    sinks: &'a SinkMap,
}

impl<'a> Iterator for Entries<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let (sink, sample) = self.iter.next()?;
        let sample_offset = (*sample).into();
        let hash = self.sinks[sink].hash;
        Some(Entry {
            sample_offset,
            hash,
        })
    }
}
