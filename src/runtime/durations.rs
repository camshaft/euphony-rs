use crate::{
    runtime::timeline::Timeline,
    time::{beat::Beat, measure::Measure, timecode::Timecode},
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

impl Future for Beat {
    type Output = Timecode;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let mut timeline = Timeline::current();
        if self.0 == 0 {
            Poll::Ready(timeline.0.timecode())
        } else {
            timeline.0.schedule((*self).into(), ctx.waker());
            self.0 = 0;
            Poll::Pending
        }
    }
}

impl Future for Measure {
    type Output = Timecode;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let mut timeline = Timeline::current();
        if self.0 == 0 {
            Poll::Ready(timeline.0.timecode())
        } else {
            timeline.0.schedule((*self).into(), ctx.waker());
            self.0 = 0;
            Poll::Pending
        }
    }
}
