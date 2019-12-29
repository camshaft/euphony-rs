use crate::{
    runtime::{
        future::poll_fn::poll_fn,
        time::delay::{delay_until, Delay},
    },
    time::timestamp::Timestamp,
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use futures_core::ready;

pub fn interval(period: Duration) -> Interval {
    assert!(period > Duration::new(0, 0), "`period` must be non-zero.");

    interval_at(Timestamp::now(), period)
}

pub fn interval_at(start: Timestamp, period: Duration) -> Interval {
    assert!(period > Duration::new(0, 0), "`period` must be non-zero.");

    Interval {
        delay: delay_until(start),
        period,
    }
}

/// Stream returned by [`interval`](interval) and [`interval_at`](interval_at).
#[derive(Debug)]
pub struct Interval {
    /// Future that completes the next time the `Interval` yields a value.
    delay: Delay,

    /// The duration between values yielded by `Interval`.
    period: Duration,
}

impl Interval {
    pub fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<Timestamp> {
        // Wait for the delay to be done
        ready!(Pin::new(&mut self.delay).poll(cx));

        // Get the `now` by looking at the `delay` deadline
        let now = self.delay.deadline();

        // The next interval value is `duration` after the one that just
        // yielded.
        self.delay.update(now + self.period);

        // Return the current instant
        Poll::Ready(now)
    }

    pub async fn tick(&mut self) -> Timestamp {
        poll_fn(|cx| self.poll_tick(cx)).await
    }
}

impl futures_core::Stream for Interval {
    type Item = Timestamp;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Timestamp>> {
        Poll::Ready(Some(ready!(self.poll_tick(cx))))
    }
}
