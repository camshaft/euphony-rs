use crate::{
    runtime::time::{clock, driver::registration::Registration},
    time::timestamp::Timestamp,
};
use core::{
    future::Future,
    pin::Pin,
    task::{self, Poll},
    time::Duration,
};
use futures_core::ready;

/// Wait until `deadline` is reached.
///
/// No work is performed while awaiting on the delay to complete.
///
/// # Cancellation
///
/// Canceling a delay is done by dropping the returned future. No additional
/// cleanup work is required.
pub fn delay_until(deadline: Timestamp) -> Delay {
    Delay {
        registration: Registration::new(deadline),
    }
}

/// Wait until `duration` has elapsed.
///
/// Equivalent to `delay_until(Timestamp::now() + duration)`. An asynchronous
/// analog to `std::thread::sleep`.
///
/// No work is performed while awaiting on the delay to complete. The delay
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
///
/// # Cancellation
///
/// Canceling a delay is done by dropping the returned future. No additional
/// cleanup work is required.
pub fn delay_for(duration: Duration) -> Delay {
    delay_until(Timestamp::now() + duration)
}

/// Future returned by [`delay_until`](delay_until) and
/// [`delay_for`](delay_for).
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Delay {
    /// The link between the `Delay` instance and the timer that drives it.
    ///
    /// This also stores the `deadline` value.
    registration: Registration,
}

impl Delay {
    /// Returns the instant at which the future will complete.
    pub fn deadline(&self) -> Timestamp {
        self.registration.deadline()
    }

    /// Returns true if the `Delay` has elapsed
    ///
    /// A `Delay` is elapsed when the requested duration has elapsed.
    pub fn is_elapsed(&self) -> bool {
        self.registration.is_elapsed()
    }

    /// Updates the `Delay` instance to a new deadline.
    pub fn update(&mut self, deadline: Timestamp) {
        self.registration.update(deadline);
    }
}

impl Future for Delay {
    type Output = Timestamp;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        ready!(self.registration.poll_elapsed(cx)).expect("timer error");

        // Set the clock to the delay's timestamp
        clock::set(self.deadline());

        Poll::Ready(self.deadline())
    }
}
