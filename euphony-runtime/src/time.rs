use bach::time::{
    entry::atomic::{self, ArcEntry},
    wheel::Wheel,
};
use core::{
    fmt,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
    time::Duration,
};
use euphony_core::time::{Beat, Tempo};
use flume::{Receiver, Sender};
use std::sync::Arc;

pub struct Scheduler {
    handle: Handle,
    wheel: InnerWheel,
}

impl fmt::Debug for Scheduler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Scheduler")
            .field("beats", &self.beats())
            .field("now", &self.now())
            .finish()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new(Tempo::default(), Beat(1, 256), None)
    }
}

impl Scheduler {
    /// Creates a new Scheduler
    pub fn new(tempo: Tempo, min_beat: Beat, capacity: Option<usize>) -> Self {
        let (wheel, sender) = InnerWheel::new(capacity);
        let handle = Handle::new(tempo, min_beat, sender);

        Self { handle, wheel }
    }

    /// Returns a handle that can be easily cloned
    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    /// Returns the amount of time until the next task
    ///
    /// An implementation may sleep for the duration.
    pub fn advance(&mut self) -> Option<Duration> {
        let ticks = self.wheel.advance()?;
        let time = self.handle.advance(ticks);

        Some(time)
    }

    /// Wakes all of the expired tasks
    pub fn wake(&mut self) -> usize {
        self.wheel.wake()
    }
}

struct InnerWheel {
    wheel: Wheel<ArcEntry>,
    queue: Receiver<ArcEntry>,
}

impl InnerWheel {
    pub fn new(capacity: Option<usize>) -> (Self, Sender<ArcEntry>) {
        let (sender, queue) = if let Some(capacity) = capacity {
            flume::bounded(capacity)
        } else {
            flume::unbounded()
        };

        let wheel = Self {
            wheel: Default::default(),
            queue,
        };

        (wheel, sender)
    }

    pub fn advance(&mut self) -> Option<u64> {
        self.update();

        self.wheel.advance()
    }

    pub fn wake(&mut self) -> usize {
        self.wheel.wake(atomic::wake)
    }

    /// Move the queued entries into the wheel
    fn update(&mut self) {
        for entry in self.queue.try_iter() {
            self.wheel.insert(entry);
        }
    }
}

impl core::ops::Deref for Scheduler {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

#[derive(Debug, Clone)]
pub struct Handle(Arc<InnerHandle>);

impl Handle {
    fn new(tempo: Tempo, min_beat: Beat, sender: Sender<ArcEntry>) -> Self {
        let inner = InnerHandle {
            ticks: AtomicU64::new(0),
            micros: AtomicU64::new(0),
            sender,
            tempo_micros: AtomicU64::new(Self::tempo_to_micros(tempo)),
            beats_per_tick: min_beat,
        };
        Self(Arc::new(inner))
    }

    /// Returns a future that sleeps for the given duration
    pub fn delay(&self, beat: Beat) -> Timer {
        let ticks = self.beats_to_ticks(beat);

        let entry = atomic::Entry::new(ticks);
        let handle = self.clone();
        Timer { handle, entry }
    }

    pub fn now(&self) -> Duration {
        Duration::from_micros(self.0.micros.load(Ordering::SeqCst))
    }

    /// Returns the current number of beats that has passed for this scheduler
    pub fn beats(&self) -> Beat {
        self.ticks_to_beats(self.ticks())
    }

    /// Updates the tempo for the given scheduler
    pub fn set_tempo(&self, tempo: Tempo) {
        self.0
            .tempo_micros
            .store(Self::tempo_to_micros(tempo), Ordering::SeqCst);
    }

    fn advance(&self, ticks: u64) -> Duration {
        if cfg!(test) {
            self.0
                .ticks
                .load(Ordering::SeqCst)
                .checked_add(ticks)
                .expect("tick overflow");
        }
        self.0.ticks.fetch_add(ticks, Ordering::SeqCst);
        let duration = self.beats_to_time(self.ticks_to_beats(ticks));
        self.0
            .micros
            .fetch_add(duration.as_micros() as _, Ordering::SeqCst);
        duration
    }

    fn beats_to_ticks(&self, beat: Beat) -> u64 {
        let ticks = beat / self.0.beats_per_tick;
        ticks.whole()
    }

    fn ticks_to_beats(&self, ticks: u64) -> Beat {
        self.0.beats_per_tick * ticks
    }

    fn beats_to_time(&self, beat: Beat) -> Duration {
        self.beat_duration() * beat.as_ratio()
    }

    fn ticks(&self) -> u64 {
        self.0.ticks.load(Ordering::SeqCst)
    }

    fn beat_duration(&self) -> Duration {
        Duration::from_micros(self.0.tempo_micros.load(Ordering::SeqCst))
    }

    fn tempo_to_micros(tempo: Tempo) -> u64 {
        tempo.as_beat_duration().as_micros() as u64
    }
}

#[derive(Debug)]
struct InnerHandle {
    ticks: AtomicU64,
    micros: AtomicU64,
    tempo_micros: AtomicU64,
    sender: Sender<ArcEntry>,
    beats_per_tick: Beat,
}

impl Handle {
    fn register(&self, entry: &ArcEntry) {
        self.0.sender.send(entry.clone()).expect("send queue full")
    }
}

/// A future that sleeps a task for a duration
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Timer {
    handle: Handle,
    entry: ArcEntry,
}

impl Timer {
    /// Cancels the timer
    pub fn cancel(&mut self) {
        self.entry.cancel();
    }

    pub fn poll_unpin(&mut self, cx: &mut Context) -> Poll<()> {
        // check condition before to avoid needless registration
        if self.entry.take_expired() {
            return Poll::Ready(());
        }

        // register the waker with the entry
        self.entry.register(cx.waker());

        // check condition after registration to avoid loss of notification
        if self.entry.take_expired() {
            return Poll::Ready(());
        }

        // register the timer with the handle
        if self.entry.should_register() {
            self.handle.register(&self.entry);
        }

        Poll::Pending
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.cancel();
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        self.poll_unpin(cx)
    }
}
