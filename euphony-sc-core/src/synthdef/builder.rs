#![allow(dead_code)]

use crate::{
    param::Param,
    synthdef::{BinaryOp, CalculationRate, UnaryOp},
};
use core::{fmt, marker::PhantomData};

mod graph;
mod ops;
mod value;

pub use graph::{OutputSpec, Ugen};
pub use value::{Value, ValueVec};

pub fn unary_op(op: UnaryOp, value: ValueVec) -> Value {
    ugen(
        UgenSpec {
            name: "UnaryOpGen",
            special_index: op as _,
            ..Default::default()
        },
        |mut ugen| {
            ugen.input(value);
            ugen.finish()
        },
    )
}

pub fn binary_op(op: BinaryOp, lhs: ValueVec, rhs: ValueVec) -> Value {
    ugen(
        UgenSpec {
            name: "BinaryOpGen",
            special_index: op as _,
            ..Default::default()
        },
        |mut ugen| {
            ugen.input(lhs);
            ugen.input(rhs);
            ugen.finish()
        },
    )
}

pub trait Parameters: 'static + Sized {
    type Desc;

    fn new<F>(f: F) -> Self
    where
        F: FnOnce(fn() -> Self::Desc) -> SynthDescRef;
}

#[derive(Clone)]
pub struct Synth {
    id: crate::osc::node::Id,
    track: crate::track::Handle,
    synthdef: &'static str,
}

impl Synth {
    pub fn new(
        id: crate::osc::node::Id,
        track: crate::track::Handle,
        synthdef: &'static str,
    ) -> Self {
        Self {
            id,
            track,
            synthdef,
        }
    }

    pub fn name(&self) -> &'static str {
        self.synthdef
    }

    pub fn id(&self) -> crate::osc::node::Id {
        self.id
    }

    pub fn track_name(&self) -> &str {
        use crate::track::Track;
        self.track.name()
    }
}

impl fmt::Debug for Synth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Synth")
            .field("id", &self.id)
            .field("track", &self.track_name())
            .field("synthdef", &self.synthdef)
            .finish()
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        use crate::track::Track;
        self.track.free(self.id)
    }
}

pub struct SynthDesc {
    name: String,
    desc: Vec<u8>,
}

impl SynthDesc {
    pub fn as_ref(&'static self) -> SynthDescRef {
        SynthDescRef {
            name: &self.name,
            desc: &self.desc,
        }
    }
}

impl fmt::Debug for SynthDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthDesc")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct SynthDescRef {
    name: &'static str,
    desc: &'static [u8],
}

impl fmt::Debug for SynthDescRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthDesc")
            .field("name", &self.name)
            .finish()
    }
}

pub struct SynthDef<Params> {
    desc: SynthDescRef,
    params: PhantomData<Params>,
}

impl<Params> SynthDef<Params> {
    pub fn new(desc: SynthDescRef) -> Self {
        Self {
            desc,
            params: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        &self.desc.name
    }

    pub fn desc(&self) -> &'static [u8] {
        &self.desc.desc
    }
}

impl<Params> fmt::Debug for SynthDef<Params> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthDef")
            .field("name", &self.name())
            .finish()
    }
}

use std::cell::RefCell;

pub struct UgenSpec {
    pub name: &'static str,
    pub special_index: i16,
    pub rate: fn(inputs: &[CalculationRate]) -> CalculationRate,
    pub outputs: usize,
    pub output_rate: fn(calculation_rate: CalculationRate, idx: usize) -> CalculationRate,
    // TODO expand callback
}

impl fmt::Debug for UgenSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UgenSpec")
            .field("name", &self.name)
            .field("special_index", &self.special_index)
            .finish()
    }
}

impl Default for UgenSpec {
    fn default() -> Self {
        Self {
            name: "",
            special_index: 0,
            rate: |inputs| {
                inputs
                    .iter()
                    .copied()
                    .max()
                    .unwrap_or(CalculationRate::Scalar)
            },
            outputs: 1,
            output_rate: |rate, _idx| rate,
        }
    }
}

#[derive(Debug, Default)]
struct Context {
    params: Vec<(&'static str, f32)>,
    graph: graph::Graph,
}

impl Context {
    fn build(&mut self, name: &'static str) -> SynthDesc {
        let this = core::mem::take(self);

        let (consts, ugens) = this.graph.build(self.params.len());

        let params = this.params.iter().map(|(_, v)| *v).collect();

        let param_names = this
            .params
            .iter()
            .enumerate()
            .map(|(index, (name, _))| crate::synthdef::ParamName {
                name: name.to_string(),
                index: index as _,
            })
            .collect();

        let definition = crate::synthdef::Definition {
            name: name.to_string(),
            consts,
            params,
            param_names,
            ugens,
            variants: vec![],
        };

        let container = crate::synthdef::Container {
            version: crate::synthdef::V2 as _,
            defs: vec![definition],
        };

        dbg!(container);

        SynthDesc {
            name: name.to_owned(),
            desc: vec![],
        }
    }

    fn param(&mut self, id: u32, name: &'static str, default: f32) -> Param {
        let idx = id as usize;
        if idx >= self.params.len() {
            self.params.resize(idx + 1, ("", 0.0));
        }
        self.params[idx] = (name, default);
        param_instance(id)
    }

    fn ugen(&mut self, ugen: UgenSpec) -> graph::Ugen {
        self.graph.insert(ugen)
    }
}

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new(Context::default());
}

pub fn synthdef<F: FnOnce()>(name: &'static str, b: F) -> SynthDesc {
    b();
    CONTEXT.with(|c| c.take()).build(name)
}

pub fn external_synthdef(name: &'static str, desc: &'static [u8]) -> SynthDescRef {
    SynthDescRef { name, desc }
}

pub fn param(id: u32, name: &'static str, default: f32) -> Param {
    CONTEXT.with(|c| c.borrow_mut().param(id, name, default))
}

pub fn param_instance(id: u32) -> Param {
    let v: Value = value::Parameter::new(id).into();
    v.into()
}

pub fn ugen<F: FnOnce(Ugen) -> O, O>(ugen: UgenSpec, def: F) -> O {
    CONTEXT.with(|c| def(c.borrow_mut().ugen(ugen)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_size_test() {
        assert_eq!(core::mem::size_of::<Value>(), 8);
    }
}
