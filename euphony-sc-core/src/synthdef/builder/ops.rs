use super::{ugen, Compile, Compiler, Dependency, InputInfo, Inputs, UgenSpec, Value, ValueVec};
use crate::synthdef::{BinaryOp, CalculationRate, Input, UGen, UnaryOp};
use core::ops;

macro_rules! binary {
    ($op:ident, $name:ident, $call:ident, $assign:ident, $assign_call:ident, $optimize:ident) => {
        impl ops::$name<Value> for i32 {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl ops::$name<ValueVec> for i32 {
            type Output = Value;

            fn $call(self, rhs: ValueVec) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for f32 {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl ops::$name<ValueVec> for f32 {
            type Output = Value;

            fn $call(self, rhs: ValueVec) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for Vec<Value> {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for Vec<i32> {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for Vec<f32> {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl<const N: usize> ops::$name<Value> for [Value; N] {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl<const N: usize> ops::$name<Value> for [i32; N] {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl<const N: usize> ops::$name<Value> for [f32; N] {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl<T> ops::$name<T> for Value
        where
            T: Into<ValueVec>,
        {
            type Output = Value;

            fn $call(self, rhs: T) -> Self::Output {
                $call(self.into(), rhs.into())
            }
        }

        impl<T> ops::$name<T> for ValueVec
        where
            T: Into<ValueVec>,
        {
            type Output = Value;

            fn $call(self, rhs: T) -> Self::Output {
                $call(self, rhs.into())
            }
        }

        impl<T> ops::$assign<T> for Value
        where
            T: Into<ValueVec>,
        {
            fn $assign_call(&mut self, rhs: T) {
                *self = $call(ValueVec::from(*self), rhs.into());
            }
        }

        fn $call(lhs: ValueVec, rhs: ValueVec) -> Value {
            fn compile(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
                for expansion in inputs.expand().iter() {
                    let rate = spec
                        .rate
                        .or_else(|| expansion.rate())
                        .unwrap_or(CalculationRate::Control);

                    let mut inputs = expansion.inputs();
                    let lhs = inputs.next().unwrap();
                    let rhs = inputs.next().unwrap();
                    let output = $optimize(lhs, rhs, rate, compiler);
                    compiler.push_output(output);
                }
            }

            binary_op(BinaryOp::$op, lhs, rhs, compile)
        }
    };
}

pub fn fold_constant(
    rhs: Input,
    lhs: Input,
    op: impl FnOnce(f32, f32) -> f32,
    compiler: &mut Compiler,
) -> Option<InputInfo> {
    if let (Dependency::Const(lhs), Dependency::Const(rhs)) =
        (compiler.resolve(lhs), compiler.resolve(rhs))
    {
        Some(compiler.push_const(op(lhs, rhs)))
    } else {
        None
    }
}

binary!(Add, Add, add, AddAssign, add_assign, compile_add);
pub fn compile_add(
    lhs: Input,
    rhs: Input,
    rate: CalculationRate,
    compiler: &mut Compiler,
) -> InputInfo {
    if let Some(out) = fold_constant(rhs, lhs, |lhs, rhs| lhs + rhs, compiler) {
        return out;
    }

    match (compiler.resolve(lhs), compiler.resolve(rhs)) {
        (Dependency::Const(lhs), _) if float_cmp(lhs, 0.0) => InputInfo { input: rhs, rate },
        (_, Dependency::Const(rhs)) if float_cmp(rhs, 0.0) => InputInfo { input: lhs, rate },
        (Dependency::Ugen { ugen: lhs, .. }, _)
            if lhs.name == "BinaryOpUGen" && lhs.special_index == BinaryOp::Add as i16 =>
        {
            let mut ins = lhs.ins.clone();
            ins.push(rhs);
            let ugen = UGen {
                name: "Sum3".into(),
                rate,
                ins,
                outs: vec![rate],
                special_index: 0,
            };
            compiler.push_ugen(ugen, Default::default()).pop().unwrap()
        }
        (_, Dependency::Ugen { ugen: rhs, .. })
            if rhs.name == "BinaryOpUGen" && rhs.special_index == BinaryOp::Add as i16 =>
        {
            let mut ins = rhs.ins.clone();
            ins.push(lhs);
            let ugen = UGen {
                name: "Sum3".into(),
                rate,
                ins,
                outs: vec![rate],
                special_index: 0,
            };
            compiler.push_ugen(ugen, Default::default()).pop().unwrap()
        }
        _ => {
            let ugen = UGen {
                name: "BinaryOpUGen".into(),
                rate,
                ins: vec![lhs, rhs],
                outs: vec![rate],
                special_index: BinaryOp::Add as _,
            };
            compiler.push_ugen(ugen, Default::default()).pop().unwrap()
        }
    }
}

binary!(Subtract, Sub, sub, SubAssign, sub_assign, compile_sub);
pub fn compile_sub(
    lhs: Input,
    rhs: Input,
    rate: CalculationRate,
    compiler: &mut Compiler,
) -> InputInfo {
    if let Some(out) = fold_constant(rhs, lhs, |lhs, rhs| lhs - rhs, compiler) {
        return out;
    }

    // TODO optimize

    let ugen = UGen {
        name: "BinaryOpUGen".into(),
        rate,
        ins: vec![lhs, rhs],
        outs: vec![rate],
        special_index: BinaryOp::Subtract as _,
    };
    compiler.push_ugen(ugen, Default::default()).pop().unwrap()
}

binary!(Multiply, Mul, mul, MulAssign, mul_assign, compile_mul);
pub fn compile_mul(
    lhs: Input,
    rhs: Input,
    rate: CalculationRate,
    compiler: &mut Compiler,
) -> InputInfo {
    if let Some(out) = fold_constant(rhs, lhs, |lhs, rhs| lhs * rhs, compiler) {
        return out;
    }

    match (compiler.resolve(lhs), compiler.resolve(rhs)) {
        (Dependency::Const(lhs), _) if float_cmp(lhs, 1.0) => InputInfo { input: rhs, rate },
        (_, Dependency::Const(rhs)) if float_cmp(rhs, 1.0) => InputInfo { input: lhs, rate },
        _ => {
            let ugen = UGen {
                name: "BinaryOpUGen".into(),
                rate,
                ins: vec![lhs, rhs],
                outs: vec![rate],
                special_index: BinaryOp::Multiply as _,
            };
            compiler.push_ugen(ugen, Default::default()).pop().unwrap()
        }
    }
}

binary!(Divide, Div, div, DivAssign, div_assign, compile_div);
pub fn compile_div(
    lhs: Input,
    rhs: Input,
    rate: CalculationRate,
    compiler: &mut Compiler,
) -> InputInfo {
    if let Some(out) = fold_constant(rhs, lhs, |lhs, rhs| lhs / rhs, compiler) {
        return out;
    }

    // TODO optimize

    let ugen = UGen {
        name: "BinaryOpUGen".into(),
        rate,
        ins: vec![lhs, rhs],
        outs: vec![rate],
        special_index: BinaryOp::Divide as _,
    };
    compiler.push_ugen(ugen, Default::default()).pop().unwrap()
}

binary!(Modulus, Rem, rem, RemAssign, rem_assign, compile_rem);
pub fn compile_rem(
    lhs: Input,
    rhs: Input,
    rate: CalculationRate,
    compiler: &mut Compiler,
) -> InputInfo {
    if let Some(out) = fold_constant(rhs, lhs, |lhs, rhs| lhs % rhs, compiler) {
        return out;
    }

    let ugen = UGen {
        name: "BinaryOpUGen".into(),
        rate,
        ins: vec![lhs, rhs],
        outs: vec![rate],
        special_index: BinaryOp::Modulus as _,
    };
    compiler.push_ugen(ugen, Default::default()).pop().unwrap()
}

//binary!(And, BitAnd, bitand, BitAndAssign, bitand_assign);
//binary!(Or, BitOr, bitor, BitOrAssign, bitor_assign);
//binary!(Xor, BitXor, bitxor, BitXorAssign, bitxor_assign);

macro_rules! unary {
    ($name:ident, $op:ident, $call:ident) => {
        impl ops::$name for Value {
            type Output = Value;

            fn $call(self) -> Self::Output {
                unary_op(UnaryOp::$op, self.into())
            }
        }

        impl ops::$name for ValueVec {
            type Output = Value;

            fn $call(self) -> Self::Output {
                unary_op(UnaryOp::$op, self)
            }
        }
    };
}

unary!(Neg, Neg, neg);
unary!(Not, BitNot, not);

pub fn unary_op(op: UnaryOp, value: ValueVec) -> Value {
    ugen(
        UgenSpec {
            name: "UnaryOpUGen",
            special_index: op as _,
            // compile: Compile,
            ..Default::default()
        },
        |mut ugen| {
            ugen.input(value);
            ugen.finish()
        },
    )
}

pub fn binary_op(op: BinaryOp, lhs: ValueVec, rhs: ValueVec, compile: Compile) -> Value {
    ugen(
        UgenSpec {
            name: "BinaryOpUGen",
            special_index: op as _,
            compile,
            ..Default::default()
        },
        |mut ugen| {
            ugen.input(lhs);
            ugen.input(rhs);
            ugen.finish()
        },
    )
}

impl<T> core::iter::Sum<T> for Value
where
    T: Into<ValueVec>,
{
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        let mut acc = Value::from(0);

        for value in iter {
            acc += value.into();
        }

        acc
    }
}

fn float_cmp(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}
