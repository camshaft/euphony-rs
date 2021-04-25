use super::{
    value::{Output, Value, ValueVec, Variant},
    UgenSpec,
};
use crate::synthdef::{self, CalculationRate};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
struct Node {
    spec: UgenSpec,
    inputs: Vec<ValueVec>,
}
type Input = (u32, u32);

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Default for Graph {
    fn default() -> Self {
        let node = Node {
            spec: UgenSpec {
                name: "Control",
                special_index: 0,
                rate: |_| CalculationRate::Control,
                outputs: 0,
                output_rate: |_, _| CalculationRate::Control,
            },
            inputs: vec![],
        };
        Self { nodes: vec![node] }
    }
}

impl Graph {
    pub fn insert(&mut self, spec: UgenSpec) -> Ugen {
        let id = self.nodes.len() as u32;

        let node = Node {
            spec,
            inputs: vec![],
        };
        self.nodes.push(node);

        Ugen {
            node: &mut self.nodes[id as usize],
            ugen: id,
        }
    }

    pub fn build(mut self, params: usize) -> (Vec<f32>, Vec<synthdef::UGen>) {
        self.nodes[0].spec.outputs = params;

        let mut consts = ConstIds::default();

        // TODO multichannel expansion

        for node in &self.nodes {
            for input in &node.inputs {
                for input in input.iter() {
                    let _input: synthdef::Input = match input.as_variant() {
                        Variant::Const(v) => consts.get(v),
                        Variant::Output(v) => v.into(),
                        Variant::Parameter(v) => v.into(),
                    };
                }
            }
        }

        let consts = consts.values;
        let ugens = vec![];
        (consts, ugens)
    }
}

#[derive(Default)]
struct ConstIds {
    values: Vec<f32>,
    ids: HashMap<u32, i32>,
}

impl ConstIds {
    pub fn get(&mut self, value: f32) -> synthdef::Input {
        match self.ids.entry(value.to_bits()) {
            Entry::Vacant(entry) => {
                let index = self.values.len() as i32;
                self.values.push(value);
                entry.insert(index);
                synthdef::Input::Constant { index }
            }
            Entry::Occupied(entry) => synthdef::Input::Constant {
                index: *entry.get(),
            },
        }
    }
}

pub struct Ugen<'a> {
    node: &'a mut Node,
    ugen: u32,
}

impl<'a> Ugen<'a> {
    pub fn input(&mut self, value: impl Into<ValueVec>) {
        self.node.inputs.push(value.into());
    }

    pub fn finish<T: OutputSpec>(self) -> T {
        let ugen = self.ugen;
        let count = self.node.spec.outputs as u32;
        let mut builder = OutputBuilder {
            ugen,
            output: 0,
            count,
        };
        let outputs = T::output(&mut builder);
        debug_assert_eq!(builder.output, count);
        outputs
    }
}

pub struct OutputBuilder {
    ugen: u32,
    output: u32,
    count: u32,
}

impl OutputBuilder {
    pub fn count(&self) -> usize {
        self.count as usize
    }

    pub fn output(&mut self) -> Value {
        let id = self.output;
        self.output += 1;
        Output::new(self.ugen, id).into()
    }
}

pub trait OutputSpec {
    fn output(builder: &mut OutputBuilder) -> Self;
}

impl OutputSpec for Value {
    fn output(builder: &mut OutputBuilder) -> Self {
        builder.output()
    }
}

impl OutputSpec for ValueVec {
    fn output(builder: &mut OutputBuilder) -> Self {
        let count = builder.count();
        (0..count).map(|_| builder.output()).collect()
    }
}

impl OutputSpec for (Value, Value) {
    fn output(builder: &mut OutputBuilder) -> Self {
        let a = builder.output();
        let b = builder.output();
        (a, b)
    }
}

impl OutputSpec for (Value, Value, Value) {
    fn output(builder: &mut OutputBuilder) -> Self {
        let a = builder.output();
        let b = builder.output();
        let c = builder.output();
        (a, b, c)
    }
}

impl OutputSpec for (Value, Value, Value, Value) {
    fn output(builder: &mut OutputBuilder) -> Self {
        let a = builder.output();
        let b = builder.output();
        let c = builder.output();
        let d = builder.output();
        (a, b, c, d)
    }
}
