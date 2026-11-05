use crate::record::{self, StreamKind};
use euphony_compiler::{Entry, Hash};
use euphony_mix::SpatialSample;
use serde::de::DeserializeOwned;
use std::io::{self, Read as _};

pub mod fs;

pub trait Storage {
    type Output: Output;
    type Reader: io::Read;
    type Group: Iterator<Item = io::Result<Entry>>;
    type Sink: Iterator<Item = io::Result<SpatialSample>>;

    /// Starts a stream of `kind`, whose file is named by its content hash once it finishes.
    fn create(&self, kind: StreamKind) -> Self::Output;

    /// Writes a record's canonical bytes under `hash`, where no file holds that name yet.
    fn write_record(&self, hash: &Hash, bytes: &[u8]) -> io::Result<()>;

    fn open_raw(&self, hash: &Hash) -> io::Result<Self::Reader>;

    /// Reads the record named `hash` as a `T`.
    fn read_record<T: DeserializeOwned>(&self, hash: &Hash) -> io::Result<T> {
        let mut bytes = vec![];
        self.open_raw(hash)?.read_to_end(&mut bytes)?;
        record::decode(&bytes)
    }

    fn open_group(&self, hash: &Hash) -> io::Result<Self::Group>;

    fn open_sink(&self, hash: &Hash) -> io::Result<Self::Sink>;
}

pub trait Output: 'static + Send + Sync {
    /// Adds `bytes`, whole values of the stream's kind, to the stream. A write that fails ends the
    /// stream, and `finish` answers the failure.
    fn write(&mut self, bytes: &[u8]);

    /// Stores the stream and answers its name, or the failure that kept it from being stored.
    fn finish(&mut self) -> io::Result<Hash>;
}
