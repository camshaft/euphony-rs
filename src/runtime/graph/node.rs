use crate::runtime::graph::{
    handle::NodeHandle,
    map::{MapCell, MappedCell},
    observer::Observer,
    subscription::{Observable, Subscription},
};
use core::{cmp::Ordering, fmt};

pub struct Node<T: Observable>(pub(crate) T);

impl<T: Observable> Node<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn map<Map, Output>(&self, map: Map) -> Node<MappedCell<T::Subscription, Map, Output>>
    where
        Map: FnMut(<T::Subscription as Subscription>::Output) -> Output,
        Self: MapCell<Map, Output>,
    {
        <T as MapCell<_, _>>::map(&self.0, map)
    }

    pub fn observe(&self) -> Observer<T::Subscription> {
        Observer::new(&self.0)
    }
}

impl<T> Clone for Node<T>
where
    T: Clone + Observable,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static + Observable> Node<T> {
    pub fn boxed(self) -> BoxedNode<T::Subscription> {
        Self(Box::new(self.0))
    }
}

impl<T> fmt::Debug for Node<T>
where
    T: Observable + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Observable> Observable for Node<T> {
    type Subscription = T::Subscription;

    fn try_get(&self) -> Option<<Self::Subscription as Subscription>::Output> {
        self.0.try_get()
    }

    fn observe(&self, handle: &NodeHandle) -> Self::Subscription {
        self.0.observe(handle)
    }
}

pub type BoxedNode<Subscription> = Node<Box<dyn Observable<Subscription = Subscription>>>;

impl<A, B> PartialEq<Node<B>> for Node<A>
where
    A: Observable,
    B: Observable,
    <A::Subscription as Subscription>::Output: PartialEq<<B::Subscription as Subscription>::Output>,
{
    fn eq(&self, other: &Node<B>) -> bool {
        self.get().eq(&other.get())
    }
}

impl<A, B> PartialOrd<Node<B>> for Node<A>
where
    A: Observable,
    B: Observable,
    <A::Subscription as Subscription>::Output:
        PartialOrd<<B::Subscription as Subscription>::Output>,
{
    fn partial_cmp(&self, other: &Node<B>) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

macro_rules! impl_cell_op {
    ($binary:ident, $binary_call:ident, $assign:ident, $assign_call:ident) => {
        impl<A, B> core::ops::$binary<Node<B>> for Node<A>
        where
            A: Observable,
            B: Observable,
            <A::Subscription as Subscription>::Output:
                core::ops::$binary<<B::Subscription as Subscription>::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        <A::Subscription as Subscription>::Output,
                        <B::Subscription as Subscription>::Output,
                    )
                        -> <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                    <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                >,
            >;

            fn $binary_call(self, rhs: Node<B>) -> Self::Output {
                (self, rhs).map(core::ops::$binary::$binary_call)
            }
        }

        impl<A, B> core::ops::$binary<Node<B>> for &Node<A>
        where
            A: Observable,
            B: Observable,
            <A::Subscription as Subscription>::Output:
                core::ops::$binary<<B::Subscription as Subscription>::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        <A::Subscription as Subscription>::Output,
                        <B::Subscription as Subscription>::Output,
                    )
                        -> <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                    <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                >,
            >;

            fn $binary_call(self, rhs: Node<B>) -> Self::Output {
                (self, rhs).map(core::ops::$binary::$binary_call)
            }
        }

        impl<A, B> core::ops::$binary<&Node<B>> for Node<A>
        where
            A: Observable,
            B: Observable,
            <A::Subscription as Subscription>::Output:
                core::ops::$binary<<B::Subscription as Subscription>::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        <A::Subscription as Subscription>::Output,
                        <B::Subscription as Subscription>::Output,
                    )
                        -> <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                    <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                >,
            >;

            fn $binary_call(self, rhs: &Node<B>) -> Self::Output {
                (self, rhs).map(core::ops::$binary::$binary_call)
            }
        }

        impl<A, B> core::ops::$binary<&Node<B>> for &Node<A>
        where
            A: Observable,
            B: Observable,
            <A::Subscription as Subscription>::Output:
                core::ops::$binary<<B::Subscription as Subscription>::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        <A::Subscription as Subscription>::Output,
                        <B::Subscription as Subscription>::Output,
                    )
                        -> <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                    <<A::Subscription as Subscription>::Output as core::ops::$binary<
                        <B::Subscription as Subscription>::Output,
                    >>::Output,
                >,
            >;

            fn $binary_call(self, rhs: &Node<B>) -> Self::Output {
                (self, rhs).map(core::ops::$binary::$binary_call)
            }
        }

        //         impl<A, B> core::ops::$binary<&Node<B>> for &Node<A>
        //         where
        //             A: Clone + NodeOp,
        //             B: Clone + NodeOp,
        //             Node<A>: core::ops::$binary<Node<B>>,
        //         {
        //             type Output = <Node<A> as core::ops::$binary<Node<B>>>::Output;

        //             fn $binary_call(self, rhs: &Node<B>) -> Self::Output {
        //                 self.clone().$binary_call(rhs.clone())
        //             }
        //         }

        //         impl<A, B> core::ops::$binary<Node<B>> for &Node<A>
        //         where
        //             A: Clone + NodeOp,
        //             B: Clone + NodeOp,
        //             Node<A>: core::ops::$binary<Node<B>>,
        //         {
        //             type Output = <Node<A> as core::ops::$binary<Node<B>>>::Output;

        //             fn $binary_call(self, rhs: Node<B>) -> Self::Output {
        //                 self.clone().$binary_call(rhs)
        //             }
        //         }

        //         impl<A, B> core::ops::$binary<&Node<B>> for Node<A>
        //         where
        //             A: Clone + NodeOp,
        //             B: Clone + NodeOp,
        //             Node<A>: core::ops::$binary<Node<B>>,
        //         {
        //             type Output = <Node<A> as core::ops::$binary<Node<B>>>::Output;

        //             fn $binary_call(self, rhs: &Node<B>) -> Self::Output {
        //                 self.$binary_call(rhs.clone())
        //             }
        //         }

        //         impl<T, RHS> core::ops::$assign<RHS> for super::cell::Cell<T>
        //         where
        //             T: Clone + core::ops::$assign<RHS>,
        //         {
        //             fn $assign_call(&mut self, other: RHS) {
        //                 self.update(move |value| {
        //                     value.$assign_call(other);
        //                 });
        //             }
        //         }

        //         impl<T, RHS> core::ops::$assign<RHS> for Node<super::cell::Cell<T>>
        //         where
        //             T: Clone + core::ops::$assign<RHS>,
        //         {
        //             fn $assign_call(&mut self, other: RHS) {
        //                 core::ops::$assign::$assign_call(&mut self.0, other)
        //             }
        //         }
    };
}

impl_cell_op!(Add, add, AddAssign, add_assign);
impl_cell_op!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_cell_op!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_cell_op!(BitXor, bitxor, BitXorAssign, bitxor_assign);
impl_cell_op!(Div, div, DivAssign, div_assign);
impl_cell_op!(Mul, mul, MulAssign, mul_assign);
impl_cell_op!(Rem, rem, RemAssign, rem_assign);
impl_cell_op!(Shl, shl, ShlAssign, shl_assign);
impl_cell_op!(Shr, shr, ShrAssign, shr_assign);
impl_cell_op!(Sub, sub, SubAssign, sub_assign);
