use crate::{
    sample::{Offset, RelOffset},
    Hash,
};
use std::collections::{BTreeMap, BTreeSet};

pub type SinkMap = BTreeMap<u64, Sink>;

#[derive(Debug)]
pub struct Sink {
    pub hash: Hash,
    pub members: BTreeSet<u64>,
    pub start: Offset,
    pub end: RelOffset,
    pub is_acyclic: bool,
    pub is_cached: bool,
}

impl Default for Sink {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            members: Default::default(),
            start: Default::default(),
            end: Default::default(),
            is_acyclic: true,
            is_cached: false,
        }
    }
}
