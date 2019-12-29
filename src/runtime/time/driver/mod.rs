use crate::{
    runtime::time::{
        driver::{
            entry::Entry, handle::Handle, notifications_stack::NotificationsStack,
            wheel_stack::WheelStack,
        },
        wheel::{self, Wheel},
    },
    time::timestamp::Timestamp,
};
use alloc::rc::Rc;
use core::{cell::UnsafeCell, fmt};

pub mod entry;
pub mod handle;
pub mod notifications_stack;
pub mod registration;
pub mod wheel_stack;

#[derive(Debug)]
pub(crate) struct Driver {
    /// Shared state
    inner: Rc<Inner>,

    /// Timer wheel
    wheel: Wheel<WheelStack>,
}

/// Maximum number of timeouts the system can handle concurrently.
const MAX_TIMEOUTS: usize = core::usize::MAX >> 1;

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver {
    pub(crate) fn new() -> Self {
        Driver {
            inner: Rc::new(Inner::new()),
            wheel: Wheel::new(),
        }
    }

    pub(crate) fn handle(&self) -> Handle {
        Handle::new(Rc::downgrade(&self.inner))
    }

    pub fn tick(&mut self, now: Timestamp) -> Option<Timestamp> {
        self.process(now);
        self.prepare_park()
    }

    pub fn prepare_park(&mut self) -> Option<Timestamp> {
        self.wheel.prepare_park(&mut ())
    }

    /// Run timer related logic
    pub fn process(&mut self, now: Timestamp) -> bool {
        let mut has_changes = self.process_queue();

        let mut poll = wheel::Poll::new(now);

        while let Some(entry) = self.wheel.poll(&mut poll, &mut ()) {
            // Fire the entry
            entry.fire();
            has_changes = true;
        }

        // Update the elapsed cache
        unsafe {
            *self.inner.timestamp.get() = self.wheel.timestamp();
        }

        has_changes
    }

    /// Process the entry queue
    ///
    /// This handles adding and canceling timeouts.
    fn process_queue(&mut self) -> bool {
        use entry::EntryState;
        let mut has_changes = false;

        for entry in self.inner.notifications.drain() {
            has_changes = true;
            match entry.state() {
                EntryState::Error | EntryState::Elapsed => {
                    self.remove_entry(&entry);
                }
                EntryState::New => self.insert_entry(entry),
                EntryState::Pending => {
                    self.remove_entry(&entry);
                    self.insert_entry(entry)
                }
            }
        }

        has_changes
    }

    fn remove_entry(&mut self, entry: &Rc<Entry>) {
        self.wheel.remove(entry, &mut ());
    }

    /// Fire the entry if it needs to, otherwise queue it to be processed later.
    ///
    /// Returns `None` if the entry was fired.
    fn insert_entry(&mut self, entry: Rc<Entry>) {
        use wheel::InsertError;
        let deadline = entry.deadline();
        entry.pending();

        match self.wheel.insert(deadline, entry, &mut ()) {
            Ok(_) => {}
            Err((entry, InsertError::Elapsed)) => {
                entry.fire();
            }
            Err((entry, InsertError::Invalid)) => {
                entry.error();
            }
        }
    }
}

impl Drop for Driver {
    fn drop(&mut self) {
        // Shutdown the stack of entries to process, preventing any new entries
        // from being pushed.
        self.inner.notifications.shutdown();

        while let Some(entry) = self.wheel.take(&mut ()) {
            entry.error();
        }
    }
}

// ===== impl Inner =====

/// Timer state shared between `Driver`, `Handle`, and `Registration`.
pub(crate) struct Inner {
    /// The current timestamp
    timestamp: UnsafeCell<Timestamp>,

    /// Number of active timeouts
    num: UnsafeCell<usize>,

    /// Head of the "process" linked list.
    notifications: NotificationsStack,
}

impl Inner {
    fn new() -> Inner {
        Inner {
            num: UnsafeCell::new(0),
            timestamp: Default::default(),
            notifications: Default::default(),
        }
    }

    fn timestamp(&self) -> Timestamp {
        unsafe { *self.timestamp.get() }
    }

    /// Increment the number of active timeouts
    fn increment(&self) -> Result<(), ()> {
        unsafe {
            if let Some(num) = (*&*self.num.get()).checked_add(1) {
                *self.num.get() = num;
                Ok(())
            } else {
                Err(())
            }
        }
    }

    /// Decrement the number of active timeouts
    fn decrement(&self) {
        unsafe {
            *self.num.get() -= 1;
        }
    }

    fn notify(&self, entry: &Rc<Entry>) -> Result<(), ()> {
        self.notifications.push(entry)?;
        Ok(())
    }
}

impl fmt::Debug for Inner {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Inner").finish()
    }
}
