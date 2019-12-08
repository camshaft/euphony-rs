use crate::{
    runtime::{cell::Cell, period::Period},
    time::{
        beat::Beat, measure::Measure, tempo::Tempo, time_signature::TimeSignature,
        timecode::Timecode, timestamp::Timestamp,
    },
};
use alloc::collections::BinaryHeap;
use core::{
    cmp::{Ordering, Reverse},
    future::Future,
    mem::replace,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::Duration,
};

#[derive(Clone, Debug)]
pub struct TimelineEvent {
    source: Timecode,
    period: Period,
    waker: Waker,
}

impl TimelineEvent {
    fn recompute(&mut self, _timecode: &Timecode) {
        match &mut self.period {
            Period::Duration(_) => {}
            Period::CompoundDuration(_) => unimplemented!(),
        }
    }

    fn target_beat(&self) -> Beat {
        // TODO
        self.source.beat
    }

    fn target_measure(&self) -> Measure {
        // TODO
        self.source.measure
    }

    fn target_timestamp(&self) -> Timestamp {
        self.source.timestamp + self.target_duration()
    }

    fn target_duration(&self) -> Duration {
        self.source.tempo
            * match self.period {
                Period::Duration(duration) => return duration,
                Period::CompoundDuration(duration) => duration * self.source.time_signature,
            }
    }
}

impl PartialEq for TimelineEvent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for TimelineEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TimelineEvent {}

impl Ord for TimelineEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.target_timestamp().cmp(&other.target_timestamp())
    }
}

type InnerTimelineCell = Cell<InnerTimeline>;

#[must_use]
#[derive(Debug)]
pub struct TimelineFuture {
    state: TimelineFutureState,
    timeline: InnerTimelineCell,
}

#[derive(Debug)]
enum TimelineFutureState {
    Pending(Period),
    Waiting,
}

impl Future for TimelineFuture {
    type Output = Timecode;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        match replace(&mut self.state, TimelineFutureState::Waiting) {
            TimelineFutureState::Pending(period) => {
                self.timeline.schedule(period, ctx.waker());
                Poll::Pending
            }
            TimelineFutureState::Waiting => Poll::Ready(self.timeline.timecode()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Composition {
    timestamp: Timestamp,
    timelines: Cell<Vec<InnerTimelineCell>>,
}

pub struct CompositionEvent {
    timestamp: Timestamp,
    timeline: InnerTimelineCell,
}

impl Composition {
    pub fn new_timeline(&self) -> Timeline {
        let mut inner_timeline = InnerTimelineCell::default();
        inner_timeline.timestamp = self.timestamp;
        self.timelines
            .update(|timelines| timelines.push(inner_timeline.clone()));
        Timeline(inner_timeline)
    }

    pub fn wake(&mut self, mut event: CompositionEvent) {
        self.timestamp = event.timestamp;
        event.timeline.wake()
    }

    pub fn next_event(&self) -> Option<CompositionEvent> {
        self.timelines
            .iter()
            .filter_map(|timeline| {
                let event = timeline.next_event()?;
                let timestamp = event.target_timestamp();
                Some((timeline, timestamp))
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(timeline, timestamp)| CompositionEvent {
                timestamp,
                timeline: timeline.clone(),
            })
    }
}

#[derive(Debug, Default)]
pub struct InnerTimeline {
    tempo: Tempo,
    time_signature: TimeSignature,
    events: BinaryHeap<Reverse<TimelineEvent>>,
    timestamp: Timestamp,
    beat: Beat,
    measure: Measure,
}

impl InnerTimeline {
    pub(crate) fn timecode(&self) -> Timecode {
        Timecode {
            tempo: self.tempo,
            time_signature: self.time_signature,
            timestamp: self.timestamp,
            beat: self.beat,
            measure: self.measure,
        }
    }

    pub(crate) fn schedule(&mut self, period: Period, waker: &Waker) {
        let source = self.timecode();
        let waker = waker.clone();
        let event = TimelineEvent {
            source,
            period,
            waker,
        };
        self.events.push(Reverse(event));
    }

    pub(crate) fn next_event(&self) -> Option<&TimelineEvent> {
        self.events.peek().map(|Reverse(event)| event)
    }

    fn recompute(&mut self) {
        let mut events = replace(&mut self.events, Default::default()).into_vec();

        let timecode = self.timecode();
        for Reverse(event) in events.iter_mut() {
            event.recompute(&timecode);
        }

        self.events = BinaryHeap::from(events);
    }

    fn wake(&mut self) {
        let Reverse(event) = self.events.pop().expect("Timeline woken without event");
        self.timestamp = event.target_timestamp();
        self.beat = event.target_beat();
        self.measure = event.target_measure();
        event.waker.wake();
    }
}

#[derive(Clone, Debug, Default)]
pub struct Timeline(pub(crate) InnerTimelineCell);

impl Timeline {
    pub fn current() -> Self {
        unimplemented!()
    }

    pub fn spawn<F: Future<Output = ()>>(&self, _future: F) {
        // TODO
    }

    pub fn rest<P: Into<Period>>(&self, period: P) -> TimelineFuture {
        TimelineFuture {
            state: TimelineFutureState::Pending(period.into()),
            timeline: self.0.clone(),
        }
    }

    pub fn set_tempo<T: Into<Tempo>>(&mut self, tempo: T) {
        self.0.tempo = tempo.into();
        self.0.recompute();
    }

    pub fn set_time_signature<T: Into<TimeSignature>>(&mut self, time_signature: T) {
        self.0.time_signature = time_signature.into();
        self.0.recompute();
    }
}

#[test]
fn async_test() {
    let composition = Composition::default();
    let timeline = composition.new_timeline();
    timeline.spawn(async {
        loop {
            let timecode = Beat(1, 4).await;
            println!("{:?}", timecode);
        }
    });
}
