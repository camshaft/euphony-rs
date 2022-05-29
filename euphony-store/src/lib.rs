use euphony_compiler::{Hash, Writer};
use euphony_mix::Mixer;
use std::{io, path::PathBuf};

mod codec;
mod dc;
mod ext;
mod mix;
pub mod storage;
pub mod timeline;

pub type DefaultStorage = storage::fs::Directory;
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

impl<S: storage::Storage + Writer, T: Writer> Store<S, T> {
    #[inline]
    pub fn mix_group<M: Mixer<Error = E>, E: Into<io::Error>>(
        &self,
        group: &Hash,
        mixer: &mut M,
    ) -> io::Result<()> {
        mix::mix(&self.storage, group, mixer)
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

    fn buffer<
        F: FnOnce(
            Box<dyn euphony_compiler::BufferReader>,
        ) -> euphony_compiler::Result<Vec<euphony_compiler::ConvertedBuffer>, E>,
        E,
    >(
        &self,
        path: &str,
        sample_rate: u64,
        init: F,
    ) -> euphony_compiler::Result<Vec<euphony_compiler::CachedBuffer>, E> {
        self.storage.buffer(path, sample_rate, init)
    }
}
