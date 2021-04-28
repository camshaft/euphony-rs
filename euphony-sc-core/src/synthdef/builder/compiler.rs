use super::{graph::Graph, value::Variant, UgenSpec};
use crate::synthdef::{self, CalculationRate, Input, UGen};
use core::ops::{self, Range};
use smallvec::SmallVec;
use std::collections::{hash_map::Entry, HashMap};

pub type Compile = fn(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler);

#[derive(Debug)]
pub struct Inputs<'a> {
    inputs: SmallVec<[SmallVec<[Input; 2]>; 2]>,
    ranges: &'a Vec<Range<usize>>,
    outputs: &'a Vec<InputVec>,
}

impl<'a> Inputs<'a> {
    pub fn expand(&self) -> Expand {
        Expand::new(self)
    }

    pub fn iter(&self) -> impl Iterator<Item = SmallVec<[InputVec; 2]>> + '_ {
        let ranges = &self.ranges;
        let outputs = &self.outputs;

        self.inputs.iter().map(move |input| {
            let mut input_set = SmallVec::new();
            for input in input.iter().copied() {
                let mut out = InputVec::new();
                match input {
                    Input::UGen { index, output } => {
                        let range = ranges[index as usize].clone();
                        for o in &outputs[range] {
                            out.push(o[output as usize]);
                        }
                    }
                    Input::Constant { .. } => {
                        let info = InputInfo {
                            input,
                            rate: CalculationRate::Scalar,
                        };
                        out.push(info);
                    }
                }
                input_set.push(out);
            }
            input_set
        })
    }
}

/*
impl ops::Deref for Inputs {
    type Target = [InputVec];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}
*/

#[derive(Debug)]
pub struct Expand {
    len: usize,
    inputs: SmallVec<[InputVec; 2]>,
}

impl Expand {
    fn new(inputs: &Inputs) -> Self {
        let mut len = 0;
        for input in &inputs.inputs {
            let input_len = input
                .iter()
                .map(|input| match input {
                    Input::UGen { index, .. } => {
                        let range = inputs.ranges[*index as usize].clone();
                        let outs = &inputs.outputs[range];
                        outs.len()
                    }
                    Input::Constant { .. } => 1,
                })
                .sum();

            // having an empty input cancels the whole thing
            if input_len == 0 {
                return Self {
                    len: 0,
                    inputs: Default::default(),
                };
            }

            len = len.max(input_len);
        }

        let inputs = inputs
            .inputs
            .iter()
            .map(move |input| {
                input
                    .iter()
                    .copied()
                    .flat_map(|input| {
                        let mut out = InputVec::new();
                        match input {
                            Input::UGen { index, output } => {
                                let range = inputs.ranges[index as usize].clone();
                                for o in &inputs.outputs[range] {
                                    out.push(o[output as usize]);
                                }
                            }
                            Input::Constant { .. } => {
                                let info = InputInfo {
                                    input,
                                    rate: CalculationRate::Scalar,
                                };
                                out.push(info);
                            }
                        }
                        out
                    })
                    .cycle()
                    .take(len)
                    .collect()
            })
            .collect();

        Self { len, inputs }
    }

    pub fn iter(&self) -> impl Iterator<Item = Expansion> + '_ {
        let len = self.len;
        (0..len).map(move |idx| Expansion { idx, inputs: self })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Debug)]
pub struct Expansion<'a> {
    idx: usize,
    inputs: &'a Expand,
}

impl<'a> Expansion<'a> {
    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = InputInfo> + '_ {
        let inputs = &self.inputs;
        let idx = self.idx;
        inputs.inputs.iter().map(move |input| input[idx])
    }

    pub fn rate(&self) -> Option<CalculationRate> {
        self.iter().map(|input| input.rate).max()
    }

    pub fn inputs(&self) -> impl Iterator<Item = Input> + '_ {
        self.iter().map(|input| input.input)
    }
}

impl<'a> ops::Index<usize> for Expansion<'a> {
    type Output = (Input, CalculationRate);

    fn index(&self, _index: usize) -> &Self::Output {
        /*
        let input = &self.inputs[index];
        let idx = self.idx % input.len();
        &input[idx]
        */
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputInfo {
    pub input: Input,
    pub rate: CalculationRate,
}

pub type InputVec = SmallVec<[InputInfo; 2]>;

#[derive(Copy, Clone, Debug)]
pub struct ParamSpec {
    pub name: &'static str,
    pub default: f32,
    pub rate: CalculationRate,
    pub lag: f32,
}

impl Default for ParamSpec {
    fn default() -> Self {
        Self {
            name: "",
            default: 0.0,
            rate: CalculationRate::Control,
            lag: 0.0,
        }
    }
}

pub fn compile(graph: &Graph, params: &[ParamSpec]) -> (Vec<f32>, Vec<synthdef::UGen>) {
    let mut consts = ConstIds::default();
    let mut ugens = vec![];
    let mut meta = vec![];
    let mut uses = vec![];
    let mut outputs: Vec<InputVec> = vec![];
    let mut ranges: Vec<Range<usize>> = vec![];

    for node in graph.nodes.iter() {
        let mut inputs: SmallVec<_> = node
            .inputs
            .iter()
            .map(|value_vec| {
                let mut inputs = SmallVec::new();
                for value in value_vec.iter() {
                    let out = match value.as_variant() {
                        Variant::Const(v) => consts.get_input(v),
                        Variant::Output(v) => Input::UGen {
                            index: v.ugen() as _,
                            output: v.output() as _,
                        },
                        Variant::Parameter(v) => Input::UGen {
                            index: 0,
                            output: v.0 as _,
                        },
                    };
                    inputs.push(out);
                }
                inputs
            })
            .collect();

        // make sure each set of inputs has an instantiation
        if inputs.is_empty() {
            inputs.push(SmallVec::new());
        }

        let mut compiler_outputs = vec![];
        let range_start = outputs.len();

        let mut compiler = Compiler {
            params,
            ugens: &mut ugens,
            uses: &mut uses,
            meta: &mut meta,
            outputs: &mut compiler_outputs,
            consts: &mut consts,
        };

        let inputs = Inputs {
            inputs,
            ranges: &ranges,
            outputs: &outputs,
        };

        (node.spec.compile)(&node.spec, &inputs, &mut compiler);

        outputs.extend(compiler_outputs);
        ranges.push(range_start..outputs.len());
    }

    if false {
        let consts = consts.values;
        return (consts, ugens);
    }

    // dead code eliminate
    for (idx, ugen) in ugens.iter().enumerate().rev() {
        if uses[idx] == 0 && meta[idx].is_pure {
            for input in &ugen.ins {
                match input {
                    Input::Constant { .. } => {}
                    Input::UGen { index, .. } => {
                        uses[*index as usize] -= 1;
                    }
                }
            }
        }
    }

    let prev_consts = core::mem::take(&mut consts.values);
    consts.ids.clear();

    let mut optimized = vec![];
    let mut optimized_ids = vec![usize::MAX; ugens.len()];
    for (idx, mut ugen) in ugens.drain(..).enumerate() {
        if uses[idx] == 0 && meta[idx].is_pure {
            continue;
        }

        for input in ugen.ins.iter_mut() {
            match input {
                Input::Constant { index } => {
                    let value = prev_consts[*index as usize];
                    *index = consts.get(value);
                }
                Input::UGen { index, .. } => {
                    let id = optimized_ids[*index as usize];
                    debug_assert_ne!(id, usize::MAX, "using non-existant ugen");
                    *index = id as i32;
                }
            }
        }

        optimized_ids[idx] = optimized.len();
        optimized.push(ugen);
    }

    let consts = consts.values;
    (consts, optimized)
}

pub struct Compiler<'a> {
    params: &'a [ParamSpec],
    uses: &'a mut Vec<usize>,
    ugens: &'a mut Vec<UGen>,
    meta: &'a mut Vec<UgenMeta>,
    outputs: &'a mut Vec<InputVec>,
    consts: &'a mut ConstIds,
}

impl<'a> Compiler<'a> {
    pub fn params(&self) -> &'a [ParamSpec] {
        self.params
    }

    pub fn push_ugen(&mut self, ugen: crate::synthdef::UGen, meta: UgenMeta) -> InputVec {
        let index = self.ugens.len() as i32;

        debug_assert!(!ugen.name.is_empty(), "tried to push an unnamed ugen");

        // notify upstream ugens that they're being used
        for input in &ugen.ins {
            match input {
                Input::Constant { .. } => {}
                Input::UGen { index, output } => {
                    let index = *index as usize;
                    self.uses[index] += 1;
                    debug_assert!(
                        self.ugens[index].outs.len() > *output as usize,
                        "tried to use a non-existant output {:#?} on {:#?}",
                        input,
                        &self.ugens[index],
                    );
                }
            }
        }

        let outputs = ugen
            .outs
            .iter()
            .copied()
            .enumerate()
            .map(|(output, rate)| {
                let output = output as i32;
                let input = Input::UGen { index, output };
                InputInfo { input, rate }
            })
            .collect();

        self.ugens.push(ugen);
        self.uses.push(0);
        self.meta.push(meta);

        outputs
    }

    pub fn push_const(&mut self, value: f32) -> InputInfo {
        let output = self.consts.get_input(value);
        InputInfo {
            input: output,
            rate: CalculationRate::Scalar,
        }
    }

    pub fn push_consts(&mut self, values: &[f32]) -> InputVec {
        values
            .iter()
            .copied()
            .map(|value| self.push_const(value))
            .collect()
    }

    pub fn push_output(&mut self, output: InputInfo) {
        self.push_outputs(SmallVec::from_elem(output, 1));
    }

    pub fn push_outputs(&mut self, outputs: InputVec) {
        self.outputs.push(outputs);
    }

    pub fn resolve(&self, input: Input) -> Dependency {
        match input {
            Input::Constant { index } => Dependency::Const(self.consts.values[index as usize]),
            Input::UGen { index, output } => Dependency::Ugen {
                ugen: &self.ugens[index as usize],
                output,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UgenMeta {
    /// Is this UGen free from side-effects (e.g. writing to a buffer, bus, etc)
    pub is_pure: bool,
    /// Does calling this UGen with the same arguments produce the same result?
    pub is_deterministic: bool,
}

impl Default for UgenMeta {
    fn default() -> Self {
        Self {
            is_pure: true,
            is_deterministic: true,
        }
    }
}

pub enum Dependency<'a> {
    Const(f32),
    Ugen {
        ugen: &'a crate::synthdef::UGen,
        output: i32,
    },
}

#[derive(Default)]
struct ConstIds {
    values: Vec<f32>,
    ids: HashMap<u32, i32>,
}

impl ConstIds {
    pub fn get_input(&mut self, value: f32) -> synthdef::Input {
        synthdef::Input::Constant {
            index: self.get(value),
        }
    }

    pub fn get(&mut self, value: f32) -> i32 {
        debug_assert!(!value.is_nan());

        match self.ids.entry(value.to_bits()) {
            Entry::Vacant(entry) => {
                let index = self.values.len() as i32;
                self.values.push(value);
                entry.insert(index);
                index
            }
            Entry::Occupied(entry) => *entry.get(),
        }
    }
}

pub fn default_compile(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    for expansion in inputs.expand().iter() {
        let rate = spec
            .rate
            .or_else(|| expansion.rate())
            .unwrap_or(CalculationRate::Control);

        let ugen = synthdef::UGen {
            name: spec.name.into(),
            special_index: spec.special_index,
            rate,
            ins: expansion.inputs().collect(),
            outs: (0..spec.outputs).map(|_| rate).collect(),
        };

        let outputs = compiler.push_ugen(ugen, spec.meta);
        compiler.push_outputs(outputs);
    }
}
