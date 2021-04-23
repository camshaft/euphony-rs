use crate::project::BufferHandle;
use core::time::Duration;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

pub type Sample = f32;
pub type Buffer = Arc<Vec<Sample>>;

pub struct Timeline {
    start: usize,
    end: usize,
    looping: bool,
    playing: bool,
    playhead: Playhead,
}

impl Timeline {
    pub fn new(buffer: BufferHandle) -> Self {
        let playhead = Playhead(Arc::new(Controls::new(buffer)));

        Self {
            playhead,
            looping: false,
            playing: true,
            start: 0,
            end: 2,
        }
    }

    pub fn playing(&self) -> bool {
        self.playing
    }

    pub fn looping(&self) -> bool {
        self.looping
    }

    pub fn update(&mut self, update: Update) {
        if let Some(playing) = update.playing {
            self.playhead.0.set_playing(playing);
            self.playing = playing;
        }

        if let Some(looping) = update.looping {
            self.playhead.0.set_looping(looping);
            self.looping = looping;
        }

        if let Some(value) = update.set {
            let cursor = value.as_secs_f64() * 48000.0 * 2.0;
            self.playhead.0.set_cursor(cursor as usize);
        }

        // TODO clip_start / clip_end
    }

    pub fn duration(&self) -> Duration {
        let samples = self.playhead.0.buffer.load().len() as f64;
        let offset = samples / 48000.0 / 2.0;
        Duration::from_secs_f64(offset)
    }

    pub fn cursor(&self) -> Duration {
        let samples = self.playhead.0.cursor.load(Ordering::Relaxed) as f64;
        let offset = samples / 48000.0 / 2.0;
        Duration::from_secs_f64(offset)
    }

    pub fn progress(&self) -> f64 {
        let total = self.playhead.0.buffer.load().len() as f64;
        let cursor = self.playhead.0.cursor.load(Ordering::Relaxed) as f64;
        cursor.min(total) / total
    }

    pub fn playhead(&self) -> &Playhead {
        &self.playhead
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
    clip_start: Option<Duration>,
    clip_end: Option<Duration>,
}

impl Update {
    pub fn new() -> Self {
        Self {
            playing: None,
            looping: None,
            set: None,
            clip_start: None,
            clip_end: None,
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

    pub fn set(&mut self, value: Duration) -> &mut Self {
        self.set = Some(value);
        self
    }

    pub fn clip_start(&mut self, start: Duration) -> &mut Self {
        self.clip_start = Some(start);
        self
    }

    pub fn clip_end(&mut self, end: Duration) -> &mut Self {
        self.clip_end = Some(end);
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
    clip_start: AtomicUsize,
    clip_end: AtomicUsize,
}

impl Controls {
    fn new(buffer: BufferHandle) -> Self {
        let len = buffer.load().len();

        Self {
            clip_end: AtomicUsize::new(len),
            buffer,
            playing: AtomicBool::new(true),
            looping: AtomicBool::new(false),
            ending: AtomicBool::new(false),
            cursor: AtomicUsize::new(0),
            clip_start: AtomicUsize::new(0),
        }
    }

    fn set_playing(&self, playing: bool) {
        self.playing.store(playing, Ordering::Relaxed);
    }

    fn set_looping(&self, looping: bool) {
        self.looping.store(looping, Ordering::Relaxed);
    }

    fn set_ending(&self) {
        self.ending.store(true, Ordering::Relaxed);
    }

    fn set_cursor(&self, value: usize) {
        self.cursor.store(value, Ordering::Relaxed);
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

        if !controls.playing.load(Ordering::Relaxed) {
            return Some(0.0);
        }

        let cursor = controls.cursor.fetch_add(1, Ordering::Relaxed);
        let buffer = controls.buffer.load();

        if let Some(sample) = buffer.get(cursor) {
            Some(*sample)
        } else {
            if controls.looping.load(Ordering::Relaxed) {
                controls.cursor.store(0, Ordering::Relaxed);
            } else {
                controls.playing.store(false, Ordering::Relaxed);
            }
            Some(0.0)
        }
    }
}
