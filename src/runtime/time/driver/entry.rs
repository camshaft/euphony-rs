use crate::{
    runtime::time::driver::{Handle, Inner},
    time::timestamp::Timestamp,
};
use alloc::rc::{Rc, Weak};
use core::{
    cell::UnsafeCell,
    fmt, ptr,
    task::{self, Poll, Waker},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum EntryState {
    New,
    Pending,
    Elapsed,
    Error,
}

impl EntryState {
    fn is_elapsed(&self) -> bool {
        self == &Self::Elapsed
    }

    fn pending(&mut self) -> bool {
        match self {
            Self::New => {
                *self = Self::Pending;
                true
            }
            _ => false,
        }
    }

    fn fire(&mut self) -> bool {
        match self {
            Self::Pending => {
                *self = Self::Elapsed;
                true
            }
            _ => false,
        }
    }

    fn error(&mut self) -> bool {
        match self {
            Self::Pending => {
                *self = Self::Error;
                true
            }
            _ => false,
        }
    }

    fn poll(&self) -> Poll<Result<(), ()>> {
        match self {
            Self::New => Poll::Pending,
            Self::Pending => Poll::Pending,
            Self::Elapsed => Poll::Ready(Ok(())),
            Self::Error => Poll::Ready(Err(())),
        }
    }
}

/// Internal state shared between a `Delay` instance and the timer.
///
/// This struct is used as a node in two intrusive data structures:
///
/// * An atomic stack used to signal to the timer thread that the entry state
///   has changed. The timer thread will observe the entry on this stack and
///   perform any actions as necessary.
///
/// * A doubly linked list used **only** by the timer thread. Each slot in the
///   timer wheel is a head pointer to the list of entries that must be
///   processed during that timer tick.
pub(crate) struct Entry {
    /// Only accessed from `Registration`.
    deadline: UnsafeCell<Timestamp>,

    /// Timer internals. Using a weak pointer allows the timer to shutdown
    /// without all `Delay` instances having completed.
    ///
    /// When `None`, the entry has not yet been linked with a timer instance.
    driver: Weak<Inner>,

    pub(super) state: UnsafeCell<EntryState>,

    /// Task to notify once the deadline is reached.
    waker: UnsafeCell<Option<Waker>>,

    /// True when the entry is queued in the "process" stack. This value
    /// is set before pushing the value and unset after popping the value.
    ///
    /// TODO: This could possibly be rolled up into `state`.
    pub(super) queued: UnsafeCell<bool>,

    /// Next entry in the "process" linked list.
    ///
    /// Access to this field is coordinated by the `queued` flag.
    ///
    /// Represents a strong Rc ref.
    pub(super) notification_next: UnsafeCell<*mut Entry>,

    /// Next entry in the State's linked list.
    ///
    /// This is only accessed by the timer
    pub(super) wheel_next: UnsafeCell<Option<Rc<Entry>>>,

    /// Previous entry in the State's linked list.
    ///
    /// This is only accessed by the timer and is used to unlink a canceled
    /// entry.
    ///
    /// This is a weak reference.
    pub(super) wheel_prev: UnsafeCell<*const Entry>,
}

// ===== impl Entry =====

impl Entry {
    pub(crate) fn new(deadline: Timestamp) -> Rc<Entry> {
        let driver = Handle::current().driver().unwrap();

        // Increment the number of active timeouts
        let entry = if driver.increment().is_err() {
            Entry::new2(deadline, Weak::new(), EntryState::Error)
        } else {
            let state = if deadline <= driver.timestamp() {
                EntryState::Elapsed
            } else {
                EntryState::New
            };
            Entry::new2(deadline, Rc::downgrade(&driver), state)
        };

        let entry = Rc::new(entry);
        if driver.notify(&entry).is_err() {
            entry.error();
        }

        entry
    }

    pub(crate) fn is_elapsed(&self) -> bool {
        self.state().is_elapsed()
    }

    pub(crate) fn fire(&self) {
        if self.state_mut().fire() {
            self.wake()
        }
    }

    pub(crate) fn pending(&self) {
        self.state_mut().pending();
    }

    pub(crate) fn error(&self) {
        if self.state_mut().error() {
            self.wake()
        }
    }

    pub(crate) fn cancel(entry: &Rc<Entry>) {
        if entry.state_mut().fire() {
            if let Some(driver) = entry.upgrade_driver() {
                driver.notify(entry).unwrap();
            }
        }
    }

    pub(crate) fn poll_elapsed(&self, cx: &mut task::Context<'_>) -> Poll<Result<(), ()>> {
        if let Poll::Ready(result) = self.state().poll() {
            return Poll::Ready(result);
        }

        *self.waker_mut() = Some(cx.waker().clone());

        Poll::Pending
    }

    /// Only called by `Registration`
    pub(crate) fn update(entry: &mut Rc<Entry>, deadline: Timestamp) {
        let driver = match entry.upgrade_driver() {
            Some(driver) => driver,
            None => return,
        };

        if entry.state() == EntryState::Error || entry.deadline() == deadline {
            return;
        }

        unsafe {
            *entry.deadline.get() = deadline;
        }

        if deadline <= driver.timestamp() {
            let prev = entry.state();
            *entry.state_mut() = EntryState::Elapsed;
            if !prev.is_elapsed() {
                driver.notify(entry).unwrap();
            }
        } else {
            driver.notify(entry).unwrap();
        }
    }

    fn new2(deadline: Timestamp, driver: Weak<Inner>, state: EntryState) -> Self {
        Self {
            deadline: UnsafeCell::new(deadline),
            driver,
            waker: UnsafeCell::new(None),
            state: UnsafeCell::new(state),
            queued: UnsafeCell::new(false),
            notification_next: UnsafeCell::new(ptr::null_mut()),
            wheel_next: UnsafeCell::new(None),
            wheel_prev: UnsafeCell::new(ptr::null_mut()),
        }
    }

    fn upgrade_driver(&self) -> Option<Rc<Inner>> {
        self.driver.upgrade()
    }

    fn wake(&self) {
        if let Some(waker) = self.waker_mut().take() {
            waker.wake()
        }
    }

    pub(super) fn state(&self) -> EntryState {
        unsafe { *&*self.state.get() }
    }

    fn state_mut(&self) -> &mut EntryState {
        unsafe { &mut *self.state.get() }
    }

    pub(super) fn deadline(&self) -> Timestamp {
        unsafe { *&*self.deadline.get() }
    }

    // fn deadline_mut(&self) -> &mut Timestamp {
    //     unsafe { &mut *self.deadline.get() }
    // }

    fn waker_mut(&self) -> &mut Option<Waker> {
        unsafe { &mut *self.waker.get() }
    }

    pub(super) fn notification_next(&self) -> &mut *mut Entry {
        unsafe { &mut *self.notification_next.get() }
    }

    pub(super) fn wheel_next(&self) -> &mut Option<Rc<Entry>> {
        unsafe { &mut *self.wheel_next.get() }
    }

    pub(super) fn wheel_prev(&self) -> &mut *const Entry {
        unsafe { &mut *self.wheel_prev.get() }
    }

    pub(super) fn try_enqueue(&self) -> bool {
        let queued = unsafe { &mut *self.queued.get() };
        if *queued {
            false
        } else {
            *queued = true;
            true
        }
    }

    pub(super) fn dequeue(&self) {
        unsafe {
            *self.queued.get() = false;
        }
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Entry")
            .field("deadline", &self.deadline())
            .field("state", &self.state())
            // .field("queued", &self.queued())
            .finish()
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        if let Some(driver) = self.upgrade_driver() {
            driver.decrement()
        }
    }
}
