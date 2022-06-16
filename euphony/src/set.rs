use crate::value::Parameter;

pub trait Set<K> {
    fn set<V: Into<Parameter>>(&self, k: K, v: V) -> &Self;
}
