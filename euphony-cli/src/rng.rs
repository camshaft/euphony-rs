use core::ops::Range;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::cell::RefCell;

thread_local! {
    static SEED: u64 = if let Ok(seed) = std::env::var("EUPHONY_SEED") {
        u64::from_str_radix(&seed, 16).unwrap()
    } else {
        let seed: u64 = rand::thread_rng().gen();
        eprintln!("EUPHONY_SEED={:x}", seed);
        seed
    };
    static RNG: RefCell<StdRng> = RefCell::new(StdRng::seed_from_u64(seed()));
}

pub fn seed() -> u64 {
    SEED.with(|s| *s)
}

pub fn gen<T>() -> T
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    RNG.with(|r| r.borrow_mut().gen())
}

pub fn gen_range<T>(range: Range<T>) -> T
where
    T: rand::distributions::uniform::SampleUniform,
{
    RNG.with(|r| r.borrow_mut().gen_range(range.start, range.end))
}

pub fn shuffle<T>(items: &mut [T]) {
    RNG.with(|r| items.shuffle(&mut *r.borrow_mut()))
}

pub fn swap<T>(items: &mut [T]) {
    let a = gen_range(0..items.len());
    let b = gen_range(0..items.len());
    items.swap(a, b);
}

pub fn swap_count<T>(items: &mut [T], count: usize) {
    for _ in 0..count {
        swap(items);
    }
}

pub fn one_of<T: Clone>(items: &[T]) -> T {
    let index = gen_range(0..items.len());
    items[index].clone()
}
