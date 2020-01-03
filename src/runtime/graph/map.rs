#![allow(non_snake_case)]

use crate::runtime::graph::{
    handle::NodeHandle,
    node::Node,
    subscription::{Observable, Readable, Subscription, SubscriptionHandle},
};
use alloc::rc::Rc;
use core::{cell::UnsafeCell, marker::PhantomData};

pub struct MappedCell<S, Map, Output>(Rc<InnerComputation<S, Map, Output>>);

pub struct MappedCellSubscription<S, Map, Output> {
    #[allow(dead_code)]
    subscription: SubscriptionHandle,
    parent: MappedCell<S, Map, Output>,
}

impl<S, Map, Output> Subscription for MappedCellSubscription<S, Map, Output>
where
    S: MappedSubscription<Map, Output>,
{
    fn is_open(&self) -> bool {
        self.parent.is_open()
    }
}

impl<S, Map, Output> Readable for MappedCellSubscription<S, Map, Output>
where
    S: MappedSubscription<Map, Output>,
{
    type Output = Output;

    fn try_read(&self) -> Option<Output> {
        self.parent.0.try_read()
    }
}

impl<S, Map, Output> Observable for MappedCell<S, Map, Output>
where
    S: MappedSubscription<Map, Output>,
{
    type Subscription = MappedCellSubscription<S, Map, Output>;

    fn observe(&self, handle: &NodeHandle) -> Self::Subscription {
        MappedCellSubscription {
            subscription: handle.subscribe_to(&self.0.handle),
            parent: self.clone(),
        }
    }
}

impl<S, Map, Output> Readable for MappedCell<S, Map, Output>
where
    S: MappedSubscription<Map, Output>,
{
    type Output = Output;

    fn try_read(&self) -> Option<Output> {
        self.0.try_read()
    }
}

impl<S, Map, Output> Subscription for MappedCell<S, Map, Output>
where
    S: MappedSubscription<Map, Output>,
{
    fn is_open(&self) -> bool {
        MappedSubscription::is_open(&self.0.subscription)
    }
}

impl<S, Map, Output> MappedCell<S, Map, Output> {
    fn new(handle: NodeHandle, subscription: S, map: Map) -> Self {
        MappedCell(Rc::new(InnerComputation::new(handle, subscription, map)))
    }
}

impl<S, Map, Output> Clone for MappedCell<S, Map, Output> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// impl<Dependency, Map, Output> fmt::Debug for MappedCell<Dependency, Map, Output>
// where
//     Dependency: MapCell<Map, Output>,
//     Output: fmt::Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_tuple("MappedCell").field(self.0.get()).finish()
//     }
// }

struct InnerComputation<S, Map, Output> {
    handle: NodeHandle,
    subscription: S,
    map: UnsafeCell<Map>,
    output: PhantomData<Output>,
}

impl<S, Map, Output> InnerComputation<S, Map, Output> {
    fn new(handle: NodeHandle, subscription: S, map: Map) -> Self {
        InnerComputation {
            handle,
            subscription,
            map: UnsafeCell::new(map),
            output: PhantomData,
        }
    }

    fn map_mut(&self) -> &mut Map {
        unsafe { &mut *self.map.get() }
    }
}

impl<S, Map, Output> InnerComputation<S, Map, Output>
where
    S: MappedSubscription<Map, Output>,
{
    fn try_read(&self) -> Option<Output> {
        self.handle.mark_clean();
        MappedSubscription::try_read(&self.subscription, self.map_mut())
    }
}

pub trait MapCell<Map, Output>: Sized {
    type Subscription: MappedSubscription<Map, Output>;

    fn map(&self, map: Map) -> Node<MappedCell<Self::Subscription, Map, Output>>;
}

pub trait MappedSubscription<Map, Output> {
    fn try_read(subscription: &Self, map: &mut Map) -> Option<Output>;
    fn is_open(subscription: &Self) -> bool;
}

impl<O, Map, Output> MapCell<Map, Output> for O
where
    O: Observable,
    Map: FnMut(O::Output) -> Output,
{
    type Subscription = O::Subscription;

    fn map(&self, map: Map) -> Node<MappedCell<Self::Subscription, Map, Output>> {
        let handle = NodeHandle::new();
        let subscription = self.observe(&handle);
        let cell = MappedCell::new(handle, subscription, map);
        Node::new(cell)
    }
}

impl<S, Map, Output> MappedSubscription<Map, Output> for S
where
    S: Subscription,
    Map: FnMut(S::Output) -> Output,
{
    fn try_read(subscription: &Self, map: &mut Map) -> Option<Output> {
        Some(map(subscription.try_read()?))
    }

    fn is_open(subscription: &Self) -> bool {
        Subscription::is_open(subscription)
    }
}

macro_rules! impl_map_tuple {
    ([$($c:ident),*]) => {
        impl_map_tuple!([$($c,)*], []);
    };
    ([], [$($p:ident,)*]) => {
        // noop
    };
    ([$a:ident, $($rest:ident,)*], [$($p:ident,)*]) => {
        impl<$($p,)* $a, Map, Output> MapCell<Map, Output> for ($($p,)* $a,)
        where
            Map: FnMut(
                $(
                    $p::Output,
                )*
                $a::Output
            ) -> Output,
            $(
                $p: Observable,
            )*
            $a: Observable
        {
            type Subscription = (
                $(
                    $p::Subscription,
                )*
                $a::Subscription,
            );

            fn map(&self, map: Map) -> Node<MappedCell<Self::Subscription, Map, Output>> {
                let handle = NodeHandle::new();
                let ($($p,)* $a,) = self;
                let subscription = (
                    $(
                        $p.observe(&handle),
                    )*
                    $a.observe(&handle),
                );
                let cell = MappedCell::new(handle, subscription, map);
                Node::new(cell)
            }
        }

        impl<$($p,)* $a, Map, Output> MappedSubscription<Map, Output> for ($($p,)* $a,)
        where
            Map: FnMut($($p::Output,)* $a::Output) -> Output,
            $(
                $p: Subscription,
            )*
            $a: Subscription
        {
            fn try_read(subscription: &Self, map: &mut Map) -> Option<Output> {
                let ($($p,)* $a,) = subscription;
                Some(map(
                    $(
                        Readable::try_read($p)?,
                    )*
                    Readable::try_read($a)?
                ))
            }

            fn is_open(subscription: &Self) -> bool {
                let ($($p,)* $a,) = subscription;
                $(
                    Subscription::is_open($p) &&
                )*
                Subscription::is_open($a)
            }
        }

        impl_map_tuple!([$($rest,)*], [$($p,)* $a,]);
    }
}

impl_map_tuple!([A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z]);

#[test]
fn map_test() {
    let a = super::cell::cell(1);
    let b = a.map(|v| v + 1);
    let c = a.map(|v| v * 3);
    let d = (&b, &c).map(|v1, v2| v1 + v2);

    assert_eq!(d.read(), 5);

    a.set(2);
    assert_eq!(d.read(), 9);

    a.close();
}

#[test]
fn ops_test() {
    use super::cell::cell;

    let a = cell(1);
    let add = cell(1);
    let b = &a + &add;
    let mul = cell(3);
    let c = &a * &mul;
    let d = b + c;

    assert_eq!(d.read(), 5);

    a.set(2);
    assert_eq!(d.read(), 9);

    a.close();
}
