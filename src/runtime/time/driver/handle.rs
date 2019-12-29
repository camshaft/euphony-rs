use crate::runtime::time::driver::Inner;
use alloc::rc::{Rc, Weak};
use core::{cell::RefCell, fmt};

/// Handle to time driver instance.
#[derive(Clone)]
pub(crate) struct Handle {
    driver: Weak<Inner>,
}

thread_local! {
    /// Tracks the timer for the current execution context.
    static CURRENT_TIMER: RefCell<Option<Handle>> = RefCell::new(None)
}

#[derive(Debug)]
/// Guard that unsets the current default timer on drop.
pub(crate) struct DefaultGuard {
    prev: Option<Handle>,
}

impl Drop for DefaultGuard {
    fn drop(&mut self) {
        CURRENT_TIMER.with(|current| {
            let mut current = current.borrow_mut();
            *current = self.prev.take();
        })
    }
}

/// Sets handle to default timer, returning guard that unsets it on drop.
///
/// # Panics
///
/// This function panics if there already is a default timer set.
pub(crate) fn set_default(handle: Handle) -> DefaultGuard {
    CURRENT_TIMER.with(|current| {
        let mut current = current.borrow_mut();
        let prev = current.take();

        *current = Some(handle);

        DefaultGuard { prev }
    })
}

impl Handle {
    /// Create a new timer `Handle` from a shared `Inner` timer state.
    pub(crate) fn new(driver: Weak<Inner>) -> Self {
        Handle { driver }
    }

    /// Try to get a handle to the current timer.
    ///
    /// # Panics
    ///
    /// This function panics if there is no current timer set.
    pub(crate) fn current() -> Self {
        CURRENT_TIMER.with(|current| match *current.borrow() {
            Some(ref handle) => handle.clone(),
            None => panic!("no current timer"),
        })
    }

    /// Try to return a strong ref to the driver
    pub(crate) fn driver(&self) -> Option<Rc<Inner>> {
        self.driver.upgrade()
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle")
    }
}
