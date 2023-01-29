use crate::prelude::*;
use ahash::RandomState;
use noise::permutationtable::NoiseHasher;

struct SingleHasher(RandomState);

impl SingleHasher {
    #[inline]
    fn new(seed: f64) -> Self {
        const PI: [u64; 4] = [
            0x243f_6a88_85a3_08d3,
            0x1319_8a2e_0370_7344,
            0xa409_3822_299f_31d0,
            0x082e_fa98_ec4e_6c89,
        ];

        Self(RandomState::with_seeds(
            PI[0],
            PI[1],
            PI[2],
            seed.to_bits() ^ PI[3],
        ))
    }
}

impl NoiseHasher for SingleHasher {
    #[inline]
    fn hash(&self, to_hash: &[isize]) -> usize {
        self.0.hash_one(to_hash) as _
    }
}

struct Hasher(ahash::AHasher);

impl Hasher {
    #[inline]
    fn new(seed: f64) -> Self {
        use core::hash::BuildHasher;
        Self(SingleHasher::new(seed).0.build_hasher())
    }
}

impl NoiseHasher for Hasher {
    #[inline]
    fn hash(&self, to_hash: &[isize]) -> usize {
        use core::hash::{Hash, Hasher};
        let mut hasher = self.0.clone();
        to_hash.hash(&mut hasher);
        hasher.finish() as _
    }
}

macro_rules! define_noise {
    ($name:ident, $id:literal, $fun:path) => {
        #[derive(Debug, Default, Node)]
        #[node(id = $id, module = "noise")]
        #[input(x)]
        #[input(y)]
        #[input(z)]
        #[input(w)]
        #[input(seed)]
        pub struct $name;

        impl $name {
            #[inline]
            pub fn render(
                &mut self,
                x: Input,
                y: Input,
                z: Input,
                w: Input,
                seed: Input,
                output: &mut [Sample],
            ) {
                match seed {
                    Input::Constant(seed) => {
                        let hasher = Hasher::new(seed);
                        for (x, y, z, w, output) in (x, y, z, w, output.iter_mut()).zip() {
                            *output = $fun([x, y, z, w], &hasher);
                        }
                    }
                    Input::Buffer(seed) => {
                        for (x, y, z, w, seed, output) in (x, y, z, w, seed, output).zip() {
                            let hasher = SingleHasher::new(*seed);
                            *output = $fun([x, y, z, w], &hasher);
                        }
                    }
                }
            }
        }
    };
}

#[inline(always)]
fn simplex_4d<H: NoiseHasher>(coords: [f64; 4], hasher: &H) -> f64 {
    ::noise::core::simplex::simplex_4d(coords, hasher).0
}

define_noise!(Simplex, 150, simplex_4d);
define_noise!(Perlin, 151, ::noise::core::perlin::perlin_4d);
define_noise!(
    OpenSimplex,
    152,
    ::noise::core::open_simplex::open_simplex_4d
);
