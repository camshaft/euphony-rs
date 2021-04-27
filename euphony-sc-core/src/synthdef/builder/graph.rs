use super::{
    compiler::{Compiler, Inputs},
    value::{Output, Value, ValueVec},
    UgenSpec,
};
use crate::synthdef::CalculationRate;

#[derive(Debug)]
pub struct Node {
    pub spec: UgenSpec,
    pub inputs: Vec<ValueVec>,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Default for Graph {
    fn default() -> Self {
        let node = Node {
            spec: UgenSpec {
                name: "Control",
                compile: compile_control,
                ..Default::default()
            },
            inputs: vec![],
        };
        Self { nodes: vec![node] }
    }
}

fn compile_control(spec: &UgenSpec, _inputs: &Inputs, compiler: &mut Compiler) {
    let params = compiler.params();

    let outs = params
        .iter()
        .map(|param| {
            // TODO lag?
            param.rate
        })
        .collect();

    let outputs = compiler.push_ugen(
        crate::synthdef::UGen {
            name: spec.name.into(),
            special_index: spec.special_index,
            rate: CalculationRate::Control,
            ins: vec![],
            outs,
        },
        spec.meta,
    );

    compiler.push_outputs(outputs);
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

impl OutputSpec for () {
    fn output(_builder: &mut OutputBuilder) -> Self {}
}

impl OutputSpec for Value {
    fn output(builder: &mut OutputBuilder) -> Self {
        builder.output()
    }
}

impl<const N: usize> OutputSpec for [Value; N] {
    fn output(builder: &mut OutputBuilder) -> Self {
        let mut out = [Value::from(0); N];
        for v in out.iter_mut() {
            *v = builder.output();
        }
        out
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
