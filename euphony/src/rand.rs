use crate::prelude::Beat;
use core::future::Future;
use once_cell::sync::Lazy;

pub use bach::rand::*;

pub fn rhythm(length: Beat, durations: impl IntoIterator<Item = Beat>) -> Vec<Beat> {
    let durations: Vec<_> = durations.into_iter().collect();
    let mut beats = vec![];
    let mut total = Beat(0, 1);
    while total < length {
        let b = *one_of(&durations);
        let new_total = total + b;
        if new_total > length {
            let b = length - total;
            beats.push(b);
            break;
        } else {
            total = new_total;
            beats.push(b);
        }
    }
    beats
}

pub fn with_seed<F: FnOnce() -> R, R>(seed: u64, f: F) -> R {
    scope::with(Scope::new(seed), f)
}

pub static SEED: Lazy<u64> = Lazy::new(|| {
    if let Ok(seed) = std::env::var("EUPHONY_SEED") {
        u64::from_str_radix(&seed, 16).unwrap()
    } else {
        0
    }
});

pub trait Ext: Sized {
    fn seed(self, seed: u64) -> Task<Self>;
    fn inhert_seed(self) -> Task<Self>;
}

impl<T> Ext for T
where
    T: Future,
{
    fn seed(self, seed: u64) -> Task<Self> {
        Task::new(self, Scope::new(seed))
    }

    fn inhert_seed(self) -> Task<Self> {
        Task::new(self, scope::borrow_with(|v| v.clone()))
    }
}
