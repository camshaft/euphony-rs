use crate::Hash;
pub use euphony_node::BoxProcessor;

pub trait Writer {
    fn is_cached(&self, hash: &Hash) -> bool;
    fn sink(&self, hash: &Hash) -> BoxProcessor;
}
