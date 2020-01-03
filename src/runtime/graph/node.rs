use crate::runtime::graph::{
    handle::NodeHandle,
    map::{MapCell, MappedCell},
    observer::Observer,
    subscription::{Observable, Readable, Subscription},
};
use core::{cmp::Ordering, fmt};

pub struct Node<T: Observable>(pub(crate) T);

impl<T: Observable> Node<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn map<Map, Output>(&self, map: Map) -> Node<MappedCell<T::Subscription, Map, Output>>
    where
        Map: FnMut(<T::Subscription as Readable>::Output) -> Output,
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

// impl<T: 'static + Observable> Node<T> {
//     pub fn boxed(self) -> BoxedNode<T::Output> {
//         Self(Box::new(self.0))
//     }
// }

impl<T> fmt::Debug for Node<T>
where
    T: Observable + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Observable> Readable for Node<T> {
    type Output = T::Output;

    fn try_read(&self) -> Option<Self::Output> {
        self.0.try_read()
    }
}

impl<T: Observable> Observable for Node<T> {
    type Subscription = T::Subscription;

    fn observe(&self, handle: &NodeHandle) -> Self::Subscription {
        self.0.observe(handle)
    }
}

pub type BoxedNode<Output> = Node<
    Box<dyn Observable<Output = Output, Subscription = Box<dyn Subscription<Output = Output>>>>,
>;

impl<A, B> PartialEq<Node<B>> for Node<A>
where
    A: Observable,
    B: Observable,
    A::Output: PartialEq<B::Output>,
{
    fn eq(&self, other: &Node<B>) -> bool {
        self.read().eq(&other.read())
    }
}

impl<A, B> PartialOrd<Node<B>> for Node<A>
where
    A: Observable,
    B: Observable,
    A::Output: PartialOrd<B::Output>,
{
    fn partial_cmp(&self, other: &Node<B>) -> Option<Ordering> {
        self.read().partial_cmp(&other.read())
    }
}

macro_rules! impl_cell_op {
    ($binary:ident, $binary_call:ident, $assign:ident, $assign_call:ident) => {
        impl<A, B> core::ops::$binary<Node<B>> for Node<A>
        where
            A: Observable,
            B: Observable,
            A::Output: core::ops::$binary<B::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        A::Output,
                        B::Output,
                    ) -> <A::Output as core::ops::$binary<B::Output>>::Output,
                    <A::Output as core::ops::$binary<B::Output>>::Output,
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
            A::Output: core::ops::$binary<B::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        A::Output,
                        B::Output,
                    ) -> <A::Output as core::ops::$binary<B::Output>>::Output,
                    <A::Output as core::ops::$binary<B::Output>>::Output,
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
            A::Output: core::ops::$binary<B::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        A::Output,
                        B::Output,
                    ) -> <A::Output as core::ops::$binary<B::Output>>::Output,
                    <A::Output as core::ops::$binary<B::Output>>::Output,
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
            A::Output: core::ops::$binary<B::Output>,
        {
            type Output = Node<
                MappedCell<
                    (A::Subscription, B::Subscription),
                    fn(
                        A::Output,
                        B::Output,
                    ) -> <A::Output as core::ops::$binary<B::Output>>::Output,
                    <A::Output as core::ops::$binary<B::Output>>::Output,
                >,
            >;

            fn $binary_call(self, rhs: &Node<B>) -> Self::Output {
                (self, rhs).map(core::ops::$binary::$binary_call)
            }
        }
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
