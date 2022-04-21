pub use euphony_macros::Node;

#[doc(hidden)]
#[cfg(feature = "reflect")]
pub mod reflect;

use euphony_graph as graph;

pub type Error = String;

pub type BoxProcessor = Box<dyn graph::Processor<Config>>;

mod sink;
pub use sink::{SampleType, Sink};

#[inline]
pub fn spawn<const I: usize, const B: usize, N: Node<I, B>>(node: N) -> BoxProcessor {
    Box::new(StaticNode::new(node))
}

type BufferMap = ();

pub type Sample = f64;
pub const LEN: usize = 4000 / (core::mem::size_of::<Sample>() / 8);
pub type Output = [Sample; LEN];

type BufferKey = u64;

#[derive(Debug, Default)]
pub struct Context {
    pub buffers: BufferMap,
    pub partial: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Config;

impl graph::Config for Config {
    type Context = Context;
    type Output = Output;
    type Value = Value;
    type Parameter = Parameter;
}

type Parameter = u64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterValue {
    Constant(f64),
    Node(u64),
}

pub enum Value {
    Constant(f64),
    Buffer(BufferKey),
}

pub struct Inputs<'a, const I: usize> {
    inputs: graph::Inputs<'a, Config>,
    keys: &'a [graph::Input<f64>; I],
}

impl<'a, const I: usize> Inputs<'a, I> {
    #[inline]
    pub fn get(&self, index: usize) -> Input {
        debug_assert!(index < I);
        match unsafe { *self.keys.get_unchecked(index) } {
            graph::Input::Value(v) => Input::Constant(v),
            graph::Input::Node(n) => Input::Buffer(&self.inputs[n]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Input<'a> {
    Constant(f64),
    Buffer(&'a Output),
}

impl<'a> Input<'a> {
    pub fn iter(&self) -> InputIter {
        match self {
            Self::Constant(v) => InputIter::Constant(*v),
            Self::Buffer(v) => InputIter::Buffer(v.iter()),
        }
    }
}

pub enum InputIter<'a> {
    Constant(f64),
    Buffer(core::slice::Iter<'a, Sample>),
}

impl<'a> Iterator for InputIter<'a> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Constant(v) => Some(*v),
            Self::Buffer(v) => v.next().copied(),
        }
    }
}

#[allow(dead_code)] // TODO
pub struct Buffers<'a, const B: usize> {
    buffers: &'a BufferMap,
    keys: &'a [BufferKey; B],
}

pub struct StaticNode<const I: usize, const B: usize, P: Node<I, B>> {
    inputs: [graph::Input<f64>; I],
    buffers: [BufferKey; B],
    output: Output,
    processor: P,
}

impl<const I: usize, const B: usize, P: Node<I, B>> StaticNode<I, B, P> {
    #[inline]
    pub fn new(processor: P) -> Self {
        let defaults = P::DEFAULTS;

        let mut inputs = [graph::Input::Value(0.0); I];

        for (from, to) in defaults.iter().zip(inputs.iter_mut()) {
            *to = graph::Input::Value(*from);
        }

        Self {
            inputs,
            buffers: [BufferKey::default(); B],
            output: [0.0; LEN],
            processor,
        }
    }
}

impl<const I: usize, const B: usize, P: Node<I, B>> graph::Processor<Config>
    for StaticNode<I, B, P>
{
    fn set(
        &mut self,
        param: Parameter,
        value: graph::Input<Value>,
    ) -> Result<graph::Input<Value>, u64> {
        let value = match value {
            graph::Input::Value(Value::Buffer(idx)) => {
                let input = self.buffers.get_mut(param as usize).ok_or(param)?;
                let prev = core::mem::replace(input, idx);
                return Ok(graph::Input::Value(Value::Buffer(prev)));
            }
            graph::Input::Value(Value::Constant(v)) => {
                self.processor.trigger(param, v);
                graph::Input::Value(v)
            }
            graph::Input::Node(node) => {
                // check to see if this is a trigger
                if self.processor.trigger(param, 0.0) {
                    return Err(param);
                }
                graph::Input::Node(node)
            }
        };

        let input = self.inputs.get_mut(param as usize).ok_or(param)?;
        let prev = core::mem::replace(input, value);

        Ok(match prev {
            graph::Input::Value(v) => graph::Input::Value(Value::Constant(v)),
            graph::Input::Node(n) => graph::Input::Node(n),
        })
    }

    fn remove(&mut self, node: graph::NodeKey) {
        for input in self.inputs.iter_mut() {
            if let graph::Input::Node(key) = input {
                if *key == node {
                    *input = graph::Input::Value(0.0);
                }
            }
        }
    }

    fn output(&self) -> &Output {
        &self.output
    }

    fn output_mut(&mut self) -> &mut Output {
        &mut self.output
    }

    fn process(&mut self, inputs: graph::Inputs<Config>, context: &Context) {
        let inputs = Inputs {
            inputs,
            keys: &self.inputs,
        };

        let buffers = Buffers {
            buffers: &context.buffers,
            keys: &self.buffers,
        };

        if let Some(partial) = context.partial {
            let output = unsafe {
                debug_assert!(partial <= LEN);
                self.output.get_unchecked_mut(..partial)
            };
            self.processor.process(inputs, buffers, output);
        } else {
            self.processor
                .process_full(inputs, buffers, &mut self.output);
        }
    }
}

pub trait Node<const INPUTS: usize, const BUFFERS: usize>: 'static + Send {
    const DEFAULTS: [f64; INPUTS] = [0.0; INPUTS];

    #[inline]
    fn trigger(&mut self, param: Parameter, value: f64) -> bool {
        // no op
        let _ = param;
        let _ = value;
        false
    }

    #[inline]
    fn process_full(
        &mut self,
        inputs: Inputs<INPUTS>,
        buffers: Buffers<BUFFERS>,
        output: &mut Output,
    ) {
        self.process(inputs, buffers, output)
    }

    fn process(&mut self, inputs: Inputs<INPUTS>, buffers: Buffers<BUFFERS>, output: &mut [Sample]);
}
