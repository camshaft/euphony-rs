use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use lazy_static::lazy_static;
use pin_project::pin_project;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, RngCore, SeedableRng};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

lazy_static! {
    static ref SEED: u64 = if let Ok(seed) = std::env::var("EUPHONY_SEED") {
        u64::from_str_radix(&seed, 16).unwrap()
    } else {
        let seed: u64 = rand::thread_rng().gen();
        eprintln!("EUPHONY_SEED={:x}", seed);
        seed
    };
}

thread_local! {
    static RNG: RefCell<Scope> = RefCell::new(Scope::new(seed()));
}

pub fn seed() -> u64 {
    *SEED
}

pub fn gen<T>() -> T
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    RNG.with(|r| r.borrow_mut().gen())
}

pub fn gen_range<B, T>(range: B) -> T
where
    B: rand::distributions::uniform::SampleRange<T>,
    T: rand::distributions::uniform::SampleUniform + PartialOrd,
{
    RNG.with(|r| r.borrow_mut().gen_range(range))
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

pub fn scope() -> Scope {
    RNG.with(|r| r.borrow().clone())
}

#[derive(Clone, Debug)]
pub struct Scope {
    rng: Arc<Mutex<StdRng>>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new(gen())
    }
}

impl Scope {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Arc::new(Mutex::new(StdRng::seed_from_u64(seed))),
        }
    }

    pub fn with<F: FnOnce() -> R, R>(&self, f: F) -> R {
        let prev = RNG.with(|r| core::mem::replace(&mut *r.borrow_mut(), self.clone()));
        let res = f();
        RNG.with(|r| *r.borrow_mut() = prev);
        res
    }
}

impl RngCore for Scope {
    fn next_u32(&mut self) -> u32 {
        self.rng.lock().unwrap().next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.lock().unwrap().next_u64()
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.rng.lock().unwrap().fill_bytes(bytes)
    }

    fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.lock().unwrap().try_fill_bytes(bytes)
    }
}

pub trait Task: Sized {
    fn rng(self) -> TaskScope<Self>;
    fn seed(self, seed: u64) -> TaskScope<Self>;
}

impl<F: Future> Task for F {
    fn rng(self) -> TaskScope<Self> {
        TaskScope::new(self)
    }

    fn seed(self, seed: u64) -> TaskScope<Self> {
        TaskScope {
            inner: self,
            scope: Scope::new(seed),
        }
    }
}

#[pin_project]
pub struct TaskScope<F> {
    #[pin]
    inner: F,
    scope: Scope,
}

impl<F> TaskScope<F> {
    pub fn new(inner: F) -> Self {
        Self {
            inner,
            scope: scope(),
        }
    }
}

impl<F: Future> Future for TaskScope<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let inner = this.inner;
        this.scope.with(move || Future::poll(inner, cx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_seed() {
        let _ = async {}.seed(123);
    }
}
