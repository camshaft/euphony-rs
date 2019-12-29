use crate::runtime::graph::{
    handle::NodeHandle,
    subscription::{Observable, Subscription},
};
use core::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use futures_core::Stream;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum ObserverState {
    Initial,
    Pending,
    Ready,
}

pub struct Observer<S: Subscription> {
    subscription: S,
    handle: NodeHandle,
    state: ObserverState,
}

impl<S: Subscription> Observer<S> {
    pub fn new<O>(dependency: &O) -> Self
    where
        O: Observable<Subscription = S>,
    {
        let handle = NodeHandle::new();
        let subscription = dependency.observe(&handle);
        Self {
            handle,
            subscription,
            state: ObserverState::Initial,
        }
    }

    fn set_waker(&self, waker: &Waker) {
        self.handle.set_waker(waker)
    }
}

impl<S: Subscription> Subscription for Observer<S> {
    type Output = S::Output;

    fn try_get(&self) -> Option<Self::Output> {
        self.handle.mark_clean();
        self.subscription.try_get()
    }

    fn is_open(&self) -> bool {
        self.subscription.is_open()
    }
}

impl<S> fmt::Debug for Observer<S>
where
    S: Subscription,
    S::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Observer").field(&self.get()).finish()
    }
}

impl<S> Future for Observer<S>
where
    S: Subscription + Unpin,
{
    type Output = Option<S::Output>;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        let observer = self.get_mut();
        match observer.state {
            ObserverState::Initial | ObserverState::Pending => {
                observer.state = ObserverState::Ready;
                observer.set_waker(context.waker());
                Poll::Pending
            }
            ObserverState::Ready => {
                observer.state = ObserverState::Pending;
                Poll::Ready(observer.try_get())
            }
        }
    }
}

impl<S> Stream for Observer<S>
where
    S: Subscription + Unpin,
{
    type Item = S::Output;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        let observer = self.get_mut();
        match observer.state {
            ObserverState::Pending => {
                observer.state = ObserverState::Ready;
                observer.set_waker(context.waker());
                Poll::Pending
            }
            ObserverState::Initial | ObserverState::Ready => {
                observer.state = ObserverState::Pending;
                Poll::Ready(observer.try_get())
            }
        }
    }
}