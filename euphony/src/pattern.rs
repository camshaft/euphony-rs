use core::{
    pin::Pin,
    task::{Context, Poll},
};
use euphony_core::time::beat::Instant;
use euphony_pattern::{Pattern, ResultExt, Status};
use euphony_runtime::time::Timer;
use futures::stream::Stream;
use pin_project::pin_project;

pub trait PatternStreamExt: Pattern {
    fn as_stream(self) -> PatternStream<Self> {
        PatternStream {
            pattern: self,
            start_time: Instant(0, 1),
            expansion: 0,
            is_finished: false,
            timer: None,
        }
    }

    fn read(&self) -> Option<Self::Output> {
        let now = crate::runtime::time::scheduler().beats();
        let pattern_cx = euphony_pattern::Context::new(now, 0);
        let result = self.poll(&pattern_cx);
        let (value, status) = result.split_status();
        value
    }
}

impl<P: Pattern> PatternStreamExt for P {}

#[pin_project]
pub struct PatternStream<P: Pattern> {
    pattern: P,
    start_time: Instant,
    expansion: usize,
    is_finished: bool,
    timer: Option<Timer>,
}

impl<P: Pattern> Stream for PatternStream<P> {
    type Item = P::Output;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;

        if this.is_finished {
            return None.into();
        }

        if let Some(timer) = this.timer.as_mut() {
            if timer.poll_unpin(cx).is_pending() {
                return Poll::Pending;
            }
            this.timer = None;
        }

        let now = crate::runtime::time::scheduler().beats() - this.start_time;
        let pattern_cx = euphony_pattern::Context::new(now, this.expansion);
        let result = this.pattern.poll(&pattern_cx);
        let (value, status) = result.split_status();

        match status {
            Status::Pending(time) => {
                let delay = (time - now).as_ratio().into();
                this.timer = Some(crate::runtime::time::scheduler().delay(delay));
            }
            Status::Continuous => {
                this.is_finished = true;
            }
        }

        if let Some(value) = value {
            Some(value).into()
        } else {
            Poll::Pending
        }
    }
}
