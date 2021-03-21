// http://doc.sccode.org/Reference/Synth-Definition-File-Format.html

use self::Version::*;
use codec::{
    buffer::{self, FiniteBuffer},
    decode::{Decoder, DecoderBuffer, TypeDecoder},
};

macro_rules! decode_vec {
    ($buffer:expr, @version $version:expr) => {{
        let (len, buffer) = $version.decode_int($buffer)?;
        decode_vec!(len, buffer)
    }};
    ($buffer:expr, < $ty:ty >) => {{
        let (len, buffer) = $buffer.decode::<$ty>()?;
        decode_vec!(len, buffer)
    }};
    ($len:expr, $buffer:expr) => {{
        let mut buffer = $buffer;
        let mut values = vec![];
        for _ in 0..$len {
            let (v, new_buffer) = buffer.decode()?;
            values.push(v);
            buffer = new_buffer;
        }
        (values, buffer)
    }};
}

macro_rules! decode_vec_with {
    ($buffer:expr, @version $param:expr) => {{
        let (len, buffer) = $param.decode_int($buffer)?;
        decode_vec_with!(len, buffer, $param)
    }};
    ($buffer:expr, @len $ty:ty, $param:expr) => {{
        let (len, buffer) = $buffer.decode::<$ty>()?;
        decode_vec_with!(len, buffer, $param)
    }};
    ($len:expr, $buffer:expr, $param:expr) => {{
        let mut buffer = $buffer;
        let mut values = vec![];
        for _ in 0..$len {
            let (v, new_buffer) = buffer.decode_with($param)?;
            values.push(v);
            buffer = new_buffer;
        }
        (values, buffer)
    }};
}

#[derive(Clone, Debug, PartialEq)]
pub struct Container {
    pub version: i32,
    pub defs: Vec<Definition>,
}

const ID: i32 = i32::from_be_bytes(*b"SCgf");

impl<B: DecoderBuffer + FiniteBuffer> TypeDecoder<B> for Container {
    fn decode_type(buffer: B) -> buffer::Result<Self, B> {
        let (id, buffer) = buffer.decode::<i32>()?;

        if id != ID {
            return Err(buffer::BufferError {
                buffer,
                reason: buffer::BufferErrorReason::InvalidValue {
                    message: "invalid container ID",
                },
            });
        }

        let (version, buffer) = buffer.decode()?;

        let (defs, buffer) = match version {
            1i32 => {
                decode_vec_with!(buffer, @len i16, V1)
            }
            2 => {
                decode_vec_with!(buffer, @len i16, V2)
            }
            _ => {
                return Err(buffer::BufferError {
                    buffer,
                    reason: buffer::BufferErrorReason::InvalidValue {
                        message: "invalid version",
                    },
                });
            }
        };

        let value = Self { version, defs };

        Ok((value, buffer))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    V1,
    V2,
}

impl Version {
    fn decode_int<B: DecoderBuffer + FiniteBuffer>(self, buffer: B) -> buffer::Result<i32, B> {
        let (len, buffer) = match self {
            Self::V1 => {
                let (len, buffer) = buffer.decode::<i16>()?;
                (len as i32, buffer)
            }
            Self::V2 => {
                let (len, buffer) = buffer.decode()?;
                (len, buffer)
            }
        };
        Ok((len, buffer))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub name: String,
    pub consts: Vec<f32>,
    pub params: Vec<f32>,
    pub param_names: Vec<ParamName>,
    pub ugens: Vec<UGen>,
    pub variants: Vec<Variant>,
}

impl<B: DecoderBuffer + FiniteBuffer> TypeDecoder<B> for Definition {
    fn decode_type(buffer: B) -> buffer::Result<Self, B> {
        buffer.decode_with(V2)
    }
}

impl<B: DecoderBuffer + FiniteBuffer> Decoder<Definition, B> for Version {
    fn decode_from(self, buffer: B) -> buffer::Result<Definition, B> {
        let (name, buffer) = decode_pstring(buffer)?;

        let (consts, buffer) = decode_vec!(buffer, @version self);
        let (params, buffer) = decode_vec!(buffer, @version self);
        let (param_names, buffer) = decode_vec_with!(buffer, @version self);
        let (ugens, buffer) = decode_vec_with!(buffer, @version self);
        let (variants, buffer) = decode_vec_with!(buffer, @len i16, params.len());

        let value = Definition {
            name,
            consts,
            params,
            param_names,
            ugens,
            variants,
        };

        Ok((value, buffer))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParamName {
    pub name: String,
    pub index: i32,
}

impl<B: DecoderBuffer + FiniteBuffer> TypeDecoder<B> for ParamName {
    fn decode_type(buffer: B) -> buffer::Result<Self, B> {
        buffer.decode_with(V2)
    }
}

impl<B: DecoderBuffer + FiniteBuffer> Decoder<ParamName, B> for Version {
    fn decode_from(self, buffer: B) -> buffer::Result<ParamName, B> {
        let (name, buffer) = decode_pstring(buffer)?;
        let (index, buffer) = self.decode_int(buffer)?;
        let value = ParamName { name, index };

        Ok((value, buffer))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UGen {
    pub name: String,
    pub rate: CalculationRate,
    pub ins: Vec<Input>,
    pub outs: Vec<CalculationRate>,
    pub special_index: i16,
}

impl<B: DecoderBuffer + FiniteBuffer> TypeDecoder<B> for UGen {
    fn decode_type(buffer: B) -> buffer::Result<Self, B> {
        buffer.decode_with(V2)
    }
}

impl<B: DecoderBuffer + FiniteBuffer> Decoder<UGen, B> for Version {
    fn decode_from(self, buffer: B) -> buffer::Result<UGen, B> {
        let (name, buffer) = decode_pstring(buffer)?;
        let (rate, buffer) = buffer.decode()?;
        let (in_len, buffer) = self.decode_int(buffer)?;
        let (out_len, buffer) = self.decode_int(buffer)?;
        let (special_index, buffer) = buffer.decode()?;

        let (ins, buffer) = decode_vec_with!(in_len, buffer, self);
        let (outs, buffer) = decode_vec!(out_len, buffer);

        let value = UGen {
            name,
            rate,
            ins,
            outs,
            special_index,
        };

        Ok((value, buffer))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Input {
    UGen { index: i32, output: i32 },
    Constant { index: i32 },
}

impl<B: DecoderBuffer + FiniteBuffer> TypeDecoder<B> for Input {
    fn decode_type(buffer: B) -> buffer::Result<Self, B> {
        buffer.decode_with(V2)
    }
}

impl<B: DecoderBuffer + FiniteBuffer> Decoder<Input, B> for Version {
    fn decode_from(self, buffer: B) -> buffer::Result<Input, B> {
        let (index, buffer) = self.decode_int(buffer)?;
        let (value, buffer) = self.decode_int(buffer)?;

        let value = match index {
            -1 => Input::Constant { index: value },
            _ => Input::UGen {
                index,
                output: value,
            },
        };

        Ok((value, buffer))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub name: String,
    pub params: Vec<f32>,
}

impl<B: DecoderBuffer + FiniteBuffer> Decoder<Variant, B> for usize {
    fn decode_from(self, buffer: B) -> buffer::Result<Variant, B> {
        let (name, buffer) = decode_pstring(buffer)?;
        let (params, buffer) = decode_vec!(self, buffer);

        let value = Variant { name, params };
        Ok((value, buffer))
    }
}

fn decode_pstring<B: DecoderBuffer + FiniteBuffer>(buffer: B) -> buffer::Result<String, B> {
    let (len, buffer) = buffer.decode::<u8>()?;
    let (bytes, buffer) = buffer.checked_split(len as usize)?;

    match core::str::from_utf8(bytes.as_less_safe_slice()) {
        Ok(value) => Ok((value.to_owned(), buffer)),
        Err(_) => Err(buffer::BufferError {
            buffer,
            reason: buffer::BufferErrorReason::InvalidValue {
                message: "invalid string encoding",
            },
        }),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum CalculationRate {
    /// one sample is computed at initialization time only.
    Scalar = 0,
    /// one sample is computed each control period.
    Control = 1,
    /// one sample is computed for each sample of audio output.
    Audio = 2,
    /// on demand
    Demand = 3,
}

impl<B: DecoderBuffer> TypeDecoder<B> for CalculationRate {
    fn decode_type(buffer: B) -> buffer::Result<Self, B> {
        let (value, buffer) = buffer.decode()?;
        let value = match value {
            0i8 => Self::Scalar,
            1i8 => Self::Control,
            2i8 => Self::Audio,
            _ => {
                return Err(buffer::BufferError {
                    buffer,
                    reason: buffer::BufferErrorReason::InvalidValue {
                        message: "invalid calculation rate",
                    },
                })
            }
        };
        Ok((value, buffer))
    }
}

// https://github.com/overtone/overtone/blob/master/src/overtone/sc/machinery/ugen/special_ops.clj

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum UnaryOps {
    Neg = 0,
    NotPos = 1,
    IsNil = 2,
    NotNil = 3,
    BitNot = 4,
    Abs = 5,
    AsFloat = 6,
    AsInt = 7,
    Ceil = 8,
    Floor = 9,
    Frac = 10,
    Sign = 11,
    Squared = 12,
    Cubed = 13,
    Sqrt = 14,
    Exp = 15,
    Reciprocal = 16,
    MidiCps = 17,
    CpsMidi = 18,
    MidiRatio = 19,
    RatioMidi = 20,
    DbAmp = 21,
    AmpDb = 22,
    OctCps = 23,
    CpsOct = 24,
    Log = 25,
    Log2 = 26,
    Log10 = 27,
    Sin = 28,
    Cos = 29,
    Tan = 30,
    Asin = 31,
    Acos = 32,
    Atan = 33,
    Sinh = 34,
    Cosh = 35,
    Tanh = 36,
    Rand = 37,
    Rand2 = 38,
    LinRand = 39,
    BilinRand = 40,
    Sum3Rand = 41,
    Distort = 42,
    Softclip = 43,
    Coin = 44,
    DigitVal = 45,
    Silence = 46,
    Thru = 47,
    RectangularWindow = 48,
    HanningWindow = 49,
    WelchWindow = 50,
    TriangleWindow = 51,
    Ramp = 52,
    SCurve = 53,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum BinaryOps {
    Add = 0,
    Subtract = 1,
    Multiply = 2,
    /// Not implemented on the server
    Div = 3,
    Divide = 4,
    Modulus = 5,
    Equal = 6,
    NotEqual = 7,
    LessThan = 8,
    GreaterThan = 9,
    LessThanOrEqual = 10,
    GreaterThanOrEqual = 11,
    Minimum = 12,
    Maximum = 13,
    And = 14,
    Or = 15,
    Xor = 16,
    /// Not implemented on the server
    Lcm = 17,
    /// Not implemented on the server
    Gcd = 18,
    Round = 19,
    RoundUp = 20,
    RoundDown = 21,
    Atan2 = 22,
    Hypotenuse = 23,
    HypotenuseApprox = 24,
    Pow = 25,
    /// Not implemented on the server
    LeftShift = 26,
    /// Not implemented on the server
    RightShift = 27,
    /// Not implemented on the server
    UnRightShift = 28,
    /// Not implemented on the server
    Fill = 29,
    Ring1 = 30,
    Ring2 = 31,
    Ring3 = 32,
    Ring4 = 33,
    DifSqr = 34,
    SqrSum = 36,
    SqrDif = 37,
    AbsDif = 38,
    Thresh = 39,
    AmClip = 40,
    ScaleNeg = 41,
    Clip2 = 42,
    Excess = 43,
    Fold2 = 44,
    Wrap2 = 45,
    FirstArg = 46,
    /// Not implemented on the server
    RRand = 47,
    /// Not implemented on the server
    ExpRand = 48,
}

// TODO port https://github.com/overtone/overtone/tree/master/src/overtone/sc/machinery/ugen/metadata

#[cfg(test)]
mod tests {
    use super::*;

    static V1: &[u8] = include_bytes!("../artifacts/v1.scsyndef");
    static V2: &[u8] = include_bytes!("../artifacts/v2.scsyndef");

    macro_rules! snap {
        ($name:ident, $contents:expr) => {
            #[test]
            fn $name() {
                let (container, _) = $contents.decode::<Container>().unwrap();
                insta::assert_debug_snapshot!(container);
            }
        };
    }

    snap!(v1, V1);
    snap!(v2, V2);
}
