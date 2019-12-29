use crate::{
    runtime::time::{driver::entry::Entry, wheel},
    time::timestamp::Timestamp,
};
use alloc::rc::Rc;
use core::ptr;

/// A doubly linked stack
#[derive(Debug)]
pub(crate) struct WheelStack {
    head: Option<Rc<Entry>>,
}

impl WheelStack {
    fn debug_contains(&self, entry: &Rc<Entry>) -> bool {
        // This walks the full linked list even if an entry is found.
        let mut next = self.head.as_ref();
        let mut contains = false;

        while let Some(n) = next {
            if Rc::ptr_eq(entry, n) {
                debug_assert!(!contains, "entry was present more than once");
                contains = true;
            }

            next = n.wheel_next().as_ref();
        }

        contains
    }
}

impl Default for WheelStack {
    fn default() -> WheelStack {
        WheelStack { head: None }
    }
}

impl wheel::Stack for WheelStack {
    type Store = ();
    type Value = Rc<Entry>;

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn push(&mut self, entry: Self::Value, _: &mut Self::Store) {
        // Get a pointer to the entry to for the prev link
        let ptr: *const Entry = &*entry as *const _;

        // Remove the old head entry
        let old = self.head.take();

        // Ensure the entry is not already in a stack.
        debug_assert!((*entry.wheel_next()).is_none());
        debug_assert!((*entry.wheel_prev()).is_null());

        if let Some(ref entry) = old.as_ref() {
            debug_assert!({
                // The head is not already set to the entry
                ptr != &***entry as *const _
            });

            // Set the previous link on the old head
            *entry.wheel_prev() = ptr;
        }

        // Set this entry's next pointer
        *entry.wheel_next() = old;

        // Update the head pointer
        self.head = Some(entry);
    }

    /// Pop an item from the stack
    fn pop(&mut self, _: &mut ()) -> Option<Rc<Entry>> {
        let entry = self.head.take();

        if let Some(entry) = entry.as_ref() {
            self.head = (*entry.wheel_next()).take();

            if let Some(entry) = self.head.as_ref() {
                *entry.wheel_prev() = ptr::null();
            }

            *entry.wheel_prev() = ptr::null();
        }

        entry
    }

    fn remove(&mut self, entry: &Rc<Entry>, _: &mut ()) {
        // Ensure that the entry is in fact contained by the stack
        debug_assert!(self.debug_contains(entry));

        unsafe {
            // Unlink `entry` from the next node
            let next = (*entry.wheel_next()).take();

            if let Some(next) = next.as_ref() {
                (*next.wheel_prev()) = *entry.wheel_prev();
            }

            // Unlink `entry` from the prev node

            if let Some(prev) = (*entry.wheel_prev()).as_ref() {
                *prev.wheel_next() = next;
            } else {
                // It is the head
                self.head = next;
            }

            // Unset the prev pointer
            *entry.wheel_prev() = ptr::null();
        }
    }

    fn when(item: &Rc<Entry>, _: &()) -> Timestamp {
        item.deadline()
    }
}
