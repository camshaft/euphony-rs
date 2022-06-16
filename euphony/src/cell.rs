use core::{cell::UnsafeCell, fmt, ops};

#[derive(Default)]
pub struct Cell<T> {
    inner: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) -> T {
        let v = unsafe { &mut *self.inner.get() };
        core::mem::replace(v, value)
    }
}

impl<T> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

unsafe impl<T> Send for Cell<T> {}
unsafe impl<T> Sync for Cell<T> {}

impl<T: fmt::Debug> fmt::Debug for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T> AsRef<T> for Cell<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.inner.get() }
    }
}

impl<T> AsMut<T> for Cell<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}

impl<T> ops::Deref for Cell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> ops::DerefMut for Cell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
