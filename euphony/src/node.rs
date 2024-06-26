use crate::{
    output,
    processor::Definition,
    sink::Sink,
    value::{Parameter, ParameterValue},
};
use euphony_buffer::AsChannel;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

thread_local! {
    static NODE_ID: Counter = Counter::new();
    static BUFFER_ID: Counter = Counter::new();
}

struct Counter(RefCell<u64>);

impl Counter {
    const fn new() -> Self {
        Self(RefCell::new(0))
    }

    fn next(&self) -> u64 {
        let mut v = self.0.borrow_mut();
        let current = *v;
        *v += 1;
        current
    }
}

#[derive(Debug)]
struct OwnedNode {
    id: u64,
    parameters: Mutex<Vec<Parameter>>,
    buffers: u64,
}

impl Drop for OwnedNode {
    fn drop(&mut self) {
        output::finish_node(self.id);
    }
}

#[derive(Clone, Debug)]
#[must_use = "nodes do nothing unless routed to a Sink"]
pub struct Node(Arc<OwnedNode>);

impl Node {
    pub(crate) fn id(&self) -> u64 {
        self.0.id
    }

    pub(crate) fn new(definition: &Definition, group: Option<u64>) -> Self {
        let id = NODE_ID.with(|v| v.next());

        output::spawn_node(id, definition.id, group);

        let node = OwnedNode {
            id,
            parameters: Mutex::new(vec![
                Parameter(ParameterValue::Unset);
                definition.inputs as usize
            ]),
            buffers: definition.buffers,
        };

        Node(Arc::new(node))
    }

    pub(crate) fn fork(&self) -> Self {
        let id = NODE_ID.with(|v| v.next());

        output::fork_node(self.id(), id);

        let parameters = self.0.parameters.lock().unwrap().len();

        let node = OwnedNode {
            id,
            parameters: Mutex::new(vec![Parameter(ParameterValue::Unset); parameters]),
            buffers: self.0.buffers,
        };

        Node(Arc::new(node))
    }

    pub(crate) fn set<V: Into<Parameter>>(&self, index: u64, value: V) {
        let value = value.into();
        value.set(self.id(), index);
        self.0.parameters.lock().unwrap()[index as usize] = value;
    }

    pub(crate) fn set_buffer<C: AsChannel>(&self, index: u64, channel: C) {
        let buffer = channel.buffer(|path, ext| {
            let id = BUFFER_ID.with(|v| v.next());
            // load the buffer if needed
            output::load_buffer(id, path, ext);
            id
        });
        let buffer_channel = channel.channel();

        // update the buffer for the node
        output::set_buffer(self.id(), index, buffer, buffer_channel);

        assert!(self.0.buffers > index);
    }

    pub fn sink(&self) -> Sink {
        Sink::default().with(self)
    }
}

impl crate::processor::Processor for Node {
    fn sink(&self) -> Sink {
        Sink::default().with(self)
    }

    fn node(&self) -> Node {
        self.clone()
    }
}

define_processor_ops!(Node);
