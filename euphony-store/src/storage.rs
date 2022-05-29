use euphony_compiler::{Entry, Hash};
use euphony_mix::SpatialSample;
use std::io;

pub mod fs;

pub trait Storage {
    type Output: Output;
    type Reader: io::Read;
    type Group: Iterator<Item = io::Result<Entry>>;
    type Sink: Iterator<Item = io::Result<SpatialSample>>;

    fn create(&self) -> Self::Output;

    fn open_raw(&self, hash: &Hash) -> io::Result<Self::Reader>;

    fn open_group(&self, hash: &Hash) -> io::Result<Self::Group>;

    fn open_sink(&self, hash: &Hash) -> io::Result<Self::Sink>;
}

pub trait Output: 'static + Send + Sync {
    fn write(&mut self, bytes: &[u8]);
    fn finish(&mut self) -> Hash;
}
