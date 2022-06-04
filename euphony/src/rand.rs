use crate::prelude::Beat;
use ::rand::distributions;
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

pub trait OneOfExt<'a> {
    type Output;

    fn one_of(&'a self) -> Self::Output;

    fn pick(&'a self) -> Self::Output {
        self.one_of()
    }
}

impl<'a, T: 'a> OneOfExt<'a> for [T] {
    type Output = &'a T;

    fn one_of(&'a self) -> Self::Output {
        one_of(self)
    }
}

impl<'a> OneOfExt<'a> for usize {
    type Output = usize;

    fn one_of(&'a self) -> Self::Output {
        (0..*self).one_of()
    }
}

impl<'a> OneOfExt<'a> for u64 {
    type Output = u64;

    fn one_of(&'a self) -> Self::Output {
        (0..*self).one_of()
    }
}

macro_rules! one_of_range {
    ($range:ident) => {
        impl<'a, T> OneOfExt<'a> for core::ops::$range<T>
        where
            core::ops::$range<T>: distributions::uniform::SampleRange<T>,
            T: Copy + distributions::uniform::SampleUniform + PartialOrd,
        {
            type Output = T;

            fn one_of(&'a self) -> Self::Output {
                gen_range(self.clone())
            }
        }
    };
}

one_of_range!(Range);
one_of_range!(RangeTo);
one_of_range!(RangeInclusive);
one_of_range!(RangeToInclusive);

pub trait TaskExt: Sized {
    fn seed(self, seed: u64) -> Task<Self>;
    fn inhert_seed(self) -> Task<Self>;
}

impl<T> TaskExt for T
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
