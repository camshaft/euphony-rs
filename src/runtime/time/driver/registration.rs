use crate::{runtime::time::driver::entry::Entry, time::timestamp::Timestamp};
use alloc::rc::Rc;
use core::task::{self, Poll};

/// Registration with a timer.
///
/// The association between a `Delay` instance and a timer is done lazily in
/// `poll`
#[derive(Debug)]
pub(crate) struct Registration {
    entry: Rc<Entry>,
}

impl Registration {
    pub(crate) fn new(deadline: Timestamp) -> Registration {
        Registration {
            entry: Entry::new(deadline),
        }
    }

    pub(crate) fn deadline(&self) -> Timestamp {
        self.entry.deadline()
    }

    pub(crate) fn update(&mut self, deadline: Timestamp) {
        Entry::update(&mut self.entry, deadline);
    }

    pub(crate) fn is_elapsed(&self) -> bool {
        self.entry.is_elapsed()
    }

    pub(crate) fn poll_elapsed(&self, cx: &mut task::Context<'_>) -> Poll<Result<(), ()>> {
        self.entry.poll_elapsed(cx)
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        Entry::cancel(&self.entry);
    }
}
