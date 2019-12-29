use crate::runtime::time::driver::Entry;
use alloc::rc::Rc;
use core::{cell::UnsafeCell, ptr};

/// A stack of `Entry` nodes
#[derive(Debug)]
pub(crate) struct NotificationsStack {
    /// Stack head
    head: UnsafeCell<*mut Entry>,
}

/// Entries that were removed from the stack
#[derive(Debug)]
pub(crate) struct NotificationsStackEntries {
    ptr: *mut Entry,
}

/// Used to indicate that the timer has shutdown.
const SHUTDOWN: *mut Entry = 1 as *mut _;

impl Default for NotificationsStack {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationsStack {
    pub(crate) fn new() -> NotificationsStack {
        NotificationsStack {
            head: UnsafeCell::new(ptr::null_mut()),
        }
    }

    /// Push an entry onto the stack.
    ///
    /// Returns `true` if the entry was pushed, `false` if the entry is already
    /// on the stack, `Err` if the timer is shutdown.
    pub(crate) fn push(&self, entry: &Rc<Entry>) -> Result<bool, ()> {
        if !entry.try_enqueue() {
            // Already queued, nothing more to do
            return Ok(false);
        }

        let ptr = Rc::into_raw(entry.clone()) as *mut _;

        let head = unsafe { &mut *self.head.get() };

        if *head == SHUTDOWN {
            // Don't leak the entry node
            let _ = unsafe { Rc::from_raw(ptr) };

            return Err(());
        }

        *entry.notification_next() = *head;

        *head = ptr;

        Ok(true)
    }

    /// Take all entries from the stack
    pub(crate) fn drain(&self) -> NotificationsStackEntries {
        let ptr = self.replace_head(ptr::null_mut());
        NotificationsStackEntries { ptr }
    }

    /// Drain all remaining nodes in the stack and prevent any new nodes from
    /// being pushed onto the stack.
    pub(crate) fn shutdown(&self) {
        let ptr = self.replace_head(SHUTDOWN);

        // Let the drop fn of `NotificationsStackEntries` handle draining the stack
        drop(NotificationsStackEntries { ptr });
    }

    fn replace_head(&self, ptr: *mut Entry) -> *mut Entry {
        core::mem::replace(unsafe { &mut *self.head.get() }, ptr)
    }
}

// ===== impl NotificationsStackEntries =====

impl Iterator for NotificationsStackEntries {
    type Item = Rc<Entry>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            return None;
        }

        // Convert the pointer to an `Rc<Entry>`
        let entry = unsafe { Rc::from_raw(self.ptr) };

        // Update `self.ptr` to point to the next element of the stack
        self.ptr = *entry.notification_next();

        // Unset the queued flag
        entry.dequeue();

        // Return the entry
        Some(entry)
    }
}

impl Drop for NotificationsStackEntries {
    fn drop(&mut self) {
        for entry in self {
            // Flag the entry as errored
            entry.error();
        }
    }
}
