use crate::output::*;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

thread_local! {
    static NODE_ID: Counter = Counter::new();
    static SINK_ID: Counter = Counter::new();
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
pub struct Sink {
    id: u64,
    #[allow(dead_code)] // retain a reference to the output for the duration of the sink
    node: Node,
}

impl Sink {
    pub fn spawn(node: Node) -> Self {
        let id = NODE_ID.with(|v| v.next());

        let group = crate::runtime::group::current().as_u64();

        emit(SpawnSink {
            id,
            source_node: node.id(),
            group,
        });

        Self { id, node }
    }
}

impl Drop for Sink {
    fn drop(&mut self) {
        emit(FinishSink { sink: self.id })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Generator {
    pub id: u64,
    pub name: &'static str,
    pub inputs: u64,
    pub outputs: u64,
}

impl Generator {
    pub fn spawn(&'static self) -> Node {
        let id = NODE_ID.with(|v| v.next());

        emit(SpawnNode {
            id,
            generator: self.id,
        });

        let node = OwnedNode {
            id,
            generator: self,
            parameters: Mutex::new(vec![Parameter::Unset; self.inputs as usize]),
        };

        Node(Arc::new(node))
    }
}

#[derive(Debug)]
struct OwnedNode {
    id: u64,
    generator: &'static Generator,
    parameters: Mutex<Vec<Parameter>>,
}

impl Drop for OwnedNode {
    fn drop(&mut self) {
        emit(FinishNode { node: self.id })
    }
}

#[derive(Clone, Debug)]
pub struct Node(Arc<OwnedNode>);

impl Node {
    fn id(&self) -> u64 {
        self.0.id
    }

    pub fn set<V: Into<Parameter>>(&self, index: u64, value: V) {
        let value = value.into();
        match value {
            Parameter::Unset => emit(SetParameter {
                target_node: self.id(),
                target_parameter: index as _,
                value: 0.0f64.to_bits(),
            }),
            Parameter::Constant(value) => emit(SetParameter {
                target_node: self.id(),
                target_parameter: index as _,
                value: value.to_bits(),
            }),
            Parameter::Node(ref source) => emit(PipeParameter {
                target_node: self.id(),
                target_parameter: index as _,
                source_node: source.id(),
            }),
        }
        self.0.parameters.lock().unwrap()[index as usize] = value;
    }

    pub fn sink(&self) -> Sink {
        Sink::spawn(self.clone())
    }
}

#[derive(Clone, Debug)]
pub enum Parameter {
    Unset,
    Constant(f64),
    Node(Node),
}

impl From<Node> for Parameter {
    fn from(node: Node) -> Self {
        Self::Node(node)
    }
}

impl From<&Node> for Parameter {
    fn from(node: &Node) -> Self {
        Self::Node(node.clone())
    }
}

impl From<f64> for Parameter {
    fn from(value: f64) -> Self {
        Self::Constant(value)
    }
}
