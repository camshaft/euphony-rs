use core::future::Future;
use once_cell::sync::Lazy;

pub use bach::rand::*;

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
