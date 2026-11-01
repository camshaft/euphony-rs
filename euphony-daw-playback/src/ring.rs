//! The ring between the prepared-audio reader and a device callback.
//!
//! A reader on a normal-priority thread copies prepared audio into a
//! single-producer single-consumer ring of binary32 samples. The device
//! callback only pops and copies from it: where the ring runs short it
//! writes silence for the rest of the period, counts an underrun and reports
//! it on an event ring. Neither side allocates, locks or blocks in the
//! callback.

use crate::Stream;
use rtrb::{Consumer, Producer, RingBuffer};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread::Thread,
    time::Duration,
};

/// The events a callback ring holds before it counts the rest as dropped.
pub const EVENT_CAPACITY: usize = 64;

/// How many of the device's measured periods a ring holds, by default.
pub const DEFAULT_PERIODS_AHEAD: usize = 4;

/// What playback reports while it runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackEvent {
    /// The ring ran short and the callback played silence for these frames.
    Underrun { missing_frames: u64 },
    /// Every frame of the audio was played.
    Ended,
    /// The device was disconnected or is gone.
    DeviceLost,
    /// The platform rerouted the stream to another device.
    DeviceChanged,
    /// The stream's format changed under it, and it must be built again.
    StreamInvalidated,
    /// The platform reported an underrun or overrun of its own buffer.
    Xrun,
    /// The callback thread wasn't promoted to real-time priority, and plays
    /// at normal priority.
    RealtimeDenied,
    /// The transport failed for another reason.
    Transport(TransportError),
}

/// Why a transport failed, where it isn't a device loss or change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportError {
    DeviceBusy,
    HostUnavailable,
    PermissionDenied,
    ResourceExhausted,
    Backend,
    Other,
}

/// The counts the callback keeps, which any thread may read.
#[derive(Debug, Default)]
pub struct Counters {
    underruns: AtomicU64,
    missing_frames: AtomicU64,
    dropped_events: AtomicU64,
    latency_nanos: AtomicU64,
    latency_known: AtomicBool,
}

impl Counters {
    /// How many callbacks found the ring short.
    pub fn underruns(&self) -> u64 {
        self.underruns.load(Ordering::Acquire)
    }

    /// How many frames of silence the underruns played.
    pub fn missing_frames(&self) -> u64 {
        self.missing_frames.load(Ordering::Acquire)
    }

    /// How many events didn't fit in their ring.
    pub fn dropped_events(&self) -> u64 {
        self.dropped_events.load(Ordering::Acquire)
    }

    /// The latest latency from a callback to the playback of its first
    /// frame, as the stream's timestamps measure it.
    pub fn latency(&self) -> Option<Duration> {
        if !self.latency_known.load(Ordering::Acquire) {
            return None;
        }
        let nanos = self.latency_nanos.load(Ordering::Acquire);
        Some(Duration::from_nanos(nanos))
    }

    /// Records a callback's latency.
    pub fn record_latency(&self, latency: Duration) {
        let nanos = u64::try_from(latency.as_nanos()).unwrap_or(u64::MAX);
        self.latency_nanos.store(nanos, Ordering::Release);
        self.latency_known.store(true, Ordering::Release);
    }

    pub(crate) fn drop_event(&self) {
        self.dropped_events.fetch_add(1, Ordering::AcqRel);
    }
}

/// The ring's three ends: the reader that fills it, the callback that
/// drains it, and the events they report.
pub struct Ring {
    pub reader: Reader,
    pub callback: Callback,
    pub events: Events,
}

/// Makes a ring for `stream` that holds `capacity_frames` frames of
/// `channels` samples each.
///
/// The caller sizes it from the device's measured buffer size.
pub fn ring(stream: Stream, channels: u16, capacity_frames: usize) -> Ring {
    ring_with(stream, channels, capacity_frames, Arc::default())
}

/// Makes a ring as [`ring`] does, whose callback keeps its counts in
/// `counters`.
pub fn ring_with(
    stream: Stream,
    channels: u16,
    capacity_frames: usize,
    counters: Arc<Counters>,
) -> Ring {
    let channels = usize::from(channels.max(1));
    let capacity = capacity_frames.max(1) * channels;
    let (producer, consumer) = RingBuffer::new(capacity);
    let (event_producer, event_consumer) = RingBuffer::new(EVENT_CAPACITY);
    let waiting = Arc::new(AtomicBool::new(false));
    Ring {
        reader: Reader {
            stream,
            producer: Some(producer),
            scratch: vec![0.0; capacity],
            channels,
            waiting: waiting.clone(),
        },
        callback: Callback {
            consumer,
            events: event_producer,
            counters: counters.clone(),
            channels,
            waiting,
            reader: None,
            ended: false,
        },
        events: Events {
            callback: event_consumer,
            device: None,
            counters,
        },
    }
}

/// What one fill of the ring did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fill {
    /// It copied this many frames into the ring.
    Filled(usize),
    /// The ring has no room for a whole frame.
    Full,
    /// Every frame of the audio is in the ring, and the reader let it go.
    Finished,
    /// The callback's end is gone.
    Abandoned,
}

/// The ring's producer, which a normal-priority thread runs.
pub struct Reader {
    stream: Stream,
    producer: Option<Producer<f32>>,
    scratch: Vec<f32>,
    channels: usize,
    waiting: Arc<AtomicBool>,
}

impl Reader {
    /// Copies at most `max_frames` whole frames of the audio into the ring.
    pub fn fill(&mut self, max_frames: usize) -> Fill {
        let Some(producer) = self.producer.as_mut() else {
            return Fill::Finished;
        };
        if producer.is_abandoned() {
            self.producer = None;
            return Fill::Abandoned;
        }
        if self.stream.finished() {
            self.producer = None;
            return Fill::Finished;
        }
        let room = producer.slots().min(self.scratch.len()) / self.channels;
        if room == 0 {
            return Fill::Full;
        }
        let frames = room.min(max_frames);
        let out = &mut self.scratch[..frames * self.channels];
        let count = self.stream.fill(out);
        let (_, rest) = producer.push_partial_slice(&out[..count]);
        debug_assert!(rest.is_empty());
        if self.stream.finished() {
            self.producer = None;
            return Fill::Finished;
        }
        Fill::Filled(count / self.channels)
    }

    /// Parks the thread until the callback frees room in a full ring, or
    /// until a caller unparks it.
    pub fn wait(&mut self) {
        self.waiting.store(true, Ordering::SeqCst);
        let full = match &self.producer {
            Some(producer) => producer.slots() < self.channels && !producer.is_abandoned(),
            None => false,
        };
        if full {
            std::thread::park();
        }
        self.waiting.store(false, Ordering::SeqCst);
    }

    /// Fills the ring until the audio ends or the callback's end is gone,
    /// waiting while the ring is full.
    pub fn run(mut self) {
        loop {
            match self.fill(usize::MAX) {
                Fill::Filled(_) => {}
                Fill::Full => self.wait(),
                Fill::Finished | Fill::Abandoned => return,
            }
        }
    }
}

/// What one callback period played.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Render {
    /// The whole period came from the audio.
    Played,
    /// The ring ran short, and this many frames were silence.
    Underrun { missing_frames: usize },
    /// The audio ended in or before this period: its first `played_frames`
    /// frames came from the audio, and the rest is silence.
    Ended { played_frames: usize },
}

/// The ring's consumer, which the device callback runs.
pub struct Callback {
    consumer: Consumer<f32>,
    events: Producer<PlaybackEvent>,
    counters: Arc<Counters>,
    channels: usize,
    waiting: Arc<AtomicBool>,
    reader: Option<Thread>,
    ended: bool,
}

impl Callback {
    /// The thread the callback unparks once it frees room for a waiting
    /// reader.
    pub fn wake(&mut self, reader: Thread) {
        self.reader = Some(reader);
    }

    /// The counts this callback keeps.
    pub fn counters(&self) -> &Arc<Counters> {
        &self.counters
    }

    /// Fills one period: it pops and copies the ring's samples, and writes
    /// silence where the ring runs short.
    ///
    /// It doesn't allocate, lock or block.
    pub fn render(&mut self, out: &mut [f32]) -> Render {
        let (filled, rest) = self.consumer.pop_partial_slice(out);
        let played = filled.len();
        let missing = rest.len();
        rest.fill(0.0);
        if played > 0 && self.waiting.swap(false, Ordering::SeqCst) {
            if let Some(reader) = &self.reader {
                reader.unpark();
            }
        }
        if missing == 0 {
            return Render::Played;
        }
        if self.consumer.is_abandoned() && self.consumer.is_empty() {
            if !self.ended {
                self.ended = true;
                self.report(PlaybackEvent::Ended);
            }
            return Render::Ended {
                played_frames: played / self.channels,
            };
        }
        let missing_frames = missing / self.channels;
        let frames = missing_frames as u64;
        let counters = &self.counters;
        counters.underruns.fetch_add(1, Ordering::AcqRel);
        counters.missing_frames.fetch_add(frames, Ordering::AcqRel);
        self.report(PlaybackEvent::Underrun {
            missing_frames: frames,
        });
        Render::Underrun { missing_frames }
    }

    fn report(&mut self, event: PlaybackEvent) {
        if self.events.push(event).is_err() {
            self.counters.drop_event();
        }
    }
}

/// The events playback reports, from the callback and from the device.
pub struct Events {
    callback: Consumer<PlaybackEvent>,
    device: Option<Consumer<PlaybackEvent>>,
    counters: Arc<Counters>,
}

impl Events {
    /// Adds the ring a device's error callback reports on.
    pub fn with_device(&mut self, device: Consumer<PlaybackEvent>) {
        self.device = Some(device);
    }

    /// The next event, the device's first.
    pub fn pop(&mut self) -> Option<PlaybackEvent> {
        if let Some(event) = self.device.as_mut().and_then(|device| device.pop().ok()) {
            return Some(event);
        }
        self.callback.pop().ok()
    }

    /// The counts the callback keeps.
    pub fn counters(&self) -> &Arc<Counters> {
        &self.counters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audio(frames: usize) -> crate::PreparedAudio {
        let samples: Vec<f32> = (0..frames * 2).map(|i| (i + 1) as f32).collect();
        let layout = euphony_daw_sample::ChannelLayout::Stereo;
        crate::PreparedAudio::new(48_000, layout, samples)
    }

    #[test]
    fn a_short_ring_plays_silence_and_counts_each_underrun() {
        let prepared = audio(10);
        let Ring {
            mut reader,
            mut callback,
            mut events,
        } = ring(prepared.stream(), 2, 4);

        assert_eq!(reader.fill(3), Fill::Filled(3));
        let mut period = [f32::NAN; 8];
        assert_eq!(
            callback.render(&mut period),
            Render::Underrun { missing_frames: 1 }
        );
        assert_eq!(period, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 0.0]);
        assert_eq!(events.counters().underruns(), 1);
        assert_eq!(events.counters().missing_frames(), 1);
        assert_eq!(
            events.pop(),
            Some(PlaybackEvent::Underrun { missing_frames: 1 })
        );

        let mut period = [f32::NAN; 8];
        assert_eq!(
            callback.render(&mut period),
            Render::Underrun { missing_frames: 4 }
        );
        assert_eq!(period, [0.0; 8]);
        assert_eq!(events.counters().underruns(), 2);
        assert_eq!(events.counters().missing_frames(), 5);

        assert_eq!(reader.fill(usize::MAX), Fill::Filled(4));
        assert_eq!(reader.fill(usize::MAX), Fill::Full);
        let mut period = [f32::NAN; 8];
        assert_eq!(callback.render(&mut period), Render::Played);
        assert_eq!(period, [7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0]);

        assert_eq!(reader.fill(usize::MAX), Fill::Finished);
        let mut period = [f32::NAN; 8];
        assert_eq!(
            callback.render(&mut period),
            Render::Ended { played_frames: 3 }
        );
        assert_eq!(&period[..6], &[15.0, 16.0, 17.0, 18.0, 19.0, 20.0]);
        assert_eq!(&period[6..], &[0.0, 0.0]);
        assert_eq!(events.counters().underruns(), 2);
        assert_eq!(
            events.pop(),
            Some(PlaybackEvent::Underrun { missing_frames: 4 })
        );
        assert_eq!(events.pop(), Some(PlaybackEvent::Ended));
        assert_eq!(events.pop(), None);
    }

    #[test]
    fn a_full_event_ring_counts_the_events_it_drops() {
        let prepared = audio(1);
        let Ring {
            mut reader,
            mut callback,
            events,
        } = ring(prepared.stream(), 2, 1);
        assert_eq!(reader.fill(0), Fill::Filled(0));
        let mut period = [0.0f32; 2];
        for _ in 0..EVENT_CAPACITY + 3 {
            callback.render(&mut period);
        }
        let counters = events.counters();
        assert_eq!(counters.underruns(), (EVENT_CAPACITY + 3) as u64);
        assert_eq!(counters.dropped_events(), 3);
    }

    #[test]
    fn the_reader_stops_once_the_callback_is_gone() {
        let prepared = audio(10);
        let Ring {
            mut reader,
            callback,
            ..
        } = ring(prepared.stream(), 2, 2);
        assert_eq!(reader.fill(usize::MAX), Fill::Filled(2));
        drop(callback);
        assert_eq!(reader.fill(usize::MAX), Fill::Abandoned);
        assert_eq!(reader.fill(usize::MAX), Fill::Finished);
    }

    #[test]
    fn a_reader_thread_plays_every_frame_through_a_small_ring() {
        let prepared = audio(1000);
        let Ring {
            reader,
            mut callback,
            mut events,
        } = ring(prepared.stream(), 2, 16);
        let thread = std::thread::spawn(move || reader.run());
        callback.wake(thread.thread().clone());

        let mut played = Vec::new();
        let mut period = [0.0f32; 32];
        loop {
            match callback.render(&mut period) {
                Render::Played => played.extend_from_slice(&period),
                Render::Underrun { missing_frames } => {
                    let frames = period.len() / 2 - missing_frames;
                    played.extend_from_slice(&period[..frames * 2]);
                    std::thread::yield_now();
                }
                Render::Ended { played_frames } => {
                    played.extend_from_slice(&period[..played_frames * 2]);
                    break;
                }
            }
        }
        thread.join().unwrap();
        for (played, expected) in played.iter().zip(prepared.samples()) {
            assert_eq!(played.to_bits(), expected.to_bits());
        }
        assert_eq!(played.len(), prepared.samples().len());
        let mut ended = false;
        while let Some(event) = events.pop() {
            ended |= event == PlaybackEvent::Ended;
        }
        assert!(ended);
    }
}
