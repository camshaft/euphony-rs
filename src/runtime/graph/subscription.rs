use crate::runtime::graph::{global, handle::NodeHandle, registry::NodeId};

pub trait Observable {
    type Subscription: Subscription;

    fn try_get(&self) -> Option<<Self::Subscription as Subscription>::Output>;
    fn get(&self) -> <Self::Subscription as Subscription>::Output {
        self.try_get().expect("Observable is closed")
    }
    fn observe(&self, child: &NodeHandle) -> Self::Subscription;
}

impl<'a, O: Observable> Observable for &'a O {
    type Subscription = O::Subscription;

    fn try_get(&self) -> Option<<Self::Subscription as Subscription>::Output> {
        (*self).try_get()
    }

    fn observe(&self, child: &NodeHandle) -> Self::Subscription {
        (*self).observe(child)
    }
}

impl<Sub: 'static + Subscription> Observable for Box<dyn Observable<Subscription = Sub>> {
    type Subscription = Box<dyn Subscription<Output = Sub::Output>>;

    fn try_get(&self) -> Option<Sub::Output> {
        self.as_ref().try_get()
    }

    fn observe(&self, handle: &NodeHandle) -> Self::Subscription {
        Box::new(self.as_ref().observe(handle))
    }
}

pub trait Subscription {
    type Output;

    fn try_get(&self) -> Option<Self::Output>;
    fn is_open(&self) -> bool;

    fn get(&self) -> Self::Output {
        self.try_get().expect("Subscription is closed")
    }
    fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

impl<Output> Subscription for Box<dyn Subscription<Output = Output>> {
    type Output = Output;

    fn try_get(&self) -> Option<Self::Output> {
        self.as_ref().try_get()
    }

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
