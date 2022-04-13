// based on https://xoshiro.di.unimi.it/xoshiro256plusplus.c
//static inline uint64_t rotl(const uint64_t x, int k) {
//	return (x << k) | (x >> (64 - k));
//}
//
//
//static uint64_t s[4];
//
//uint64_t next(void) {
//	const uint64_t result = rotl(s[0] + s[3], 23) + s[0];
//
//	const uint64_t t = s[1] << 17;
//
//	s[2] ^= s[0];
//	s[3] ^= s[1];
//	s[1] ^= s[2];
//	s[0] ^= s[3];
//
//	s[2] ^= t;
//
//	s[3] = rotl(s[3], 45);
//
//	return result;
//}

use core::num::Wrapping as W;
use euphony_core::time::beat::Instant;
use rand_core::RngCore;

struct State([W<u64>; 4]);

impl State {
    fn new(now: Instant, expansion: u64, seed: u64) -> Self {
        Self([
            mix(W(now.0), THING_A),
            mix(W(seed), SPLIT_MIX),
            mix(W(now.1), THING_B),
            mix(W(expansion), THING_C),
        ])
    }

    fn rotate(&mut self) {
        let state = &mut self.0;
        let t = state[1] << 17;

        state[2] ^= state[0];
        state[3] ^= state[1];
        state[1] ^= state[2];
        state[0] ^= state[3];

        state[2] ^= t;
        state[3] = rotl(state[3], 45);
    }

    fn read(&self) -> u64 {
        let state = &self.0;

        let result = rotl(state[0] + state[3], 23) + state[0];

        result.0
    }
}

impl RngCore for State {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as _
    }

    fn next_u64(&mut self) -> u64 {
        self.rotate();
        self.read()
    }

    fn fill_bytes(&mut self, _dest: &mut [u8]) {
        todo!()
    }

    fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> {
        todo!()
    }
}

pub fn get(now: Instant, expansion: u64, seed: u64) -> u64 {
    let mut state = State::new(now, expansion, seed);
    state.rotate();
    state.read()
}

fn rotl(x: W<u64>, k: usize) -> W<u64> {
    (x << k) | (x >> (64 - k))
}

// https://prng.di.unimi.it/splitmix64.c
// uint64_t next() {
//	uint64_t z = (x += 0x9e3779b97f4a7c15);
//	z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
//	z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
//	return z ^ (z >> 31);
//}
fn mix(x: W<u64>, state: [u64; 3]) -> W<u64> {
    let mut z = x + W(state[0]);
    z = (z ^ (z >> 30)) * W(state[1]);
    z = (z ^ (z >> 27)) * W(state[2]);
    z = z ^ (z >> 31);
    z
}

const SPLIT_MIX: [u64; 3] = [0x9e3779b97f4a7c15, 0xbf58476d1ce4e5b9, 0x94d049bb133111eb];
const THING_A: [u64; 3] = [0x3c8cc7219755f437, 0x671dbd34af54e996, 0x919652ae67f1d181];
const THING_B: [u64; 3] = [0x9326a101ae57df9f, 0xdd5a3fbfc88ba5d1, 0x2822711d222b55dc];
const THING_C: [u64; 3] = [0x8f91ea12c6653f76, 0xa19b234057f9b967, 0x8464e496f81bb3f9];
