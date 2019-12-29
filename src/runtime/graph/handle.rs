use crate::runtime::graph::{global, registry::NodeId, subscription::SubscriptionHandle};
use core::task::Waker;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct NodeHandle(NodeId);

impl NodeHandle {
    pub fn new() -> Self {
        Self(global::with_mut(|registry| registry.insert_node()))
    }

    pub fn subscribe_to(&self, parent: &Self) -> SubscriptionHandle {
        debug_assert!(self != parent);
        SubscriptionHandle::new(parent.0, self.0)
    }

    pub fn mark_dirty(&self) {
        global::with_mut(|registry| registry.mark_dirty(self.0));
    }

    pub fn mark_clean(&self) {
        global::with_mut(|registry| registry.mark_clean(self.0));
    }

    pub fn is_dirty(&self) -> bool {
        global::with(|registry| registry.status(self.0).is_dirty())
    }

    pub fn set_waker(&self, waker: &Waker) {
        global::with_mut(|registry| registry.set_waker(self.0, waker));
    }
}

impl Drop for NodeHandle {
    fn drop(&mut self) {
        global::with_mut(|registry| registry.remove_node(self.0));
    }
}
