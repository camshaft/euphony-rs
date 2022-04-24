use euphony_compiler::Hash;
use euphony_node::Sink;

use crate::storage::Storage;

pub mod raw;

pub trait Codec<O: Output>: 'static + Send + Sync + Sink {
    fn new<S: Storage<Output = O>>(storage: &mut S, output: O) -> Self;
}

pub trait Output: 'static + Send + Sync {
    fn write(&mut self, bytes: &[u8]);
    fn finish(&mut self) -> Hash;
}
