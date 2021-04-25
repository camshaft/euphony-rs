use crate::osc::control;
use core::fmt;
use smallvec::SmallVec;

#[derive(Clone, Copy)]
pub struct Value(u32, u32);

impl Value {
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

    pub fn from_output(value: Output) -> Self {
        Variant::Output(value).as_value()
    }

    pub(super) fn as_variant(self) -> Variant {
        match self.0 {
            v if v == u32::MAX - 1 => Variant::Parameter(Parameter(self.1)),
            u32::MAX => Variant::Const(f32::from_bits(self.1)),
            _ => Variant::Output(Output(self.0, self.1)),
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

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::from_i32(if v { 1 } else { 0 })
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

impl From<Output> for Value {
    fn from(value: Output) -> Self {
        Value::from_output(value)
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

#[derive(Clone, Debug)]
pub struct ValueVec(SmallVec<[Value; 1]>);

impl ValueVec {
    pub fn iter(&self) -> impl Iterator<Item = Value> + '_ {
        self.0.iter().copied()
    }
}

impl From<bool> for ValueVec {
    fn from(value: bool) -> Self {
        Self::from(Value::from(value))
    }
}

impl From<i32> for ValueVec {
    fn from(value: i32) -> Self {
        Self::from(Value::from(value))
    }
}

impl From<f32> for ValueVec {
    fn from(value: f32) -> Self {
        Self::from(Value::from(value))
    }
}

impl From<Value> for ValueVec {
    fn from(value: Value) -> Self {
        Self(SmallVec::from_elem(value, 1))
    }
}

impl From<Vec<Value>> for ValueVec {
    fn from(value: Vec<Value>) -> Self {
        Self(SmallVec::from_vec(value))
    }
}

impl From<Vec<bool>> for ValueVec {
    fn from(value: Vec<bool>) -> Self {
        Self(value.into_iter().map(Value::from).collect())
    }
}

impl From<Vec<i32>> for ValueVec {
    fn from(value: Vec<i32>) -> Self {
        Self(value.into_iter().map(Value::from).collect())
    }
}

impl From<Vec<f32>> for ValueVec {
    fn from(value: Vec<f32>) -> Self {
        Self(value.into_iter().map(Value::from).collect())
    }
}

impl<const N: usize> From<[Value; N]> for ValueVec {
    fn from(value: [Value; N]) -> Self {
        Self(SmallVec::from_vec(value.to_vec()))
    }
}

impl<const N: usize> From<[bool; N]> for ValueVec {
    fn from(value: [bool; N]) -> Self {
        Self(value.iter().copied().map(Value::from).collect())
    }
}

impl<const N: usize> From<[i32; N]> for ValueVec {
    fn from(value: [i32; N]) -> Self {
        Self(value.iter().copied().map(Value::from).collect())
    }
}

impl<const N: usize> From<[f32; N]> for ValueVec {
    fn from(value: [f32; N]) -> Self {
        Self(value.iter().copied().map(Value::from).collect())
    }
}

impl core::iter::FromIterator<Value> for ValueVec {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Variant {
    Const(f32),
    Parameter(Parameter),
    Output(Output),
}

impl Variant {
    pub fn as_value(self) -> Value {
        match self {
            Self::Const(v) => Value(u32::MAX, v.to_bits()),
            Self::Parameter(v) => Value(u32::MAX - 1, v.0),
            Self::Output(v) => Value(v.0, v.1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Parameter(u32);

impl Parameter {
    pub const fn new(idx: u32) -> Self {
        Self(idx)
    }
}

impl From<Parameter> for crate::synthdef::Input {
    fn from(value: Parameter) -> Self {
        Self::UGen {
            index: 0, // inputs are always ugen 0
            output: value.0 as _,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Output(u32, u32);

impl Output {
    pub(crate) const fn new(ugen: u32, output: u32) -> Self {
        Self(output, ugen)
    }
}

impl From<Output> for crate::synthdef::Input {
    fn from(value: Output) -> Self {
        Self::UGen {
            index: value.1 as _,
            output: value.0 as _,
        }
    }
}
