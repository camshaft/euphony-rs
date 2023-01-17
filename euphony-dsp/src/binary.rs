use crate::prelude::*;

macro_rules! binary {
    ($(#[doc = $doc:literal])* $id:literal, $name:ident, | $a:ident, $b:ident | $value:expr) => {
        #[derive(Debug, Clone, Copy, Default, Node)]
        #[node(id = $id, module = "binary")]
        #[input(rhs)]
        #[input(lhs)]
        $(#[doc = $doc])*
        pub struct $name;

        impl $name {
            fn render(&mut self, a: Input, b: Input, output: &mut [Sample]) {
                match (a, b) {
                    (Input::Constant($a), Input::Constant($b)) => {
                        let v = $value;
                        for sample in output.iter_mut() {
                            *sample = v;
                        }
                    }
                    (Input::Constant($a), Input::Buffer(b)) => {
                        for (sample, $b) in output.iter_mut().zip(b.iter().copied()) {
                            *sample = $value;
                        }
                    }
                    (Input::Buffer(a), Input::Constant($b)) => {
                        for (sample, $a) in output.iter_mut().zip(a.iter().copied()) {
                            *sample = $value;
                        }
                    }
                    (Input::Buffer(a), Input::Buffer(b)) => {
                        unsafe {
                            unsafe_assert!(a.len() == b.len());
                        }

                        for (sample, ($a, $b)) in output.iter_mut().zip(a.iter().copied().zip(b.iter().copied())) {
                            *sample = $value;
                        }
                    }
                }
            }
        }
    };
}

binary!(
    /// Adds two signals together
    50,
    Add,
    |a, b| a + b
);
binary!(
    /// Computes the four quadrant arctangent of `lhs` (`y`) and `rhs` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    51,
    Atan2,
    |a, b| a.atan2(b)
);
binary!(
    /// Returns a number composed of the magnitude of `lhs` and the sign of `rhs`.
    ///
    /// Equal to `lhs` if the sign of `lhs` and `rhs` are the same, otherwise equal
    /// to `-lhs`. If `lhs` is a `NAN`, then a `NAN` with the sign of `rhs` is returned.
    52,
    Copysign,
    |a, b| a.copysign(b)
);
binary!(
    /// Divides the left hand signal by the right
    53,
    Div,
    |a, b| a / b
);
binary!(
    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// This computes the integer `n` such that `lhs = n * rhs + self.rem_euclid(rhs)`.
    /// In other words, the result is `lhs / rhs` rounded to the integer n such that `lhs >= n * rhs`.
    54,
    DivEuclid,
    |a, b| a.div_euclid(b)
);
binary!(
    /// Calculates the length of the hypotenuse of a right-angle triangle given legs of length `x` and `y`.
    55,
    Hypot,
    |a, b| a.hypot(b)
);
binary!(
    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation details;
    /// `self.log2()` can produce more accurate results for base 2, and `self.log10()` can produce
    /// more accurate results for base 10.
    56,
    Log,
    |a, b| a.log(b)
);
binary!(
    /// Returns the maximum of the two numbers.
    ///
    /// Follows the IEEE-754 2008 semantics for maxNum, except for handling of signaling `NAN`s. This
    /// matches the behavior of libm’s fmax.
    57,
    Max,
    |a, b| a.max(b)
);
binary!(
    /// Returns the minimum of the two numbers.
    ///
    /// Follows the IEEE-754 2008 semantics for minNum, except for handling of signaling `NAN`s. This
    /// matches the behavior of libm’s fmax.
    58,
    Min,
    |a, b| a.min(b)
);
binary!(
    /// Multiplies two signals together
    59,
    Mul,
    |a, b| a * b
);
binary!(
    /// Raises a number to a floating point power.
    60,
    Powf,
    |a, b| a.powf(b)
);
binary!(
    /// Raises a number to an integer power.
    ///
    /// Using this function is generally faster than using `powf`
    61,
    Powi,
    |a, b| a.powi(b as _)
);
binary!(
    /// Returns the remainder of the left hand signal by the right
    62,
    Rem,
    |a, b| a % b
);
binary!(
    /// Calculates the least nonnegative remainder of `lhs (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == rhs.abs()`, violating the mathematical definition, if
    /// `lhs` is much smaller than `rhs.abs()` in magnitude and `lhs < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `lhs == self.div_euclid(rhs) * rhs + lhs.rem_euclid(rhs)`
    /// approximatively.
    63,
    RemEuclid,
    |a, b| a.rem_euclid(b)
);
binary!(
    /// Subtracts `rhs` from `lhs`
    64,
    Sub,
    |a, b| a - b
);
binary!(
    /// Compares `rhs` to `lhs`. If `rhs > lhs`, the output is `1.0`. Otherwise the
    /// output is `0.0`.
    65,
    Gt,
    |a, b| if a > b { 1.0 } else { 0.0 }
);
binary!(
    /// Compares `rhs` to `lhs`. If `rhs >= lhs`, the output is `1.0`. Otherwise the
    /// output is `0.0`.
    66,
    Gte,
    |a, b| if a >= b { 1.0 } else { 0.0 }
);
binary!(
    /// Compares `rhs` to `lhs`. If `rhs < lhs`, the output is `1.0`. Otherwise the
    /// output is `0.0`.
    67,
    Lt,
    |a, b| if a < b { 1.0 } else { 0.0 }
);
binary!(
    /// Compares `rhs` to `lhs`. If `rhs <= lhs`, the output is `1.0`. Otherwise the
    /// output is `0.0`.
    68,
    Lte,
    |a, b| if a <= b { 1.0 } else { 0.0 }
);
binary!(
    /// Compares `rhs` to `lhs`. If `rhs == lhs`, the output is `1.0`. Otherwise the
    /// output is `0.0`.
    69,
    Eq,
    |a, b| if a == b { 1.0 } else { 0.0 }
);
binary!(
    /// Compares `rhs` to `lhs`. If `rhs != lhs`, the output is `1.0`. Otherwise the
    /// output is `0.0`.
    70,
    Ne,
    |a, b| if a != b { 1.0 } else { 0.0 }
);
