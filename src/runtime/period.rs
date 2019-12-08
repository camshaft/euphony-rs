use crate::{
    runtime::timeline::Timeline,
    time::{beat::Beat, compound::CompoundDuration, measure::Measure, timecode::Timecode},
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
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

impl Future for CompoundDuration {
    type Output = Timecode;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let mut timeline = Timeline::current();
        if self.measure == 0 && self.beat == 0 {
            Poll::Ready(timeline.0.timecode())
        } else {
            timeline.0.schedule((*self).into(), ctx.waker());
            self.measure = 0.into();
            self.beat = 0.into();
            Poll::Pending
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Period {
    Duration(Duration),
    CompoundDuration(CompoundDuration),
}

impl From<Duration> for Period {
    fn from(duration: Duration) -> Self {
        Period::Duration(duration)
    }
}

impl From<Beat> for Period {
    fn from(beat: Beat) -> Self {
        Period::CompoundDuration(beat.into())
    }
}

impl From<Measure> for Period {
    fn from(measure: Measure) -> Self {
        Period::CompoundDuration(measure.into())
    }
}

impl From<CompoundDuration> for Period {
    fn from(duration: CompoundDuration) -> Self {
        Period::CompoundDuration(duration)
    }
}
