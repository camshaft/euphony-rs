use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use lazy_static::lazy_static;
use pin_project::pin_project;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, RngCore, SeedableRng};

lazy_static! {
    pub static ref EUPHONY_SEED: u64 = if let Ok(seed) = std::env::var("EUPHONY_SEED") {
        u64::from_str_radix(&seed, 16).unwrap()
    } else {
        let seed: u64 = rand::thread_rng().gen();
        seed
    };
}

pub mod scope {
    crate::scope!(rand, super::Scope);
}

pub fn seed() -> u64 {
    scope::borrow(|r| r.seed)
}

pub fn gen<T>() -> T
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    scope::borrow_mut(|r| r.gen())
}

pub fn gen_range<B, T>(range: B) -> T
where
    B: rand::distributions::uniform::SampleRange<T>,
    T: rand::distributions::uniform::SampleUniform + PartialOrd,
{
    scope::borrow_mut(|r| r.gen_range(range))
}

pub fn shuffle<T>(items: &mut [T]) {
    scope::borrow_mut(|r| items.shuffle(r))
}

pub fn swap<T>(items: &mut [T]) {
    swap_count(items, 1)
}

pub fn swap_count<T>(items: &mut [T], count: usize) {
    scope::borrow_mut(|r| {
        for _ in 0..count {
            let a = r.gen_range(0..items.len());
            let b = r.gen_range(0..items.len());
            items.swap(a, b)
        }
    })
}

pub fn one_of<T>(items: &[T]) -> &T {
    let index = gen_range(0..items.len());
    &items[index]
}

#[derive(Debug)]
pub struct Scope {
    seed: u64,
    rng: StdRng,
}

impl Scope {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn with<F: FnOnce() -> R, R>(&mut self, f: F) -> R {
        let has_prev = scope::try_borrow_mut(|prev| {
            if let Some(prev) = prev.as_mut() {
                core::mem::swap(prev, self);
                true
            } else {
                *prev = Some(Self {
                    seed: self.seed,
                    rng: self.rng.clone(),
                });
                false
            }
        });

        let res = f();

        scope::try_borrow_mut(|current| {
            if has_prev {
                core::mem::swap(current.as_mut().unwrap(), self);
            } else {
                *self = current.take().unwrap();
            }
        });

        res
    }
}

impl RngCore for Scope {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.rng.fill_bytes(bytes)
    }

    fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(bytes)
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
            scope: Scope::new(gen()),
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
