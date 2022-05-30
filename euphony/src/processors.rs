#[rustfmt::skip]
pub mod ext {
    use crate::parameter::Parameter;
    use super::input::*;
    pub trait ProcessorExt: crate::processor::Processor
    where
        for<'a> &'a Self: Into<Parameter>,
    {
        #[inline]
        #[doc = " Computes the absolute value of `input`. Returns `NAN` if the number is `NAN`.\n"]
        fn abs(&self) -> crate::processors::unary::Abs {
            crate::processors::unary::abs().with_input(self)
        }
        #[inline]
        #[doc = " Computes the arccosine of a number. Return value is in radians in the\n range [0, pi] or NaN if the number is outside the range [-1, 1].\n"]
        fn acos(&self) -> crate::processors::unary::Acos {
            crate::processors::unary::acos().with_input(self)
        }
        #[inline]
        #[doc = " Inverse hyperbolic cosine function.\n"]
        fn acosh(&self) -> crate::processors::unary::Acosh {
            crate::processors::unary::acosh().with_input(self)
        }
        #[inline]
        #[doc = " Adds two signals together\n"]
        fn add<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Add
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::add().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Fused multiply-add. Computes `(input + add) * mul` with only one rounding\n error, yielding a more accurate result than an unfused add-multiply.\n"]
        fn add_mul<Add, Mul>(&self, add: Add, mul: Mul) -> crate::processors::tertiary::AddMul
        where
            Add: Into<Parameter>,
            Mul: Into<Parameter>,
        {
            crate::processors::tertiary::add_mul().with_input(self).with_add(add).with_mul(mul)
        }
        #[inline]
        #[doc = " Computes the arcsine of a number. Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].\n"]
        fn asin(&self) -> crate::processors::unary::Asin {
            crate::processors::unary::asin().with_input(self)
        }
        #[inline]
        #[doc = " Inverse hyperbolic sine function.\n"]
        fn asinh(&self) -> crate::processors::unary::Asinh {
            crate::processors::unary::asinh().with_input(self)
        }
        #[inline]
        #[doc = " Computes the arctangent of a number. Return value is in radians in the range [-pi/2, pi/2];\n"]
        fn atan(&self) -> crate::processors::unary::Atan {
            crate::processors::unary::atan().with_input(self)
        }
        #[inline]
        #[doc = " Computes the four quadrant arctangent of `lhs` (`y`) and `rhs` (`x`) in radians.\n\n * `x = 0`, `y = 0`: `0`\n * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`\n * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`\n * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`\n"]
        fn atan2<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Atan2
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::atan2().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Inverse hyperbolic tangent function.\n"]
        fn atanh(&self) -> crate::processors::unary::Atanh {
            crate::processors::unary::atanh().with_input(self)
        }
        #[inline]
        #[doc = " Returns the cube root of a number.\n"]
        fn cbrt(&self) -> crate::processors::unary::Cbrt {
            crate::processors::unary::cbrt().with_input(self)
        }
        #[inline]
        #[doc = " Returns the smallest integer greater than or equal to a number.\n"]
        fn ceil(&self) -> crate::processors::unary::Ceil {
            crate::processors::unary::ceil().with_input(self)
        }
        #[inline]
        #[doc = " Restrict a value to a certain interval unless it is NaN.\n\n Returns `max` if `input` is greater than `max`, and `min` if `input` is\n less than `min`. Otherwise this returns `input`.\n\n Note that this function returns NaN if the initial value was NaN as\n well or `min > max`\n"]
        fn clamp<Min, Max>(&self, min: Min, max: Max) -> crate::processors::tertiary::Clamp
        where
            Min: Into<Parameter>,
            Max: Into<Parameter>,
        {
            crate::processors::tertiary::clamp().with_input(self).with_min(min).with_max(max)
        }
        #[inline]
        #[doc = " Returns a number composed of the magnitude of `lhs` and the sign of `rhs`.\n\n Equal to `lhs` if the sign of `lhs` and `rhs` are the same, otherwise equal\n to `-lhs`. If `lhs` is a `NAN`, then a `NAN` with the sign of `rhs` is returned.\n"]
        fn copysign<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Copysign
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::copysign().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Computes the cosine of a number (in radians).\n"]
        fn cos(&self) -> crate::processors::unary::Cos {
            crate::processors::unary::cos().with_input(self)
        }
        #[inline]
        #[doc = " Hyperbolic cosine function.\n"]
        fn cosh(&self) -> crate::processors::unary::Cosh {
            crate::processors::unary::cosh().with_input(self)
        }
        #[inline]
        #[doc = " Divides the left hand signal by the right\n"]
        fn div<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Div
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::div().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Calculates Euclidean division, the matching method for `rem_euclid`.\n\n This computes the integer `n` such that `lhs = n * rhs + self.rem_euclid(rhs)`.\n In other words, the result is `lhs / rhs` rounded to the integer n such that `lhs >= n * rhs`.\n"]
        fn div_euclid<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::DivEuclid
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::div_euclid().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Returns `e^(self)`, (the exponential function).\n"]
        fn exp(&self) -> crate::processors::unary::Exp {
            crate::processors::unary::exp().with_input(self)
        }
        #[inline]
        #[doc = " Returns `2^(self)`\n"]
        fn exp2(&self) -> crate::processors::unary::Exp2 {
            crate::processors::unary::exp2().with_input(self)
        }
        #[inline]
        #[doc = " Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.\n"]
        fn exp_m1(&self) -> crate::processors::unary::ExpM1 {
            crate::processors::unary::exp_m1().with_input(self)
        }
        #[inline]
        #[doc = " Returns the largest integer less than or equal to a number.\n"]
        fn floor(&self) -> crate::processors::unary::Floor {
            crate::processors::unary::floor().with_input(self)
        }
        #[inline]
        #[doc = " Returns the fractional part of a number.\n"]
        fn fract(&self) -> crate::processors::unary::Fract {
            crate::processors::unary::fract().with_input(self)
        }
        #[inline]
        #[doc = " Calculates the length of the hypotenuse of a right-angle triangle given legs of length `x` and `y`.\n"]
        fn hypot<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Hypot
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::hypot().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Returns the natural logarithm of the number.\n"]
        fn ln(&self) -> crate::processors::unary::Ln {
            crate::processors::unary::ln().with_input(self)
        }
        #[inline]
        #[doc = " Returns ln(1+n) (natural logarithm) more accurately than if the operations\n were performed separately.\n"]
        fn ln1p(&self) -> crate::processors::unary::Ln1p {
            crate::processors::unary::ln1p().with_input(self)
        }
        #[inline]
        #[doc = " Returns the logarithm of the number with respect to an arbitrary base.\n\n The result might not be correctly rounded owing to implementation details;\n `self.log2()` can produce more accurate results for base 2, and `self.log10()` can produce\n more accurate results for base 10.\n"]
        fn log<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Log
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::log().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Returns the base 10 logarithm of the number.\n"]
        fn log10(&self) -> crate::processors::unary::Log10 {
            crate::processors::unary::log10().with_input(self)
        }
        #[inline]
        #[doc = " Returns the base 2 logarithm of the number.\n"]
        fn log2(&self) -> crate::processors::unary::Log2 {
            crate::processors::unary::log2().with_input(self)
        }
        #[inline]
        #[doc = " Returns the maximum of the two numbers.\n\n Follows the IEEE-754 2008 semantics for maxNum, except for handling of signaling `NAN`s. This\n matches the behavior of libm’s fmax.\n"]
        fn max<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Max
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::max().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Returns the minimum of the two numbers.\n\n Follows the IEEE-754 2008 semantics for minNum, except for handling of signaling `NAN`s. This\n matches the behavior of libm’s fmax.\n"]
        fn min<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Min
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::min().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Multiplies two signals together\n"]
        fn mul<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Mul
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::mul().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Fused multiply-add. Computes `(input * mul) + add` with only one rounding\n error, yielding a more accurate result than an unfused multiply-add.\n"]
        fn mul_add<Mul, Add>(&self, mul: Mul, add: Add) -> crate::processors::tertiary::MulAdd
        where
            Mul: Into<Parameter>,
            Add: Into<Parameter>,
        {
            crate::processors::tertiary::mul_add().with_input(self).with_mul(mul).with_add(add)
        }
        #[inline]
        #[doc = " The unary negation operator `-`.\n"]
        fn neg(&self) -> crate::processors::unary::Neg {
            crate::processors::unary::neg().with_input(self)
        }
        #[inline]
        #[doc = " Normalizes a number.\n\n * `-0.0` will be converted into `0.0`\n * `NAN` will be converted into `0.0`\n * `INFINITY` will be converted into `MAX`\n * `NEG_INFINITY` will be converted into `MIN`\n"]
        fn norm(&self) -> crate::processors::unary::Norm {
            crate::processors::unary::norm().with_input(self)
        }
        #[inline]
        #[doc = " Raises a number to a floating point power.\n"]
        fn powf<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Powf
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::powf().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Raises a number to an integer power.\n\n Using this function is generally faster than using `powf`\n"]
        fn powi<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Powi
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::powi().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Takes the reciprocal (inverse) of a number, 1/x.\n"]
        fn recip(&self) -> crate::processors::unary::Recip {
            crate::processors::unary::recip().with_input(self)
        }
        #[inline]
        #[doc = " Returns the remainder of the left hand signal by the right\n"]
        fn rem<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Rem
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::rem().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Calculates the least nonnegative remainder of `lhs (mod rhs)`.\n\n In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in\n most cases. However, due to a floating point round-off error it can\n result in `r == rhs.abs()`, violating the mathematical definition, if\n `lhs` is much smaller than `rhs.abs()` in magnitude and `lhs < 0.0`.\n This result is not an element of the function's codomain, but it is the\n closest floating point number in the real numbers and thus fulfills the\n property `lhs == self.div_euclid(rhs) * rhs + lhs.rem_euclid(rhs)`\n approximatively.\n"]
        fn rem_euclid<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::RemEuclid
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::rem_euclid().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Returns the nearest integer to a number. Round half-way cases away from `0.0`.\n"]
        fn round(&self) -> crate::processors::unary::Round {
            crate::processors::unary::round().with_input(self)
        }
        #[inline]
        #[doc = " If `cond` is positive, then `positive` is returned. Otherwise `negative`\n is returned.\n"]
        fn select<Positive, Negative>(&self, positive: Positive, negative: Negative) -> crate::processors::tertiary::Select
        where
            Positive: Into<Parameter>,
            Negative: Into<Parameter>,
        {
            crate::processors::tertiary::select().with_cond(self).with_positive(positive).with_negative(negative)
        }
        #[inline]
        #[doc = " Returns a number that represents the sign of `self`.\n\n * `1.0` if the number is positive, `+0.0` or `INFINITY`\n * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`\n * `NAN` if the number is `NAN`\n"]
        fn signum(&self) -> crate::processors::unary::Signum {
            crate::processors::unary::signum().with_input(self)
        }
        #[inline]
        #[doc = " Computes the sine of a number (in radians).\n"]
        fn sin(&self) -> crate::processors::unary::Sin {
            crate::processors::unary::sin().with_input(self)
        }
        #[inline]
        #[doc = " Hyperbolic sine function.\n"]
        fn sinh(&self) -> crate::processors::unary::Sinh {
            crate::processors::unary::sinh().with_input(self)
        }
        #[inline]
        #[doc = " Returns the square root of a number.\n\n Returns `NaN` if `self` is a negative number other than `-0.0`.\n"]
        fn sqrt(&self) -> crate::processors::unary::Sqrt {
            crate::processors::unary::sqrt().with_input(self)
        }
        #[inline]
        #[doc = " Subtracts `rhs` from `lhs`\n"]
        fn sub<Lhs>(&self, lhs: Lhs) -> crate::processors::binary::Sub
        where
            Lhs: Into<Parameter>,
        {
            crate::processors::binary::sub().with_rhs(self).with_lhs(lhs)
        }
        #[inline]
        #[doc = " Computes the tangent of a number (in radians).\n"]
        fn tan(&self) -> crate::processors::unary::Tan {
            crate::processors::unary::tan().with_input(self)
        }
        #[inline]
        #[doc = " Hyperbolic tangent function.\n"]
        fn tanh(&self) -> crate::processors::unary::Tanh {
            crate::processors::unary::tanh().with_input(self)
        }
        #[inline]
        #[doc = " Converts radians to degrees.\n"]
        fn to_degrees(&self) -> crate::processors::unary::ToDegrees {
            crate::processors::unary::to_degrees().with_input(self)
        }
        #[inline]
        #[doc = " Converts degrees to radians.\n"]
        fn to_radians(&self) -> crate::processors::unary::ToRadians {
            crate::processors::unary::to_radians().with_input(self)
        }
        #[inline]
        #[doc = " Returns the integer part of a number.\n"]
        fn trunc(&self) -> crate::processors::unary::Trunc {
            crate::processors::unary::trunc().with_input(self)
        }
        #[inline]
        #[doc = " Allpass filter.\n"]
        fn allpass(&self) -> crate::processors::filter::Allpass {
            crate::processors::filter::allpass().with_signal(self)
        }
        #[inline]
        #[doc = " Allpass filter with adjustable delay (delay > 0) in samples at DC.\n"]
        fn allpole(&self) -> crate::processors::filter::Allpole {
            crate::processors::filter::allpole().with_signal(self)
        }
        #[inline]
        #[doc = " Bandpass filter.\n"]
        fn bandpass(&self) -> crate::processors::filter::Bandpass {
            crate::processors::filter::bandpass().with_signal(self)
        }
        #[inline]
        #[doc = " Bell filter.\n"]
        fn bell(&self) -> crate::processors::filter::Bell {
            crate::processors::filter::bell().with_signal(self)
        }
        #[inline]
        #[doc = " Butterworth lowpass filter (2nd order).\n"]
        fn butterpass(&self) -> crate::processors::filter::Butterpass {
            crate::processors::filter::butterpass().with_signal(self)
        }
        #[inline]
        #[doc = " Highpass filter.\n"]
        fn highpass(&self) -> crate::processors::filter::Highpass {
            crate::processors::filter::highpass().with_signal(self)
        }
        #[inline]
        #[doc = " One-pole, one-zero highpass filter (1st order).\n"]
        fn highpole(&self) -> crate::processors::filter::Highpole {
            crate::processors::filter::highpole().with_signal(self)
        }
        #[inline]
        #[doc = " Highshelf filter.\n"]
        fn highshelf(&self) -> crate::processors::filter::Highshelf {
            crate::processors::filter::highshelf().with_signal(self)
        }
        #[inline]
        #[doc = " Lowpass filter.\n"]
        fn lowpass(&self) -> crate::processors::filter::Lowpass {
            crate::processors::filter::lowpass().with_signal(self)
        }
        #[inline]
        #[doc = " One-pole lowpass filter (1st order).\n"]
        fn lowpole(&self) -> crate::processors::filter::Lowpole {
            crate::processors::filter::lowpole().with_signal(self)
        }
        #[inline]
        #[doc = " Lowshelf filter.\n"]
        fn lowshelf(&self) -> crate::processors::filter::Lowshelf {
            crate::processors::filter::lowshelf().with_signal(self)
        }
        #[inline]
        #[doc = " Moog resonant lowpass filter.\n"]
        fn moog(&self) -> crate::processors::filter::Moog {
            crate::processors::filter::moog().with_signal(self)
        }
        #[inline]
        #[doc = " Morphing filter that morphs between lowpass, peak and highpass modes.\n"]
        fn morph(&self) -> crate::processors::filter::Morph {
            crate::processors::filter::morph().with_signal(self)
        }
        #[inline]
        #[doc = " Notch filter.\n"]
        fn notch(&self) -> crate::processors::filter::Notch {
            crate::processors::filter::notch().with_signal(self)
        }
        #[inline]
        #[doc = " Peak filter.\n"]
        fn peak(&self) -> crate::processors::filter::Peak {
            crate::processors::filter::peak().with_signal(self)
        }
        #[inline]
        #[doc = " Pinking filter.\n"]
        fn pinkpass(&self) -> crate::processors::filter::Pinkpass {
            crate::processors::filter::pinkpass().with_signal(self)
        }
        #[inline]
        #[doc = " Constant-gain bandpass resonator.\n"]
        fn resonator(&self) -> crate::processors::filter::Resonator {
            crate::processors::filter::resonator().with_signal(self)
        }
        #[inline]
        #[doc = ""]
        fn shaper(&self) -> crate::processors::filter::Shaper {
            crate::processors::filter::shaper().with_signal(self)
        }
    }
    impl<T> ProcessorExt for T
    where
        Self: crate::processor::Processor,
        for<'a> &'a Self: Into<Parameter>,
    {}
}
pub mod input {
    pub trait AddInput<Value> {
        fn with_add(self, value: Value) -> Self;
        fn set_add(&self, value: Value) -> &Self;
    }
    pub trait AzimuthInput<Value> {
        fn with_azimuth(self, value: Value) -> Self;
        fn set_azimuth(&self, value: Value) -> &Self;
    }
    pub trait BandwidthInput<Value> {
        fn with_bandwidth(self, value: Value) -> Self;
        fn set_bandwidth(&self, value: Value) -> &Self;
    }
    pub trait BufferInput<Value> {
        fn with_buffer(self, value: Value) -> Self;
        fn set_buffer(&self, value: Value) -> &Self;
    }
    pub trait CenterInput<Value> {
        fn with_center(self, value: Value) -> Self;
        fn set_center(&self, value: Value) -> &Self;
    }
    pub trait CondInput<Value> {
        fn with_cond(self, value: Value) -> Self;
        fn set_cond(&self, value: Value) -> &Self;
    }
    pub trait CutoffInput<Value> {
        fn with_cutoff(self, value: Value) -> Self;
        fn set_cutoff(&self, value: Value) -> &Self;
    }
    pub trait DecayInput<Value> {
        fn with_decay(self, value: Value) -> Self;
        fn set_decay(&self, value: Value) -> &Self;
    }
    pub trait DelayInput<Value> {
        fn with_delay(self, value: Value) -> Self;
        fn set_delay(&self, value: Value) -> &Self;
    }
    pub trait DurationInput<Value> {
        fn with_duration(self, value: Value) -> Self;
        fn set_duration(&self, value: Value) -> &Self;
    }
    pub trait DutyCycleInput<Value> {
        fn with_duty_cycle(self, value: Value) -> Self;
        fn set_duty_cycle(&self, value: Value) -> &Self;
    }
    pub trait FrequencyInput<Value> {
        fn with_frequency(self, value: Value) -> Self;
        fn set_frequency(&self, value: Value) -> &Self;
    }
    pub trait GainInput<Value> {
        fn with_gain(self, value: Value) -> Self;
        fn set_gain(&self, value: Value) -> &Self;
    }
    pub trait InclineInput<Value> {
        fn with_incline(self, value: Value) -> Self;
        fn set_incline(&self, value: Value) -> &Self;
    }
    pub trait InputInput<Value> {
        fn with_input(self, value: Value) -> Self;
        fn set_input(&self, value: Value) -> &Self;
    }
    pub trait LengthInput<Value> {
        fn with_length(self, value: Value) -> Self;
        fn set_length(&self, value: Value) -> &Self;
    }
    pub trait LhsInput<Value> {
        fn with_lhs(self, value: Value) -> Self;
        fn set_lhs(&self, value: Value) -> &Self;
    }
    pub trait MaxInput<Value> {
        fn with_max(self, value: Value) -> Self;
        fn set_max(&self, value: Value) -> &Self;
    }
    pub trait MinInput<Value> {
        fn with_min(self, value: Value) -> Self;
        fn set_min(&self, value: Value) -> &Self;
    }
    pub trait MorphInput<Value> {
        fn with_morph(self, value: Value) -> Self;
        fn set_morph(&self, value: Value) -> &Self;
    }
    pub trait MulInput<Value> {
        fn with_mul(self, value: Value) -> Self;
        fn set_mul(&self, value: Value) -> &Self;
    }
    pub trait NegativeInput<Value> {
        fn with_negative(self, value: Value) -> Self;
        fn set_negative(&self, value: Value) -> &Self;
    }
    pub trait PhaseInput<Value> {
        fn with_phase(self, value: Value) -> Self;
        fn set_phase(&self, value: Value) -> &Self;
    }
    pub trait PositiveInput<Value> {
        fn with_positive(self, value: Value) -> Self;
        fn set_positive(&self, value: Value) -> &Self;
    }
    pub trait QInput<Value> {
        fn with_q(self, value: Value) -> Self;
        fn set_q(&self, value: Value) -> &Self;
    }
    pub trait RadiusInput<Value> {
        fn with_radius(self, value: Value) -> Self;
        fn set_radius(&self, value: Value) -> &Self;
    }
    pub trait RepeatInput<Value> {
        fn with_repeat(self, value: Value) -> Self;
        fn set_repeat(&self, value: Value) -> &Self;
    }
    pub trait ResetInput<Value> {
        fn with_reset(self, value: Value) -> Self;
        fn set_reset(&self, value: Value) -> &Self;
    }
    pub trait RhsInput<Value> {
        fn with_rhs(self, value: Value) -> Self;
        fn set_rhs(&self, value: Value) -> &Self;
    }
    pub trait SeedInput<Value> {
        fn with_seed(self, value: Value) -> Self;
        fn set_seed(&self, value: Value) -> &Self;
    }
    pub trait SignalInput<Value> {
        fn with_signal(self, value: Value) -> Self;
        fn set_signal(&self, value: Value) -> &Self;
    }
    pub trait TargetInput<Value> {
        fn with_target(self, value: Value) -> Self;
        fn set_target(&self, value: Value) -> &Self;
    }
    pub trait ValueInput<Value> {
        fn with_value(self, value: Value) -> Self;
        fn set_value(&self, value: Value) -> &Self;
    }
}

#[rustfmt::skip]
mod api {

    pub mod binary {
        define_processor!(
            #[doc = " Adds two signals together\n"]
            #[id = 50]
            #[lower = add]
            struct Add {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Computes the four quadrant arctangent of `lhs` (`y`) and `rhs` (`x`) in radians.\n\n * `x = 0`, `y = 0`: `0`\n * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`\n * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`\n * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`\n"]
            #[id = 51]
            #[lower = atan2]
            struct Atan2 {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Returns a number composed of the magnitude of `lhs` and the sign of `rhs`.\n\n Equal to `lhs` if the sign of `lhs` and `rhs` are the same, otherwise equal\n to `-lhs`. If `lhs` is a `NAN`, then a `NAN` with the sign of `rhs` is returned.\n"]
            #[id = 52]
            #[lower = copysign]
            struct Copysign {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Divides the left hand signal by the right\n"]
            #[id = 53]
            #[lower = div]
            struct Div {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Calculates Euclidean division, the matching method for `rem_euclid`.\n\n This computes the integer `n` such that `lhs = n * rhs + self.rem_euclid(rhs)`.\n In other words, the result is `lhs / rhs` rounded to the integer n such that `lhs >= n * rhs`.\n"]
            #[id = 54]
            #[lower = div_euclid]
            struct DivEuclid {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Calculates the length of the hypotenuse of a right-angle triangle given legs of length `x` and `y`.\n"]
            #[id = 55]
            #[lower = hypot]
            struct Hypot {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Returns the logarithm of the number with respect to an arbitrary base.\n\n The result might not be correctly rounded owing to implementation details;\n `self.log2()` can produce more accurate results for base 2, and `self.log10()` can produce\n more accurate results for base 10.\n"]
            #[id = 56]
            #[lower = log]
            struct Log {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Returns the maximum of the two numbers.\n\n Follows the IEEE-754 2008 semantics for maxNum, except for handling of signaling `NAN`s. This\n matches the behavior of libm’s fmax.\n"]
            #[id = 57]
            #[lower = max]
            struct Max {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Returns the minimum of the two numbers.\n\n Follows the IEEE-754 2008 semantics for minNum, except for handling of signaling `NAN`s. This\n matches the behavior of libm’s fmax.\n"]
            #[id = 58]
            #[lower = min]
            struct Min {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Multiplies two signals together\n"]
            #[id = 59]
            #[lower = mul]
            struct Mul {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Raises a number to a floating point power.\n"]
            #[id = 60]
            #[lower = powf]
            struct Powf {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Raises a number to an integer power.\n\n Using this function is generally faster than using `powf`\n"]
            #[id = 61]
            #[lower = powi]
            struct Powi {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Returns the remainder of the left hand signal by the right\n"]
            #[id = 62]
            #[lower = rem]
            struct Rem {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Calculates the least nonnegative remainder of `lhs (mod rhs)`.\n\n In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in\n most cases. However, due to a floating point round-off error it can\n result in `r == rhs.abs()`, violating the mathematical definition, if\n `lhs` is much smaller than `rhs.abs()` in magnitude and `lhs < 0.0`.\n This result is not an element of the function's codomain, but it is the\n closest floating point number in the real numbers and thus fulfills the\n property `lhs == self.div_euclid(rhs) * rhs + lhs.rem_euclid(rhs)`\n approximatively.\n"]
            #[id = 63]
            #[lower = rem_euclid]
            struct RemEuclid {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Subtracts `rhs` from `lhs`\n"]
            #[id = 64]
            #[lower = sub]
            struct Sub {
                #[trait = RhsInput]
                #[with = with_rhs]
                #[set = set_rhs]
                rhs: Parameter<0>,
                #[trait = LhsInput]
                #[with = with_lhs]
                #[set = set_lhs]
                lhs: Parameter<1>,
            }
        );
    }
    pub mod buffer {
        define_processor!(
            #[doc = " Plays a buffer\n"]
            #[id = 500]
            #[lower = play]
            struct Play {
                #[buffer]
                #[trait = BufferInput]
                #[with = with_buffer]
                #[set = set_buffer]
                buffer: Buffer<0>,
                #[trait = RepeatInput]
                #[with = with_repeat]
                #[set = set_repeat]
                repeat: f64<0>,
                #[trait = ResetInput]
                #[with = with_reset]
                #[set = set_reset]
                reset: f64<1>,
            }
        );
    }
    pub mod env {
        define_processor!(
            #[id = 200]
            #[lower = linear]
            struct Linear {
                #[trait = TargetInput]
                #[with = with_target]
                #[set = set_target]
                target: f64<0>,
                #[trait = DurationInput]
                #[with = with_duration]
                #[set = set_duration]
                duration: f64<1>,
                #[trait = ValueInput]
                #[with = with_value]
                #[set = set_value]
                value: f64<2>,
            }
        );
    }
    pub mod filter {
        define_processor!(
            #[doc = " Butterworth lowpass filter (2nd order).\n"]
            #[id = 300]
            #[lower = butterpass]
            struct Butterpass {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " One-pole lowpass filter (1st order).\n"]
            #[id = 301]
            #[lower = lowpole]
            struct Lowpole {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Allpass filter with adjustable delay (delay > 0) in samples at DC.\n"]
            #[id = 302]
            #[lower = allpole]
            struct Allpole {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = DelayInput]
                #[with = with_delay]
                #[set = set_delay]
                delay: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " One-pole, one-zero highpass filter (1st order).\n"]
            #[id = 303]
            #[lower = highpole]
            struct Highpole {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
            }
        );

        define_processor!(
            #[doc = " Constant-gain bandpass resonator.\n"]
            #[id = 304]
            #[lower = resonator]
            struct Resonator {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
                #[trait = BandwidthInput]
                #[with = with_bandwidth]
                #[set = set_bandwidth]
                bandwidth: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Moog resonant lowpass filter.\n"]
            #[id = 305]
            #[lower = moog]
            struct Moog {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Morphing filter that morphs between lowpass, peak and highpass modes.\n"]
            #[id = 306]
            #[lower = morph]
            struct Morph {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CenterInput]
                #[with = with_center]
                #[set = set_center]
                center: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
                #[trait = MorphInput]
                #[with = with_morph]
                #[set = set_morph]
                morph: Parameter<3>,
            }
        );

        define_processor!(
            #[doc = " Pinking filter.\n"]
            #[id = 307]
            #[lower = pinkpass]
            struct Pinkpass {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Lowpass filter.\n"]
            #[id = 308]
            #[lower = lowpass]
            struct Lowpass {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Highpass filter.\n"]
            #[id = 309]
            #[lower = highpass]
            struct Highpass {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Bandpass filter.\n"]
            #[id = 310]
            #[lower = bandpass]
            struct Bandpass {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CutoffInput]
                #[with = with_cutoff]
                #[set = set_cutoff]
                cutoff: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Notch filter.\n"]
            #[id = 311]
            #[lower = notch]
            struct Notch {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CenterInput]
                #[with = with_center]
                #[set = set_center]
                center: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Peak filter.\n"]
            #[id = 312]
            #[lower = peak]
            struct Peak {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CenterInput]
                #[with = with_center]
                #[set = set_center]
                center: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Allpass filter.\n"]
            #[id = 313]
            #[lower = allpass]
            struct Allpass {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CenterInput]
                #[with = with_center]
                #[set = set_center]
                center: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Bell filter.\n"]
            #[id = 314]
            #[lower = bell]
            struct Bell {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CenterInput]
                #[with = with_center]
                #[set = set_center]
                center: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
                #[trait = GainInput]
                #[with = with_gain]
                #[set = set_gain]
                gain: Parameter<3>,
            }
        );

        define_processor!(
            #[doc = " Lowshelf filter.\n"]
            #[id = 315]
            #[lower = lowshelf]
            struct Lowshelf {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CenterInput]
                #[with = with_center]
                #[set = set_center]
                center: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
                #[trait = GainInput]
                #[with = with_gain]
                #[set = set_gain]
                gain: Parameter<3>,
            }
        );

        define_processor!(
            #[doc = " Highshelf filter.\n"]
            #[id = 316]
            #[lower = highshelf]
            struct Highshelf {
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
                #[trait = CenterInput]
                #[with = with_center]
                #[set = set_center]
                center: Parameter<1>,
                #[trait = QInput]
                #[with = with_q]
                #[set = set_q]
                q: Parameter<2>,
                #[trait = GainInput]
                #[with = with_gain]
                #[set = set_gain]
                gain: Parameter<3>,
            }
        );

        define_processor!(
            #[id = 317]
            #[lower = shaper]
            struct Shaper {
                #[buffer]
                #[trait = BufferInput]
                #[with = with_buffer]
                #[set = set_buffer]
                buffer: Buffer<0>,
                #[trait = SignalInput]
                #[with = with_signal]
                #[set = set_signal]
                signal: Parameter<0>,
            }
        );
    }
    pub mod osc {
        define_processor!(
            #[doc = " Accurate (slow) sine oscillator\n\n # frequency\n\n # phase (trigger)\n"]
            #[id = 100]
            #[lower = sine]
            struct Sine {
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        define_processor!(
            #[doc = " Mostly accurate, but faster sine oscillator\n\n # frequency\n\n # phase (trigger)\n"]
            #[id = 101]
            #[lower = sine_fast]
            struct SineFast {
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        define_processor!(
            #[doc = " Less accurate, but fast sine oscillator\n\n # frequency\n\n # phase (trigger)\n"]
            #[id = 102]
            #[lower = sine_faster]
            struct SineFaster {
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        define_processor!(
            #[doc = " A pulse (square) oscillator\n\n # frequency\n\n # phase (trigger)\n"]
            #[id = 103]
            #[lower = pulse]
            struct Pulse {
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        define_processor!(
            #[doc = " A sawtooth oscillator\n\n # frequency\n\n # phase (trigger)\n"]
            #[id = 104]
            #[lower = sawtooth]
            struct Sawtooth {
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        define_processor!(
            #[doc = " A triangle oscillator\n\n # frequency\n\n # phase (trigger)\n"]
            #[id = 105]
            #[lower = triangle]
            struct Triangle {
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        define_processor!(
            #[id = 106]
            #[lower = silence]
            struct Silence {
            }
        );

        define_processor!(
            #[id = 107]
            #[lower = phase]
            struct Phase {
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        define_processor!(
            #[id = 114]
            #[lower = wave]
            struct Wave {
                #[buffer]
                #[trait = BufferInput]
                #[with = with_buffer]
                #[set = set_buffer]
                buffer: Buffer<0>,
                #[trait = FrequencyInput]
                #[with = with_frequency]
                #[set = set_frequency]
                frequency: Parameter<0>,
                #[trait = PhaseInput]
                #[with = with_phase]
                #[set = set_phase]
                phase: f64<1>,
            }
        );

        pub mod nes {
            define_processor!(
                #[id = 108]
                #[lower = pulse]
                struct Pulse {
                    #[trait = FrequencyInput]
                    #[with = with_frequency]
                    #[set = set_frequency]
                    frequency: Parameter<0>,
                    #[trait = DutyCycleInput]
                    #[with = with_duty_cycle]
                    #[set = set_duty_cycle]
                    duty_cycle: Parameter<1>,
                    #[trait = DecayInput]
                    #[with = with_decay]
                    #[set = set_decay]
                    decay: Parameter<2>,
                    #[trait = PhaseInput]
                    #[with = with_phase]
                    #[set = set_phase]
                    phase: f64<3>,
                }
            );

            define_processor!(
                #[id = 109]
                #[lower = triangle]
                struct Triangle {
                    #[trait = FrequencyInput]
                    #[with = with_frequency]
                    #[set = set_frequency]
                    frequency: Parameter<0>,
                    #[trait = PhaseInput]
                    #[with = with_phase]
                    #[set = set_phase]
                    phase: f64<1>,
                }
            );
        }
        pub mod noise {
            define_processor!(
                #[doc = " White noise generator.\n"]
                #[id = 110]
                #[lower = white]
                struct White {
                    #[trait = SeedInput]
                    #[with = with_seed]
                    #[set = set_seed]
                    seed: f64<0>,
                }
            );

            define_processor!(
                #[doc = " Maximum Length Sequence noise generator from an `n`-bit sequence.\n"]
                #[id = 111]
                #[lower = mls]
                struct Mls {
                    #[trait = SeedInput]
                    #[with = with_seed]
                    #[set = set_seed]
                    seed: f64<0>,
                    #[trait = LengthInput]
                    #[with = with_length]
                    #[set = set_length]
                    length: f64<1>,
                }
            );

            define_processor!(
                #[doc = " Pink noise generator.\n"]
                #[id = 112]
                #[lower = pink]
                struct Pink {
                    #[trait = SeedInput]
                    #[with = with_seed]
                    #[set = set_seed]
                    seed: f64<0>,
                }
            );

            define_processor!(
                #[doc = " Brown noise generator.\n"]
                #[id = 113]
                #[lower = brown]
                struct Brown {
                    #[trait = SeedInput]
                    #[with = with_seed]
                    #[set = set_seed]
                    seed: f64<0>,
                }
            );
        }
    }
    pub mod tertiary {
        define_processor!(
            #[doc = " Fused multiply-add. Computes `(input + add) * mul` with only one rounding\n error, yielding a more accurate result than an unfused add-multiply.\n"]
            #[id = 75]
            #[lower = add_mul]
            struct AddMul {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
                #[trait = AddInput]
                #[with = with_add]
                #[set = set_add]
                add: Parameter<1>,
                #[trait = MulInput]
                #[with = with_mul]
                #[set = set_mul]
                mul: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Restrict a value to a certain interval unless it is NaN.\n\n Returns `max` if `input` is greater than `max`, and `min` if `input` is\n less than `min`. Otherwise this returns `input`.\n\n Note that this function returns NaN if the initial value was NaN as\n well or `min > max`\n"]
            #[id = 76]
            #[lower = clamp]
            struct Clamp {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
                #[trait = MinInput]
                #[with = with_min]
                #[set = set_min]
                min: Parameter<1>,
                #[trait = MaxInput]
                #[with = with_max]
                #[set = set_max]
                max: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " Fused multiply-add. Computes `(input * mul) + add` with only one rounding\n error, yielding a more accurate result than an unfused multiply-add.\n"]
            #[id = 77]
            #[lower = mul_add]
            struct MulAdd {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
                #[trait = MulInput]
                #[with = with_mul]
                #[set = set_mul]
                mul: Parameter<1>,
                #[trait = AddInput]
                #[with = with_add]
                #[set = set_add]
                add: Parameter<2>,
            }
        );

        define_processor!(
            #[doc = " If `cond` is positive, then `positive` is returned. Otherwise `negative`\n is returned.\n"]
            #[id = 78]
            #[lower = select]
            struct Select {
                #[trait = CondInput]
                #[with = with_cond]
                #[set = set_cond]
                cond: Parameter<0>,
                #[trait = PositiveInput]
                #[with = with_positive]
                #[set = set_positive]
                positive: Parameter<1>,
                #[trait = NegativeInput]
                #[with = with_negative]
                #[set = set_negative]
                negative: Parameter<2>,
            }
        );
    }
    pub mod unary {
        define_processor!(
            #[doc = " Computes the absolute value of `input`. Returns `NAN` if the number is `NAN`.\n"]
            #[id = 1]
            #[lower = abs]
            struct Abs {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Computes the arccosine of a number. Return value is in radians in the\n range [0, pi] or NaN if the number is outside the range [-1, 1].\n"]
            #[id = 2]
            #[lower = acos]
            struct Acos {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Inverse hyperbolic cosine function.\n"]
            #[id = 3]
            #[lower = acosh]
            struct Acosh {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Computes the arcsine of a number. Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].\n"]
            #[id = 4]
            #[lower = asin]
            struct Asin {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Inverse hyperbolic sine function.\n"]
            #[id = 5]
            #[lower = asinh]
            struct Asinh {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Computes the arctangent of a number. Return value is in radians in the range [-pi/2, pi/2];\n"]
            #[id = 6]
            #[lower = atan]
            struct Atan {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Inverse hyperbolic tangent function.\n"]
            #[id = 7]
            #[lower = atanh]
            struct Atanh {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the cube root of a number.\n"]
            #[id = 8]
            #[lower = cbrt]
            struct Cbrt {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the smallest integer greater than or equal to a number.\n"]
            #[id = 9]
            #[lower = ceil]
            struct Ceil {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Computes the cosine of a number (in radians).\n"]
            #[id = 10]
            #[lower = cos]
            struct Cos {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Hyperbolic cosine function.\n"]
            #[id = 11]
            #[lower = cosh]
            struct Cosh {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns `e^(self)`, (the exponential function).\n"]
            #[id = 12]
            #[lower = exp]
            struct Exp {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns `2^(self)`\n"]
            #[id = 13]
            #[lower = exp2]
            struct Exp2 {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.\n"]
            #[id = 14]
            #[lower = exp_m1]
            struct ExpM1 {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the largest integer less than or equal to a number.\n"]
            #[id = 15]
            #[lower = floor]
            struct Floor {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the fractional part of a number.\n"]
            #[id = 16]
            #[lower = fract]
            struct Fract {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the natural logarithm of the number.\n"]
            #[id = 17]
            #[lower = ln]
            struct Ln {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns ln(1+n) (natural logarithm) more accurately than if the operations\n were performed separately.\n"]
            #[id = 18]
            #[lower = ln1p]
            struct Ln1p {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the base 10 logarithm of the number.\n"]
            #[id = 19]
            #[lower = log10]
            struct Log10 {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the base 2 logarithm of the number.\n"]
            #[id = 20]
            #[lower = log2]
            struct Log2 {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Normalizes a number.\n\n * `-0.0` will be converted into `0.0`\n * `NAN` will be converted into `0.0`\n * `INFINITY` will be converted into `MAX`\n * `NEG_INFINITY` will be converted into `MIN`\n"]
            #[id = 21]
            #[lower = norm]
            struct Norm {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Takes the reciprocal (inverse) of a number, 1/x.\n"]
            #[id = 22]
            #[lower = recip]
            struct Recip {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the nearest integer to a number. Round half-way cases away from `0.0`.\n"]
            #[id = 23]
            #[lower = round]
            struct Round {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns a number that represents the sign of `self`.\n\n * `1.0` if the number is positive, `+0.0` or `INFINITY`\n * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`\n * `NAN` if the number is `NAN`\n"]
            #[id = 24]
            #[lower = signum]
            struct Signum {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Computes the sine of a number (in radians).\n"]
            #[id = 25]
            #[lower = sin]
            struct Sin {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Hyperbolic sine function.\n"]
            #[id = 26]
            #[lower = sinh]
            struct Sinh {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the square root of a number.\n\n Returns `NaN` if `self` is a negative number other than `-0.0`.\n"]
            #[id = 27]
            #[lower = sqrt]
            struct Sqrt {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Computes the tangent of a number (in radians).\n"]
            #[id = 28]
            #[lower = tan]
            struct Tan {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Hyperbolic tangent function.\n"]
            #[id = 29]
            #[lower = tanh]
            struct Tanh {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Converts radians to degrees.\n"]
            #[id = 30]
            #[lower = to_degrees]
            struct ToDegrees {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Converts degrees to radians.\n"]
            #[id = 31]
            #[lower = to_radians]
            struct ToRadians {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " Returns the integer part of a number.\n"]
            #[id = 32]
            #[lower = trunc]
            struct Trunc {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );

        define_processor!(
            #[doc = " The unary negation operator `-`.\n"]
            #[id = 33]
            #[lower = neg]
            struct Neg {
                #[trait = InputInput]
                #[with = with_input]
                #[set = set_input]
                input: Parameter<0>,
            }
        );
    }
}
pub use api::*;
