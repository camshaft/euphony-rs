use crate::{
    output::*,
    parameter::{Parameter, ParameterValue},
    processor::Definition,
    sink::Sink,
};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

thread_local! {
    static NODE_ID: Counter = Counter::new();
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
}

impl Drop for OwnedNode {
    fn drop(&mut self) {
        emit(FinishNode { node: self.id })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Node(Arc<OwnedNode>);

impl Node {
    pub fn id(&self) -> u64 {
        self.0.id
    }

    pub fn new(definition: &Definition, group: Option<u64>) -> Self {
        let id = NODE_ID.with(|v| v.next());

        emit(SpawnNode {
            id,
            processor: definition.id,
            group,
        });

        let node = OwnedNode {
            id,
            parameters: Mutex::new(vec![
                Parameter(ParameterValue::Unset);
                definition.inputs as usize
            ]),
        };

        Node(Arc::new(node))
    }

    pub fn set<V: Into<Parameter>>(&self, index: u64, value: V) {
        let value = value.into();
        value.set(self.id(), index);
        self.0.parameters.lock().unwrap()[index as usize] = value;
    }

    pub fn sink(&self) -> Sink {
        Sink::default().with(self)
    }
}
