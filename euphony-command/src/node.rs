use crate::{api, message::NodeValue};
use euphony_core_macros::dispatch_struct;
use euphony_dsp::signal::generator::Phase;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T, E = Error> = core::result::Result<T, E>;

pub const BUF_LEN: usize = 4096;
pub const CHUNK_SIZE: usize = 128 / 32;
type SampleRate = euphony_dsp::sample::Rate48000;

pub trait Constructor {
    fn load<L: Loader>(&self, loader: &mut L) -> Result<Box<dyn Node>>;
}

pub trait Node {
    fn fill(
        &mut self,
        buffers: &[&[f32]],
        inputs: &[&[f32; BUF_LEN]],
        outputs: &mut [&mut [f32; BUF_LEN]],
    );
}

pub trait NodeIter<'a> {
    type Iter: Iterator<Item = f32>;

    fn iter(&self, buffers: &'a [&[f32]], inputs: &'a [&[f32; BUF_LEN]]) -> Self::Iter;
}

pub trait Loader {
    fn mul(&mut self) -> Option<usize>;
    fn add(&mut self) -> Option<usize>;
    fn input(&mut self, id: u32) -> Result<usize>;
    fn node(&mut self, id: u32, output: u32) -> Result<usize>;
    fn buffer(&mut self, id: u32) -> Result<usize>;
}

impl Constructor for api::Sine {
    fn load<L: Loader>(&self, loader: &mut L) -> Result<Box<dyn Node>> {
        Ok(dispatch_struct!(SineNode {
            iter: Phase::default(),
            freq: match self.frequency {
                NodeValue::Constant { value: freq } if freq == 0.0 => Zero,
                NodeValue::Constant { value: freq } => Constant { value: freq },
                NodeValue::Node {
                    id: freq_id,
                    output: freq_output,
                } => Input {
                    index: loader.node(freq_id, freq_output)?,
                },
                NodeValue::Input { id: freq } => Input {
                    index: loader.input(freq)?,
                },
                NodeValue::Buffer { id: freq } => Buffer {
                    index: loader.buffer(freq)?,
                },
            },
            phase: match self.phase {
                NodeValue::Constant { value: phase } if phase == 0.0 => Zero,
                NodeValue::Constant { value: phase } => Constant { value: phase },
                NodeValue::Node {
                    id: phase_id,
                    output: phase_output,
                } => Input {
                    index: loader.node(phase_id, phase_output)?,
                },
                NodeValue::Input { id: phase_id } => Input {
                    index: loader.input(phase_id)?,
                },
                NodeValue::Buffer { id: phase_id } => Buffer {
                    index: loader.buffer(phase_id)?,
                },
            },
            mul: match loader.mul() {
                Some(mul) => Mul {
                    input: mul,
                    output: 0,
                },
                None => Noop,
            },
            add: match loader.add() {
                Some(add) => Add {
                    input: add,
                    output: 0,
                },
                None => Noop,
            },
        }))
    }
}

pub struct SineNode<Freq, Phz, Mul, Add> {
    iter: Phase<f32>,
    freq: Freq,
    phase: Phz,
    mul: Mul,
    add: Add,
}

/// Asserts that a boolean expression is true at runtime, only if debug_assertions are enabled.
///
/// Otherwise, the compiler is told to assume that the expression is always true and can perform
/// additional optimizations.
macro_rules! unsafe_assert {
    ($cond:expr) => {
        unsafe_assert!($cond, "assumption failed: {}", stringify!($cond));
    };
    ($cond:expr $(, $fmtarg:expr)* $(,)?) => {{
        let v = $cond;

        debug_assert!(v $(, $fmtarg)*);
        if cfg!(not(debug_assertions)) && !v {
            core::hint::unreachable_unchecked();
        }
    }};
}

impl<Freq, Phz, Mul, Add> Node for SineNode<Freq, Phz, Mul, Add>
where
    Freq: for<'a> NodeIter<'a>,
    Phz: for<'a> NodeIter<'a>,
    Mul: Node,
    Add: Node,
{
    #[inline]
    fn fill(
        &mut self,
        buffers: &[&[f32]],
        inputs: &[&[f32; BUF_LEN]],
        outputs: &mut [&mut [f32; BUF_LEN]],
    ) {
        let freq = self.freq.iter(buffers, inputs);
        let phase = self.phase.iter(buffers, inputs);
        let mut params = freq.zip(phase);
        let output = unsafe { outputs.get_unchecked_mut(0) };

        for chunk in output.chunks_mut(CHUNK_SIZE) {
            unsafe {
                unsafe_assert!(chunk.len() == CHUNK_SIZE);
            }

            let mut samples = [0.0f32; CHUNK_SIZE];
            for (sample, (freq, phase)) in samples.iter_mut().zip(&mut params) {
                let value = self.iter.next::<SampleRate>(freq);
                *sample = fastapprox::fast::sinfull(value * core::f32::consts::TAU + phase);
            }
            chunk.copy_from_slice(&samples);
        }

        self.mul.fill(buffers, inputs, outputs);
        self.add.fill(buffers, inputs, outputs);
    }
}

struct Mul {
    input: usize,
    output: usize,
}

impl Node for Mul {
    #[inline]
    fn fill(
        &mut self,
        _: &[&[f32]],
        input: &[&[f32; BUF_LEN]],
        output: &mut [&mut [f32; BUF_LEN]],
    ) {
        let input = unsafe { input.get_unchecked(self.input) };
        let output = unsafe { output.get_unchecked_mut(self.output) };
        for (input, output) in input.iter().zip(output.iter_mut()) {
            *output *= input;
        }
    }
}

struct Add {
    input: usize,
    output: usize,
}

impl Node for Add {
    #[inline]
    fn fill(
        &mut self,
        _: &[&[f32]],
        input: &[&[f32; BUF_LEN]],
        output: &mut [&mut [f32; BUF_LEN]],
    ) {
        let input = unsafe { input.get_unchecked(self.input) };
        let output = unsafe { output.get_unchecked_mut(self.output) };
        for (input, output) in input.iter().zip(output.iter_mut()) {
            *output += input;
        }
    }
}

struct Noop;

impl Node for Noop {
    #[inline]
    fn fill(&mut self, _: &[&[f32]], _: &[&[f32; BUF_LEN]], _: &mut [&mut [f32; BUF_LEN]]) {}
}

#[derive(Clone, Copy)]
struct Zero;

impl<'a> NodeIter<'a> for Zero {
    type Iter = Self;

    #[inline]
    fn iter(&self, _buffers: &'a [&[f32]], _inputs: &'a [&[f32; BUF_LEN]]) -> Self::Iter {
        *self
    }
}

impl Iterator for Zero {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(0.0)
    }
}

#[derive(Clone, Copy)]
struct Constant {
    value: f32,
}

impl<'a> NodeIter<'a> for Constant {
    type Iter = Self;

    #[inline]
    fn iter(&self, _buffers: &'a [&[f32]], _inputs: &'a [&[f32; BUF_LEN]]) -> Self::Iter {
        *self
    }
}

impl Iterator for Constant {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

struct Input {
    index: usize,
}

impl<'a> NodeIter<'a> for Input {
    type Iter = core::iter::Copied<core::slice::Iter<'a, f32>>;

    #[inline]
    fn iter(&self, _buffers: &'a [&[f32]], inputs: &'a [&[f32; BUF_LEN]]) -> Self::Iter {
        let input = unsafe { inputs.get_unchecked(self.index) };
        input.iter().copied()
    }
}

struct Buffer {
    index: usize,
}

impl<'a> NodeIter<'a> for Buffer {
    type Iter = core::iter::Cycle<core::iter::Copied<core::slice::Iter<'a, f32>>>;

    #[inline]
    fn iter(&self, buffers: &'a [&[f32]], _inputs: &'a [&[f32; BUF_LEN]]) -> Self::Iter {
        let input = unsafe { buffers.get_unchecked(self.index) };
        input.iter().copied().cycle()
    }
}
