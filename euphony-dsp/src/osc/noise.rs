use crate::{
    fun::{self, an, AudioNode},
    prelude::*,
};

#[derive(Node)]
#[node(id = 110, module = "osc::noise")]
#[input(seed, trigger = set_seed)]
/// White noise generator.
pub struct White {
    inner: fun::Noise<f64>,
}

impl Default for White {
    fn default() -> Self {
        Self {
            inner: an(fun::white()),
        }
    }
}

impl White {
    fn set_seed(&mut self, value: f64) {
        self.inner.set_hash(value.to_bits());
    }

    #[inline]
    pub fn render(&mut self, output: &mut [Sample]) {
        for output in output.iter_mut() {
            *output = self.inner.tick(&Default::default())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 111, module = "osc::noise")]
#[input(seed, trigger = set_seed)]
#[input(length, trigger = set_length)]
/// Maximum Length Sequence noise generator from an `n`-bit sequence.
pub struct Mls {
    inner: fun::Mls<f64>,
}

impl Default for Mls {
    fn default() -> Self {
        Self {
            inner: an(fun::mls()),
        }
    }
}

impl Mls {
    fn set_seed(&mut self, value: f64) {
        self.inner.set_hash(value.to_bits());
    }

    fn set_length(&mut self, value: f64) {
        self.inner = an(fun::mls_bits(value as _));
    }

    #[inline]
    pub fn render(&mut self, output: &mut [Sample]) {
        for output in output.iter_mut() {
            *output = self.inner.tick(&Default::default())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 112, module = "osc::noise")]
#[input(seed, trigger = set_seed)]
/// Pink noise generator.
pub struct Pink {
    inner: fun::Pipe<f64, fun::Noise<f64>, fun::Pinkpass<f64, f64>>,
}

impl Default for Pink {
    fn default() -> Self {
        Self {
            inner: an(fun::pink()),
        }
    }
}

impl Pink {
    fn set_seed(&mut self, value: f64) {
        self.inner.set_hash(value.to_bits());
    }

    #[inline]
    pub fn render(&mut self, output: &mut [f64]) {
        for output in output.iter_mut() {
            *output = self.inner.tick(&Default::default())[0];
        }
    }
}

type BrownP = fun::Pipe<
    f64,
    fun::Noise<f64>,
    fun::Binop<
        f64,
        fun::FrameMul<fun::U1, f64>,
        fun::Lowpole<f64, f64, fun::U1>,
        fun::Constant<fun::U1, f64>,
    >,
>;

#[derive(Node)]
#[node(id = 113, module = "osc::noise")]
#[input(seed, trigger = set_seed)]
/// Brown noise generator.
pub struct Brown {
    inner: BrownP,
}

impl Default for Brown {
    fn default() -> Self {
        Self {
            inner: an(fun::brown()),
        }
    }
}

impl Brown {
    fn set_seed(&mut self, value: f64) {
        self.inner.set_hash(value.to_bits());
    }

    #[inline]
    pub fn render(&mut self, output: &mut [f64]) {
        for output in output.iter_mut() {
            *output = self.inner.tick(&Default::default())[0];
        }
    }
}
