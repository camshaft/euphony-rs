use euphony_compiler::Writer;

pub mod codec;
pub mod storage;
pub mod timeline;

#[derive(Clone, Debug, Default)]
pub struct Store<
    S: Writer = storage::fs::Directory<codec::wave::Writer<storage::fs::File>>,
    T: Writer = timeline::Timeline,
> {
    pub storage: S,
    pub timeline: T,
}

impl Store {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Writer, T: Writer> Writer for Store<S, T> {
    #[inline]
    fn is_cached(&self, hash: &[u8; 32]) -> bool {
        self.storage.is_cached(hash)
    }

    #[inline]
    fn sink(&mut self, hash: euphony_compiler::Hash) -> euphony_node::BoxProcessor {
        self.storage.sink(hash)
    }

    #[inline]
    fn group<I: Iterator<Item = euphony_compiler::Entry>>(
        &mut self,
        id: u64,
        name: &str,
        entries: I,
    ) {
        self.timeline.group(id, name, entries)
    }
}
