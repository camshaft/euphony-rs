use crate::units::time::Beat;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures::{ready, stream, FutureExt, Stream, StreamExt};

pub use crate::rand::{OneOfExt as RandOneOfExt, TaskExt as RandTaskExt};

pub trait DelayExt {
    fn delay(self) -> crate::time::Timer;
}

impl DelayExt for Beat {
    fn delay(self) -> crate::time::Timer {
        crate::time::delay(self)
    }
}

impl DelayExt for &Beat {
    fn delay(self) -> crate::time::Timer {
        crate::time::delay(*self)
    }
}

impl DelayExt for u64 {
    fn delay(self) -> crate::time::Timer {
        crate::time::delay(Beat(self, 1))
    }
}

impl DelayExt for &u64 {
    fn delay(self) -> crate::time::Timer {
        crate::time::delay(Beat(*self, 1))
    }
}

pub trait DelayStreamExt {
    type Iter;

    fn delays(self) -> DelayStream<Self::Iter>;
}

impl<T> DelayStreamExt for T
where
    T: IntoIterator,
    T::Item: DelayExt,
{
    type Iter = T::IntoIter;

    fn delays(self) -> DelayStream<Self::Iter> {
        DelayStream::new(self.into_iter())
    }
}

pub struct DelayStream<T> {
    items: T,
    timer: Option<crate::time::Timer>,
}

impl<T: Clone> Clone for DelayStream<T> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            timer: None,
        }
    }
}

impl<T> DelayStream<T> {
    pub const fn new(items: T) -> Self {
        Self { items, timer: None }
    }
}

type DelayStreamWith<Stream, IntoIter, Item> =
    stream::Map<stream::Zip<Stream, stream::Iter<IntoIter>>, fn(((), Item)) -> Item>;

impl<T> DelayStream<T>
where
    T: Iterator + Unpin,
    T::Item: DelayExt,
{
    pub fn with<V: IntoIterator>(self, values: V) -> DelayStreamWith<Self, V::IntoIter, V::Item> {
        self.zip(stream::iter(values.into_iter())).map(|(_, v)| v)
    }
}

impl<I> Stream for DelayStream<I>
where
    I: Iterator + Unpin,
    I::Item: DelayExt,
{
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(timer) = self.timer.as_mut() {
            ready!(timer.poll_unpin(cx));
        }

        let delay = if let Some(delay) = self.items.next() {
            delay
        } else {
            return None.into();
        };

        self.timer = Some(delay.delay());

        Poll::Ready(Some(()))
    }
}

pub trait SpawnExt {
    type Output;

    fn spawn(self) -> crate::runtime::JoinHandle<Self::Output>;
    fn spawn_primary(self) -> crate::runtime::JoinHandle<Self::Output>;
}

impl<F: 'static + core::future::Future + Send> SpawnExt for F
where
    F::Output: 'static + Send,
{
    type Output = F::Output;

    fn spawn(self) -> crate::runtime::JoinHandle<Self::Output> {
        crate::runtime::spawn(self)
    }

    fn spawn_primary(self) -> crate::runtime::JoinHandle<Self::Output> {
        crate::runtime::primary::spawn(self)
    }
}

pub trait EachExt {
    type Item;

    fn each<F: FnMut(&Self::Item) -> T, T>(&self, f: F) -> Vec<T>;
}

impl<U> EachExt for [U] {
    type Item = U;

    fn each<F: FnMut(&U) -> T, T>(&self, f: F) -> Vec<T> {
        self.iter().map(f).collect()
    }
}

impl EachExt for usize {
    type Item = usize;

    fn each<F: FnMut(&Self::Item) -> T, T>(&self, mut f: F) -> Vec<T> {
        (0..*self).map(|v| f(&v)).collect()
    }
}
