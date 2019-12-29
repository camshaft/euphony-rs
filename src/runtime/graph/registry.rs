use alloc::collections::BTreeMap;
use core::{fmt, task::Waker};
use generational_arena::Arena;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeId(generational_arena::Index);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (i, g) = self.0.into_raw_parts();
        write!(f, "NodeId({}.{})", g, i)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Status {
    Dirty,
    Clean,
}

impl Status {
    pub fn is_dirty(&self) -> bool {
        self == &Status::Dirty
    }
}

#[derive(Debug)]
pub struct Registry {
    nodes: Arena<Status>,
    first_child: BTreeMap<NodeId, SubscriptionId>,
    subscriptions: BTreeMap<SubscriptionId, Subscription>,
    wakers: BTreeMap<NodeId, Waker>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            first_child: Default::default(),
            subscriptions: Default::default(),
            nodes: Arena::new(),
            wakers: Default::default(),
        }
    }
}

macro_rules! debug {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        //eprintln!(concat!("[REGISTRY] ", $fmt), $($arg),*);
    };
}

impl Registry {
    pub fn insert_node(&mut self) -> NodeId {
        let id = NodeId(self.nodes.insert(Status::Clean));
        debug!("insert_node: {:?}", id);
        id
    }

    pub fn set_waker(&mut self, id: NodeId, waker: &Waker) {
        match self.nodes.get(id.0) {
            Some(Status::Clean) => {
                self.wakers.insert(id, waker.clone());
            }
            Some(Status::Dirty) => {
                waker.wake_by_ref();
            }
            _ => {}
        }
    }

    pub fn remove_node(&mut self, id: NodeId) {
        debug!("remove_node: {:?}", id);
        self.nodes.remove(id.0);

        let mut next_child = self.first_child.remove(&id);

        while let Some(next_id) = next_child.take() {
            debug!("remove_child: {:?}", id);
            let subscription = self
                .subscriptions
                .remove(&next_id)
                .expect("missing child subscription");
            next_child = subscription.next;
        }

        self.wakers.remove(&id);
    }

    pub fn mark_dirty(&mut self, id: NodeId) {
        debug!("mark_dirty: {:?}", id);

        fn traverse<'a>(
            nodes: &'a mut Arena<Status>,
            subscriptions: &'a BTreeMap<SubscriptionId, Subscription>,
            first_child: &'a BTreeMap<NodeId, SubscriptionId>,
            wakers: &'a mut BTreeMap<NodeId, Waker>,
            id: NodeId,
        ) {
            match nodes.get_mut(id.0) {
                Some(status @ Status::Clean) => {
                    *status = Status::Dirty;
                }
                _ => return,
            }
            if let Some(waker) = wakers.remove(&id) {
                waker.wake();
            }
            for child in children_iter(subscriptions, first_child.get(&id).cloned()) {
                traverse(nodes, subscriptions, first_child, wakers, child)
            }
        }

        traverse(
            &mut self.nodes,
            &self.subscriptions,
            &self.first_child,
            &mut self.wakers,
            id,
        );

        // TODO make this more efficient
        self.mark_clean(id);
    }

    pub fn mark_clean(&mut self, id: NodeId) {
        debug!("mark_clean: {:?}", id);

        self.set_status(id, Status::Clean);
    }

    pub fn set_status(&mut self, id: NodeId, status: Status) -> Status {
        if let Some(s) = self.nodes.get_mut(id.0) {
            core::mem::replace(s, status)
        } else {
            panic!("Node does not exist {:?}", id)
        }
    }

    pub fn status(&self, id: NodeId) -> Status {
        self.nodes.get(id.0).cloned().unwrap_or(Status::Clean)
    }

    pub fn subscribe(&mut self, parent: NodeId, child: NodeId) {
        let id = SubscriptionId { parent, child };
        debug!("subscribe: {:?}", id);

        let next = if let Some(child) = self.first_child.get(&parent) {
            self.subscriptions
                .get_mut(child)
                .expect("missing subscription")
                .prev = Some(id);
            Some(*child)
        } else {
            None
        };

        let subscription = Subscription {
            node: child,
            next,
            prev: None,
        };

        self.subscriptions.insert(id, subscription);
        self.first_child.insert(parent, id);
    }

    pub fn unsubscribe(&mut self, parent: NodeId, child: NodeId) {
        let id = SubscriptionId { parent, child };
        debug!("unsubscribe: {:?}", id);

        if let Some(Subscription { next, prev, .. }) = self.subscriptions.remove(&id) {
            if let Some(prev) = prev {
                // set the prev subscription to the next
                self.get_subscription_mut(&prev).next = next;

                if let Some(next) = next {
                    // set the next subscription to the prev
                    self.get_subscription_mut(&next).prev = Some(prev);
                }
            } else {
                if let Some(next) = next {
                    // move the first child to the next sub
                    self.get_subscription_mut(&next).prev = None;
                    self.first_child.insert(parent, next);
                } else {
                    // no subscriptions left
                    self.first_child.remove(&parent);
                }
            }
        }
    }

    fn get_subscription_mut(&mut self, id: &SubscriptionId) -> &mut Subscription {
        self.subscriptions
            .get_mut(&id)
            .unwrap_or_else(|| panic!("missing subscription {:?}", id))
    }
}

fn children_iter<'a>(
    subscriptions: &'a BTreeMap<SubscriptionId, Subscription>,
    next: Option<SubscriptionId>,
) -> ChildrenIterator<'a> {
    ChildrenIterator {
        subscriptions,
        next,
    }
}

struct ChildrenIterator<'a> {
    subscriptions: &'a BTreeMap<SubscriptionId, Subscription>,
    next: Option<SubscriptionId>,
}

impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let next_id = self.next.take()?;
        let subscription = self
            .subscriptions
            .get(&next_id)
            .expect("missing subscription");
        self.next = subscription.next;
        Some(subscription.node)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct SubscriptionId {
    parent: NodeId,
    child: NodeId,
}

#[derive(Debug)]
struct Subscription {
    node: NodeId,

    next: Option<SubscriptionId>,
    prev: Option<SubscriptionId>,
}

#[test]
fn single_dep_test() {
    let mut subscriptions = Registry::default();

    let a = subscriptions.insert_node();
    let b = subscriptions.insert_node();
    let c = subscriptions.insert_node();

    macro_rules! assert_status {
        ($a:ident, $b:ident, $c:ident) => {
            assert_eq!(
                (
                    subscriptions.status(a),
                    subscriptions.status(b),
                    subscriptions.status(c),
                ),
                (Status::$a, Status::$b, Status::$c)
            );
        };
    }

    subscriptions.subscribe(a, b);
    subscriptions.subscribe(b, c);

    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(c);
    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(b);
    assert_status!(Clean, Clean, Dirty);

    subscriptions.mark_clean(c);
    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(a);
    assert_status!(Clean, Dirty, Dirty);

    subscriptions.mark_clean(c);
    assert_status!(Clean, Dirty, Clean);

    subscriptions.mark_clean(b);
    assert_status!(Clean, Clean, Clean);
}

#[test]
fn multi_deps_test() {
    let mut subscriptions = Registry::default();

    let a = subscriptions.insert_node();
    let b = subscriptions.insert_node();
    let c = subscriptions.insert_node();

    macro_rules! assert_status {
        ($a:ident, $b:ident, $c:ident) => {
            assert_eq!(
                (
                    subscriptions.status(a),
                    subscriptions.status(b),
                    subscriptions.status(c),
                ),
                (Status::$a, Status::$b, Status::$c)
            );
        };
    }

    subscriptions.subscribe(a, c);
    subscriptions.subscribe(b, c);

    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(c);
    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(b);
    assert_status!(Clean, Clean, Dirty);

    subscriptions.mark_clean(c);
    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(a);
    assert_status!(Clean, Clean, Dirty);

    subscriptions.mark_clean(c);
    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(a);
    subscriptions.mark_dirty(b);
    assert_status!(Clean, Clean, Dirty);
}

#[test]
fn multi_child_test() {
    let mut subscriptions = Registry::default();

    let a = subscriptions.insert_node();
    let b = subscriptions.insert_node();
    let c = subscriptions.insert_node();

    macro_rules! assert_status {
        ($a:ident, $b:ident, $c:ident) => {
            assert_eq!(
                (
                    subscriptions.status(a),
                    subscriptions.status(b),
                    subscriptions.status(c),
                ),
                (Status::$a, Status::$b, Status::$c)
            );
        };
    }

    subscriptions.subscribe(a, c);
    subscriptions.subscribe(a, b);

    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(c);
    assert_status!(Clean, Clean, Clean);

    subscriptions.mark_dirty(a);
    assert_status!(Clean, Dirty, Dirty);
}
