use crate::runtime::graph::{global, handle::NodeHandle, registry::NodeId};

pub trait Readable {
    type Output;

    fn try_read(&self) -> Option<Self::Output>;
    fn read(&self) -> Self::Output {
        self.try_read().expect("Readable is closed")
    }
}

impl<Output> Readable for Box<dyn Readable<Output = Output>> {
    type Output = Output;

    fn try_read(&self) -> Option<Self::Output> {
        self.as_ref().try_read()
    }
}

impl<'a, R: Readable> Readable for &'a R {
    type Output = R::Output;

    fn try_read(&self) -> Option<Self::Output> {
        (*self).try_read()
    }
}

pub trait Observable: Readable {
    type Subscription: Subscription + Readable<Output = Self::Output>;

    fn observe(&self, child: &NodeHandle) -> Self::Subscription;
}

impl<'a, O: Observable> Observable for &'a O {
    type Subscription = O::Subscription;

    fn observe(&self, child: &NodeHandle) -> Self::Subscription {
        (*self).observe(child)
    }
}

impl<Sub, Output> Readable for Box<dyn Observable<Output = Output, Subscription = Sub>>
where
    Sub: 'static + Subscription<Output = Output>,
{
    type Output = Output;

    fn try_read(&self) -> Option<Self::Output> {
        self.as_ref().try_read()
    }
}

impl<Sub, Output> Observable for Box<dyn Observable<Output = Output, Subscription = Sub>>
where
    Sub: 'static + Subscription<Output = Output>,
{
    type Subscription = Box<dyn Subscription<Output = Output>>;

    fn observe(&self, handle: &NodeHandle) -> Self::Subscription {
        Box::new(self.as_ref().observe(handle))
    }
}

pub trait Subscription: Readable {
    fn is_open(&self) -> bool;

    fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

impl<Output> Readable for Box<dyn Subscription<Output = Output>> {
    type Output = Output;

    fn try_read(&self) -> Option<Self::Output> {
        self.as_ref().try_read()
    }
}

impl<Output> Subscription for Box<dyn Subscription<Output = Output>> {
    fn is_open(&self) -> bool {
        self.as_ref().is_open()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct SubscriptionHandle {
    parent: NodeId,
    child: NodeId,
}

impl SubscriptionHandle {
    pub fn new(parent: NodeId, child: NodeId) -> Self {
        debug_assert!(child != parent);
        global::with_mut(|registry| registry.subscribe(parent, child));
        Self { parent, child }
    }
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        global::with_mut(|registry| registry.unsubscribe(self.parent, self.child));
    }
}
