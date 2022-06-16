use crate::prelude::*;

macro_rules! unary {
    ($(#[doc = $doc:literal])* $id:literal, $name:ident, | $input:ident | $value:expr) => {
        #[derive(Debug, Clone, Copy, Default, Node)]
        #[node(id = $id, module = "unary")]
        #[input($input)]
        $(#[doc = $doc])*
        pub struct $name;

        impl $name {
            fn render(&mut self, input: Input, output: &mut [Sample]) {
                match input {
                    Input::Constant($input) => {
                        let v = $value;
                        for sample in output.iter_mut() {
                            *sample = v;
                        }
                    }
                    Input::Buffer(a) => {
                        for (sample, input) in (output.iter_mut(), a).zip() {
                            let $input = *input;
                            *sample = $value;
                        }
                    }
                }
            }
        }
    };
}

unary!(
    /// Computes the absolute value of `input`. Returns `NAN` if the number is `NAN`.
    1,
    Abs,
    |input| input.abs()
);
unary!(
    /// Computes the arccosine of a number. Return value is in radians in the
    /// range [0, pi] or NaN if the number is outside the range [-1, 1].
    2,
    Acos,
    |input| input.acos()
);
unary!(
    /// Inverse hyperbolic cosine function.
    3,
    Acosh,
    |input| input.acosh()
);
unary!(
    /// Computes the arcsine of a number. Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    4,
    Asin,
    |input| input.asin()
);
unary!(
    /// Inverse hyperbolic sine function.
    5,
    Asinh,
    |input| input.asinh()
);
unary!(
    /// Computes the arctangent of a number. Return value is in radians in the range [-pi/2, pi/2];
    6,
    Atan,
    |input| input.atan()
);
unary!(
    /// Inverse hyperbolic tangent function.
    7,
    Atanh,
    |input| input.atanh()
);
unary!(
    /// Returns the cube root of a number.
    8,
    Cbrt,
    |input| input.cbrt()
);
unary!(
    /// Returns the smallest integer greater than or equal to a number.
    9,
    Ceil,
    |input| input.ceil()
);
unary!(
    /// Computes the cosine of a number (in radians).
    10,
    Cos,
    |input| input.cos()
);
unary!(
    /// Hyperbolic cosine function.
    11,
    Cosh,
    |input| input.cosh()
);
unary!(
    /// Returns `e^(self)`, (the exponential function).
    12,
    Exp,
    |input| input.exp()
);
unary!(
    /// Returns `2^(self)`
    13,
    Exp2,
    |input| input.exp2()
);
unary!(
    /// Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.
    14,
    ExpM1,
    |input| input.exp_m1()
);
unary!(
    /// Returns the largest integer less than or equal to a number.
    15,
    Floor,
    |input| input.floor()
);
unary!(
    /// Returns the fractional part of a number.
    16,
    Fract,
    |input| input.fract()
);
unary!(
    /// Returns the natural logarithm of the number.
    17,
    Ln,
    |input| input.ln()
);
unary!(
    /// Returns ln(1+n) (natural logarithm) more accurately than if the operations
    /// were performed separately.
    18,
    Ln1p,
    |input| input.ln_1p()
);
unary!(
    /// Returns the base 10 logarithm of the number.
    19,
    Log10,
    |input| input.log10()
);
unary!(
    /// Returns the base 2 logarithm of the number.
    20,
    Log2,
    |input| input.log2()
);
unary!(
    /// Normalizes a number.
    ///
    /// * `-0.0` will be converted into `0.0`
    /// * `NAN` will be converted into `0.0`
    /// * `INFINITY` will be converted into `MAX`
    /// * `NEG_INFINITY` will be converted into `MIN`
    21,
    Norm,
    |input| {
        use core::num::FpCategory::*;
        match input.classify() {
            Nan => 0.0,
            Infinite if input.is_sign_positive() => f64::MAX,
            Infinite => f64::MIN,
            Zero => 0.0,
            _ => input,
        }
    }
);
unary!(
    /// Takes the reciprocal (inverse) of a number, 1/x.
    22,
    Recip,
    |input| input.recip()
);
unary!(
    /// Returns the nearest integer to a number. Round half-way cases away from `0.0`.
    23,
    Round,
    |input| input.round()
);
unary!(
    /// Returns a number that represents the sign of `self`.
    ///
    /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// * `NAN` if the number is `NAN`
    24,
    Signum,
    |input| input.signum()
);
unary!(
    /// Computes the sine of a number (in radians).
    25,
    Sin,
    |input| input.sin()
);
unary!(
    /// Hyperbolic sine function.
    26,
    Sinh,
    |input| input.sinh()
);
unary!(
    /// Returns the square root of a number.
    ///
    /// Returns `NaN` if `self` is a negative number other than `-0.0`.
    27,
    Sqrt,
    |input| input.sqrt()
);
unary!(
    /// Computes the tangent of a number (in radians).
    28,
    Tan,
    |input| input.tan()
);
unary!(
    /// Hyperbolic tangent function.
    29,
    Tanh,
    |input| input.tanh()
);
unary!(
    /// Converts radians to degrees.
    30,
    ToDegrees,
    |input| input.to_degrees()
);
unary!(
    /// Converts degrees to radians.
    31,
    ToRadians,
    |input| input.to_radians()
);
unary!(
    /// Returns the integer part of a number.
    32,
    Trunc,
    |input| input.trunc()
);
unary!(
    /// The unary negation operator `-`.
    33,
    Neg,
    |input| -input
);
unary!(
    /// Passes the input signal to the output signal
    34,
    Pass,
    |input| input
);
