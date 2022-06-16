use crate::{node::Node, value};
use euphony_buffer::AsChannel;

#[derive(Clone, Debug)]
pub struct Parameter {
    pub(crate) node: Node,
    pub(crate) index: u64,
}

impl Parameter {
    pub fn set<V: Into<value::Parameter>>(&self, value: V) {
        self.node.set(self.index, value)
    }
}

#[derive(Clone, Debug)]
pub struct Trigger {
    pub(crate) node: Node,
    pub(crate) index: u64,
}

impl Trigger {
    pub fn set<V: Into<value::Trigger>>(&self, value: V) {
        self.node.set(self.index, value.into())
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    pub(crate) node: Node,
    pub(crate) index: u64,
}

impl Buffer {
    pub fn set<C: AsChannel>(&self, channel: C) {
        self.node.set_buffer(self.index, channel)
    }
}
