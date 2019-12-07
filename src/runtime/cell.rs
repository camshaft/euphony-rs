use alloc::rc::Rc;
use core::{
    cell::UnsafeCell,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

#[derive(Default)]
pub struct Cell<T>(Rc<UnsafeCell<T>>);

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(UnsafeCell::new(value)))
    }

    pub fn set(&self, value: T) {
        *self.get_mut() = value;
    }

    pub fn update<F: Fn(&mut T)>(&self, update: F) {
        update(self.get_mut())
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.0.as_ref().get() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.0.as_ref().get() }
    }
}

impl<T> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: fmt::Debug> fmt::Debug for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &T = &*self;
        value.fmt(f)
    }
}

impl<T> Deref for Cell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for Cell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: AsRef<Target>, Target> AsRef<Target> for Cell<T> {
    fn as_ref(&self) -> &Target {
        self.deref().as_ref()
    }
}

impl<T: AsMut<Target>, Target> AsMut<Target> for Cell<T> {
    fn as_mut(&mut self) -> &mut Target {
        self.deref_mut().as_mut()
    }
}

impl<T> PartialEq<Self> for Cell<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T> PartialOrd<Self> for Cell<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<T> Eq for Cell<T> where Cell<T>: Ord {}

impl<T> Ord for Cell<T>
where
    Cell<T>: PartialOrd,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<T: Hash> Hash for Cell<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.deref().hash(hasher)
    }
}

macro_rules! impl_op {
    ($binary:ident, $binary_call:ident, $unary:ident, $unary_call:ident) => {
        impl<T: Copy, Other> core::ops::$binary<Other> for Cell<T>
        where
            T: core::ops::$binary<Other>,
        {
            type Output = <T as core::ops::$binary<Other>>::Output;

            fn $binary_call(self, other: Other) -> Self::Output {
                core::ops::$binary::$binary_call(*self.deref(), other)
            }
        }

        impl<T, Other> core::ops::$unary<Other> for Cell<T>
        where
            T: core::ops::$unary<Other>,
        {
            fn $unary_call(&mut self, other: Other) {
                core::ops::$unary::$unary_call(self.deref_mut(), other)
            }
        }
    };
}

impl_op!(Add, add, AddAssign, add_assign);
impl_op!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_op!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_op!(BitXor, bitxor, BitXorAssign, bitxor_assign);
impl_op!(Div, div, DivAssign, div_assign);
impl_op!(Mul, mul, MulAssign, mul_assign);
impl_op!(Rem, rem, RemAssign, rem_assign);
impl_op!(Shl, shl, ShlAssign, shl_assign);
impl_op!(Shr, shr, ShrAssign, shr_assign);
impl_op!(Sub, sub, SubAssign, sub_assign);

#[test]
fn cell_test() {
    use crate::time::tempo::Tempo;
    let mut tempo1: Cell<Tempo> = Default::default();
    let mut tempo2 = tempo1.clone();

    dbg!(&tempo1);
    dbg!(&tempo2);

    tempo1 += 1;
    dbg!(&tempo1);
    dbg!(&tempo2);

    tempo2 += 2;
    dbg!(&tempo1);
    dbg!(&tempo2);
}
