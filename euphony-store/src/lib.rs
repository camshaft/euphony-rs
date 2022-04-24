use euphony_compiler::{Hash, Writer};
use std::path::PathBuf;

pub mod codec;
pub mod storage;
pub mod timeline;

pub type DefaultStorage = storage::fs::Directory<codec::wave::Writer<storage::fs::File>>;
pub type DefaultTimeline = timeline::Timeline;

#[derive(Clone, Debug, Default)]
pub struct Store<S: Writer = DefaultStorage, T: Writer = DefaultTimeline> {
    pub storage: S,
    pub timeline: T,
}

impl Store {
    #[inline]
    pub fn new(path: PathBuf) -> Self {
        Self {
            storage: DefaultStorage::new(path),
            timeline: Default::default(),
        }
    }
}

impl<S: Writer, T: Writer> Writer for Store<S, T> {
    #[inline]
    fn is_cached(&self, hash: &Hash) -> bool {
        self.storage.is_cached(hash)
    }

    #[inline]
    fn sink(&mut self, hash: &Hash) -> euphony_node::BoxProcessor {
        self.storage.sink(hash)
    }

    #[inline]
    fn group<I: Iterator<Item = euphony_compiler::Entry>>(
        &mut self,
        name: &str,
        hash: &Hash,
        entries: I,
    ) {
        self.storage.group(name, hash, entries);
        self.timeline.group(name, hash, None.into_iter());
    }
}
