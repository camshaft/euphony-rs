use crate::time::timestamp::Timestamp;
use alloc::rc::Rc;
use core::{cell::Cell, time::Duration};

/// A handle to a source of time.
#[derive(Debug, Clone)]
pub(crate) struct Clock {
    inner: Rc<Cell<Timestamp>>,
}

thread_local! {
    /// Thread-local tracking the current clock
    static CLOCK: Clock = Clock::new();
}

/// Advance time
///
/// Increments the saved `Timestamp` value by `duration`. Subsequent
/// calls to `Instant::now()` will return the result of the increment.
pub fn advance(duration: Duration) {
    CLOCK.with(|clock| {
        clock.advance(duration);
    });
}

pub fn set(now: Timestamp) {
    CLOCK.with(|clock| {
        clock.set(now);
    });
}

/// Return the current instant, factoring in frozen time.
fn now() -> Timestamp {
    CLOCK.with(|clock| clock.now())
}

impl Timestamp {
    pub fn now() -> Self {
        now()
    }
}

impl Clock {
    /// Return a new `Clock` instance that uses the current execution context's
    /// source of time.
    pub(crate) fn new() -> Clock {
        Clock {
            inner: Rc::new(Cell::new(Timestamp::default())),
        }
    }

    pub(crate) fn set(&self, now: Timestamp) {
        self.inner.set(now);
    }

    pub(crate) fn advance(&self, duration: Duration) {
        let next = self.inner.get() + duration;
        self.inner.set(next);
    }

    pub(crate) fn now(&self) -> Timestamp {
        self.inner.get()
    }
}
