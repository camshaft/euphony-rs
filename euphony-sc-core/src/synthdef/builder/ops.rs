use super::{binary_op, unary_op, Value, ValueVec};
use crate::synthdef::{BinaryOp, UnaryOp};
use core::ops;

macro_rules! binary {
    ($op:ident, $name:ident, $call:ident, $assign:ident, $assign_call:ident) => {
        impl ops::$name<Value> for i32 {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl ops::$name<ValueVec> for i32 {
            type Output = Value;

            fn $call(self, rhs: ValueVec) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for f32 {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl ops::$name<ValueVec> for f32 {
            type Output = Value;

            fn $call(self, rhs: ValueVec) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for Vec<Value> {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for Vec<i32> {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl ops::$name<Value> for Vec<f32> {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl<const N: usize> ops::$name<Value> for [Value; N] {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl<const N: usize> ops::$name<Value> for [i32; N] {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl<const N: usize> ops::$name<Value> for [f32; N] {
            type Output = Value;

            fn $call(self, rhs: Value) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl<T> ops::$name<T> for Value
        where
            T: Into<ValueVec>,
        {
            type Output = Value;

            fn $call(self, rhs: T) -> Self::Output {
                binary_op(BinaryOp::$op, self.into(), rhs.into())
            }
        }

        impl<T> ops::$name<T> for ValueVec
        where
            T: Into<ValueVec>,
        {
            type Output = Value;

            fn $call(self, rhs: T) -> Self::Output {
                binary_op(BinaryOp::$op, self, rhs.into())
            }
        }

        impl<T> ops::$assign<T> for Value
        where
            T: Into<ValueVec>,
        {
            fn $assign_call(&mut self, rhs: T) {
                *self = binary_op(BinaryOp::$op, (*self).into(), rhs.into());
            }
        }
    };
}

binary!(Add, Add, add, AddAssign, add_assign);
binary!(Subtract, Sub, sub, SubAssign, sub_assign);
binary!(Multiply, Mul, mul, MulAssign, mul_assign);
binary!(Divide, Div, div, DivAssign, div_assign);
binary!(Modulus, Rem, rem, RemAssign, rem_assign);
binary!(And, BitAnd, bitand, BitAndAssign, bitand_assign);
binary!(Or, BitOr, bitor, BitOrAssign, bitor_assign);
binary!(Xor, BitXor, bitxor, BitXorAssign, bitxor_assign);

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
