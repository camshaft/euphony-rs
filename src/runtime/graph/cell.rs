use crate::runtime::graph::{
    handle::NodeHandle,
    node::Node,
    subscription::{Observable, Subscription, SubscriptionHandle},
};
use alloc::rc::{Rc, Weak};
use core::{cell::UnsafeCell, fmt};

pub fn cell<Value: Clone>(value: Value) -> Node<Cell<Value>> {
    Node::new(Cell::new(value))
}

impl<Value: Clone> Node<Cell<Value>> {
    pub fn set(&self, value: Value) {
        self.0.set(value)
    }

    pub fn update<F: FnOnce(&mut Value)>(&self, update: F) {
        self.0.update(update)
    }

    pub fn close(self) {
        self.0.close()
    }
}

pub struct Cell<Value>(Rc<InnerCell<Value>>);

impl<Value> Cell<Value> {
    pub fn new(value: Value) -> Self {
        Self(Rc::new(InnerCell::new(value)))
    }

    pub fn set(&self, value: Value) {
        self.0.set(value)
    }

    pub fn update<F: FnOnce(&mut Value)>(&self, update: F) {
        self.0.update(update)
    }

    pub fn close(self) {
        // noop
    }
}

impl<Value> Clone for Cell<Value> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Value: Clone> Observable for Cell<Value> {
    type Subscription = CellSubscription<Value>;

    fn try_get(&self) -> Option<Value> {
        Some(self.0.value_mut().clone())
    }

    fn observe(&self, child: &NodeHandle) -> Self::Subscription {
        CellSubscription {
            subscription: child.subscribe_to(&self.0.handle),
            cell: Rc::downgrade(&self.0),
        }
    }
}

impl<Value> fmt::Debug for Cell<Value>
where
    Value: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Cell").field(self.0.value_mut()).finish()
    }
}

#[derive(Debug)]
pub struct InnerCell<Value> {
    handle: NodeHandle,
    value: UnsafeCell<Value>,
}

impl<Value> InnerCell<Value> {
    fn new(value: Value) -> Self {
        Self {
            handle: NodeHandle::new(),
            value: UnsafeCell::new(value),
        }
    }

    fn set(&self, value: Value) {
        *self.value_mut() = value;

        // TODO compare values
        self.handle.mark_dirty();
    }

    fn update<F: FnOnce(&mut Value)>(&self, update: F) {
        update(self.value_mut());

        // TODO compare values
        self.handle.mark_dirty();
    }

    fn value_mut(&self) -> &mut Value {
        unsafe { &mut *self.value.get() }
    }
}

pub struct CellSubscription<Value> {
    cell: Weak<InnerCell<Value>>,

    // The subscription will unsubscribe when dropped
    #[allow(dead_code)]
    subscription: SubscriptionHandle,
}

impl<Value: Clone> Subscription for CellSubscription<Value> {
    type Output = Value;

    fn try_get(&self) -> Option<Self::Output> {
        Some(self.cell.upgrade()?.value_mut().clone())
    }

    fn is_open(&self) -> bool {
        self.cell.upgrade().is_some()
    }
}

#[test]
fn cell_test() {
    let a = cell(1usize);
    let b = cell(2usize);
    let c = &a * &b;

    assert_eq!(a.get(), 1);
    assert_eq!(b.get(), 2);
    assert_eq!(c.get(), 2);

    a.set(2);
    assert_eq!(a.get(), 2);
    assert_eq!(b.get(), 2);
    assert_eq!(c.get(), 4);
}

#[test]
fn cell_dyn_test() {
    let a = cell(1usize);
    let b = a.clone().boxed().map(|a| a + 2).boxed();

    assert_eq!(a.get(), 1);
    assert_eq!(b.get(), 3);
}
