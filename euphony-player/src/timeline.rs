use crate::project::BufferHandle;
use core::time::Duration;
use std::sync::{
    atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering},
    Arc,
};

pub type Sample = f32;
pub type Buffer = Arc<Vec<Sample>>;

pub struct Timeline {
    playhead: Playhead,
}

impl Timeline {
    pub fn new(buffer: BufferHandle) -> Self {
        let playhead = Playhead(Arc::new(Controls::new(buffer)));

        Self { playhead }
    }

    pub fn playing(&self) -> bool {
        self.playhead.0.playing.load(Ordering::Relaxed)
    }

    pub fn looping(&self) -> bool {
        self.playhead.0.looping.load(Ordering::Relaxed)
    }

    pub fn clipped(&self) -> bool {
        self.playhead.0.clipped.load(Ordering::Relaxed)
    }

    pub fn volume(&self) -> u8 {
        self.playhead.0.volume()
    }

    pub fn update(&mut self, update: Update) {
        if let Some(playing) = update.playing {
            self.playhead.0.set_playing(playing);
        }

        if let Some(looping) = update.looping {
            self.playhead.0.set_looping(looping);
        }

        if let Some(pos) = update.clip_start {
            if let Some(pos) = pos {
                self.playhead.0.set_clip_start(pos.as_samples());
            } else {
                self.playhead.0.set_clip_start(usize::MAX);
            }
        }

        if let Some(pos) = update.clip_end {
            if let Some(pos) = pos {
                self.playhead.0.set_clip_end(pos.as_samples());
            } else {
                self.playhead.0.set_clip_end(usize::MAX);
            }
        }

        if let Some(clipped) = update.clipped {
            self.playhead.0.set_clipped(clipped);
        }

        if let Some(volume) = update.volume {
            self.playhead.0.set_volume(volume);
        }

        if let Some(value) = update.set {
            self.playhead.0.set_cursor(value.as_samples());
        }
    }

    pub fn duration(&self) -> Duration {
        let samples = self.playhead.0.buffer.load().len();
        Duration::from_samples(samples)
    }

    pub fn cursor(&self) -> Duration {
        Duration::from_samples(self.playhead.0.cursor())
    }

    pub fn clip_start(&self) -> Option<Duration> {
        let value = self.playhead.0.clip_start();
        if value == usize::MAX {
            None
        } else {
            Some(Duration::from_samples(value))
        }
    }

    pub fn clip_end(&self) -> Option<Duration> {
        let value = self.playhead.0.clip_end();
        if value == usize::MAX {
            None
        } else {
            Some(Duration::from_samples(value))
        }
    }

    pub fn playhead(&self) -> &Playhead {
        &self.playhead
    }
}

trait SampleConv {
    fn as_samples(&self) -> usize;
    fn from_samples(samples: usize) -> Self;
}

impl SampleConv for Duration {
    fn as_samples(&self) -> usize {
        (self.as_secs_f64() * 48000.0 * 2.0) as usize
    }

    fn from_samples(samples: usize) -> Self {
        Duration::from_secs_f64((samples as f64) / 48000.0 / 2.0)
    }
}

impl Drop for Timeline {
    fn drop(&mut self) {
        self.playhead.0.set_ending()
    }
}

pub struct Update {
    playing: Option<bool>,
    looping: Option<bool>,
    set: Option<Duration>,
    clipped: Option<bool>,
    clip_start: Option<Option<Duration>>,
    clip_end: Option<Option<Duration>>,
    volume: Option<u8>,
}

impl Update {
    pub fn new() -> Self {
        Self {
            playing: None,
            looping: None,
            set: None,
            clipped: None,
            clip_start: None,
            clip_end: None,
            volume: None,
        }
    }
}

impl Update {
    pub fn playing(&mut self, playing: bool) -> &mut Self {
        self.playing = Some(playing);
        self
    }

    pub fn looping(&mut self, looping: bool) -> &mut Self {
        self.looping = Some(looping);
        self
    }

    pub fn clipped(&mut self, clipped: bool) -> &mut Self {
        self.clipped = Some(clipped);
        self
    }

    pub fn set(&mut self, value: Duration) -> &mut Self {
        self.set = Some(value);
        self
    }

    pub fn clip_start(&mut self, start: Option<Duration>) -> &mut Self {
        self.clip_start = Some(start);
        self
    }

    pub fn clip_end(&mut self, end: Option<Duration>) -> &mut Self {
        self.clip_end = Some(end);
        self
    }

    pub fn volume(&mut self, value: u8) -> &mut Self {
        self.volume = Some(value.min(100));
        self
    }
}

#[derive(Debug)]
struct Controls {
    buffer: BufferHandle,
    playing: AtomicBool,
    looping: AtomicBool,
    ending: AtomicBool,
    cursor: AtomicUsize,
    clipped: AtomicBool,
    clip_start: AtomicUsize,
    clip_end: AtomicUsize,
    volume: AtomicU8,
}

impl Controls {
    fn new(buffer: BufferHandle) -> Self {
        Self {
            buffer,
            playing: AtomicBool::new(true),
            looping: AtomicBool::new(false),
            ending: AtomicBool::new(false),
            cursor: AtomicUsize::new(0),
            clipped: AtomicBool::new(false),
            clip_start: AtomicUsize::new(usize::MAX),
            clip_end: AtomicUsize::new(usize::MAX),
            volume: AtomicU8::new(100),
        }
    }

    fn playing(&self) -> bool {
        self.playing.load(Ordering::Relaxed)
    }

    fn set_playing(&self, playing: bool) {
        self.playing.store(playing, Ordering::Relaxed);
    }

    fn looping(&self) -> bool {
        self.looping.load(Ordering::Relaxed)
    }

    fn set_looping(&self, looping: bool) {
        self.looping.store(looping, Ordering::Relaxed);
    }

    fn clipped(&self) -> bool {
        self.clipped.load(Ordering::Relaxed)
    }

    fn set_clipped(&self, clipped: bool) {
        self.clipped.store(clipped, Ordering::Relaxed);
    }

    fn set_ending(&self) {
        self.ending.store(true, Ordering::Relaxed);
    }

    fn cursor(&self) -> usize {
        self.cursor.load(Ordering::Relaxed)
    }

    fn set_cursor(&self, value: usize) {
        self.cursor.store(value, Ordering::Relaxed);
    }

    fn clip_start(&self) -> usize {
        self.clip_start.load(Ordering::Relaxed)
    }

    fn set_clip_start(&self, value: usize) {
        self.clip_start.store(value, Ordering::Relaxed);
    }

    fn clip_end(&self) -> usize {
        self.clip_end.load(Ordering::Relaxed)
    }

    fn set_clip_end(&self, value: usize) {
        self.clip_end.store(value, Ordering::Relaxed);
    }

    fn volume(&self) -> u8 {
        self.volume.load(Ordering::Relaxed)
    }

    fn set_volume(&self, value: u8) {
        self.volume.store(value, Ordering::Relaxed)
    }
}

#[derive(Clone)]
pub struct Playhead(Arc<Controls>);

impl rodio::Source for Playhead {
    fn current_frame_len(&self) -> Option<usize> {
        if self.0.ending.load(Ordering::Relaxed) {
            Some(0)
        } else {
            None
        }
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for Playhead {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let controls = &self.0;

        if !controls.playing() {
            return Some(0.0);
        }

        let cursor = controls.cursor.fetch_add(1, Ordering::Relaxed);
        let buffer = controls.buffer.load();
        let clipped = controls.clipped();

        let buffer = if clipped {
            &buffer[..controls.clip_end().min(buffer.len())]
        } else {
            &buffer[..]
        };

        if let Some(sample) = buffer.get(cursor) {
            let sample = *sample;
            let volume = controls.volume() as f32 / 100.0;
            let sample = sample * volume;
            Some(sample)
        } else {
            let looping = controls.looping();

            if looping || clipped {
                let mut start = 0;
                if clipped {
                    start = controls.clip_start();
                    if start >= buffer.len() {
                        start = 0;
                    }
                }

                controls.cursor.store(start, Ordering::Relaxed);

                if !looping && cursor == buffer.len() {
                    controls.set_playing(false);
                }
            } else {
                controls.set_playing(false);
            }
            Some(0.0)
        }
    }
}
