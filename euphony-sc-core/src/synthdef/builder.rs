#![allow(dead_code)]

use crate::{osc::control, param::Param, synthdef::CalculationRate};
use core::{fmt, marker::PhantomData, ops};

#[derive(Clone, Copy)]
pub struct Value(u32, u32);

impl Value {
    pub fn calculation_rate(&self) -> CalculationRate {
        self.as_variant().calculation_rate()
    }

    pub fn from_f32(value: f32) -> Self {
        Variant::Const(value).as_value()
    }

    pub fn from_i32(value: i32) -> Self {
        Self::from_f32(value as _)
    }

    pub fn from_parameter(value: Parameter) -> Self {
        Variant::Parameter(value).as_value()
    }

    pub fn from_parameter_id(value: u32) -> Self {
        Variant::Parameter(Parameter(value)).as_value()
    }

    pub fn from_ugen(value: Ugen) -> Self {
        Variant::Ugen(value).as_value()
    }

    pub fn as_variant(self) -> Variant {
        match self.0 {
            v if v == u32::MAX - 1 => Variant::Parameter(Parameter(self.1)),
            u32::MAX => Variant::Const(f32::from_bits(self.1)),
            _ => Variant::Ugen(Ugen(self.0, self.1)),
        }
    }

    pub fn as_osc(self) -> Option<control::Value> {
        match self.as_variant() {
            Variant::Const(value) => Some(control::Value::Float(value)),
            _ => None,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_variant().fmt(f)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::from_i32(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::from_f32(v)
    }
}

impl From<Parameter> for Value {
    fn from(param: Parameter) -> Self {
        Value::from_parameter(param)
    }
}

impl From<Ugen> for Value {
    fn from(ugen: Ugen) -> Self {
        Value::from_ugen(ugen)
    }
}

impl From<Value> for Variant {
    fn from(value: Value) -> Self {
        value.as_variant()
    }
}

impl From<Variant> for Value {
    fn from(value: Variant) -> Self {
        value.as_value()
    }
}

impl From<control::Value> for Value {
    fn from(value: control::Value) -> Self {
        match value {
            control::Value::Int(v) => v.into(),
            control::Value::Float(v) => v.into(),
        }
    }
}

impl<T> ops::Add<T> for Value
where
    T: Into<Value>,
{
    type Output = Value;

    fn add(self, _rhs: T) -> Self::Output {
        // let rhs = rhs.into();
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Variant {
    Const(f32),
    Parameter(Parameter),
    Ugen(Ugen),
}

impl Variant {
    pub const fn calculation_rate(&self) -> CalculationRate {
        match self {
            Self::Const(_) => CalculationRate::Scalar,
            Self::Parameter(v) => v.calculation_rate(),
            Self::Ugen(v) => v.calculation_rate(),
        }
    }

    pub fn as_value(self) -> Value {
        match self {
            Self::Const(v) => Value(u32::MAX, v.to_bits()),
            Self::Parameter(v) => Value(u32::MAX - 1, v.0),
            Self::Ugen(v) => Value(v.0, v.1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Parameter(u32);

impl Parameter {
    pub const fn new(idx: u32) -> Self {
        Self(idx)
    }

    pub const fn calculation_rate(self) -> CalculationRate {
        CalculationRate::Control
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ugen(u32, u32);

impl Ugen {
    pub const fn calculation_rate(self) -> CalculationRate {
        match self.0 >> 30 {
            0b00 => CalculationRate::Scalar,
            0b01 => CalculationRate::Control,
            0b10 => CalculationRate::Audio,
            _ => CalculationRate::Demand,
        }
    }
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

struct UgenSpec {
    name: &'static str,
    rate: CalculationRate,
    inputs: Vec<Vec<Value>>,
    outputs: Vec<CalculationRate>,
    special_index: i16,
}

#[derive(Default)]
struct Context {
    params: Vec<(&'static str, f32)>,
    ugens: Vec<UgenSpec>,
}

impl Context {
    fn build(&mut self, name: &'static str) -> SynthDesc {
        // TODO
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

    fn ugen(&mut self, ugen: UgenSpec) -> Ugen {
        let id = self.ugens.len() as _;
        let idx = 0;
        self.ugens.push(ugen);
        Ugen(idx, id)
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
    let v: Value = Parameter::new(id).into();
    v.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_size_test() {
        assert_eq!(core::mem::size_of::<Value>(), 8);
    }
}
