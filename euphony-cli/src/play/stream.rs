use crate::{manifest::Manifest, watcher, Result};
use anyhow::anyhow;
use arc_swap::ArcSwap;
use cpal::{traits::*, Device, SupportedStreamConfig};
use euphony_compiler::{sample, Hash};
use euphony_mix::{
    frame::{self, Frame as _},
    mono::Mono,
    stereo::Stereo,
};
use euphony_store::Store;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicI16, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

mod midi;

#[derive(Debug)]
struct Controls {
    playhead: AtomicUsize,
    is_playing: AtomicBool,
    is_looping: AtomicBool,
    is_clipped: AtomicBool,
    is_loaded: AtomicBool,
    soloed_count: AtomicUsize,
    clip_start: AtomicUsize,
    clip_end: AtomicUsize,
    total_samples: AtomicUsize,
    volume: AtomicI16,
    sample_rate: u32,
    tracks: ArcSwap<Vec<Arc<TrackControl>>>,
    channels: usize,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            playhead: Default::default(),
            is_playing: Default::default(),
            is_looping: Default::default(),
            is_clipped: Default::default(),
            is_loaded: Default::default(),
            soloed_count: Default::default(),
            clip_start: AtomicUsize::new(usize::MAX),
            clip_end: AtomicUsize::new(usize::MAX),
            total_samples: Default::default(),
            volume: AtomicI16::new(i16::MAX),
            sample_rate: 0,
            tracks: Default::default(),
            channels: 0,
        }
    }
}

impl Controls {
    pub fn playhead(&self) -> usize {
        self.playhead.load(Ordering::Relaxed) / self.channels
    }
}

#[derive(Debug, Default)]
pub struct TrackControl {
    name: String,
    is_muted: AtomicBool,
    is_soloed: AtomicBool,
    end: AtomicUsize,
}

impl TrackControl {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted.load(Ordering::Relaxed)
    }

    pub fn is_soloed(&self) -> bool {
        self.is_soloed.load(Ordering::Relaxed)
    }

    pub fn end(&self) -> usize {
        self.end.load(Ordering::Relaxed)
    }

    fn can_play(&self, any_soloed: bool) -> bool {
        let mut can_play = !self.is_muted();

        if any_soloed {
            can_play &= self.is_soloed();
        }

        can_play
    }
}

pub struct TracksIter {
    guard: arc_swap::Guard<Arc<Vec<Arc<TrackControl>>>>,
}

impl core::ops::Deref for TracksIter {
    type Target = [Arc<TrackControl>];

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

pub struct Stream {
    inner: cpal::Stream,
    controls: Arc<Controls>,
}

impl Stream {
    pub fn play(&self) -> Result<()> {
        self.controls.is_playing.store(true, Ordering::Relaxed);
        self.inner.play()?;
        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        self.controls.is_playing.load(Ordering::Relaxed)
    }

    pub fn is_looping(&self) -> bool {
        self.controls.is_looping.load(Ordering::Relaxed)
    }

    pub fn is_clipped(&self) -> bool {
        self.controls.is_clipped.load(Ordering::Relaxed)
    }

    pub fn is_loaded(&self) -> bool {
        self.controls.is_loaded.load(Ordering::Relaxed)
    }

    pub fn play_toggle(&self) -> Result<()> {
        let prev = self.controls.is_playing.fetch_xor(true, Ordering::Relaxed);
        if prev {
            self.inner.pause()?;
        } else {
            self.inner.play()?;
        }
        Ok(())
    }

    pub fn loop_toggle(&self) {
        self.controls.is_looping.fetch_xor(true, Ordering::Relaxed);
    }

    pub fn clip_toggle(&self) {
        self.controls.is_clipped.fetch_xor(true, Ordering::Relaxed);
    }

    pub fn clip_start(&self) -> Option<Duration> {
        let samples = self.controls.clip_start.load(Ordering::Relaxed);
        if samples == usize::MAX {
            None
        } else {
            Some(self.samples_to_duration(samples))
        }
    }

    pub fn clip_start_set(&self) {
        let now = self.controls.playhead.load(Ordering::Relaxed);
        self.controls.clip_start.store(now, Ordering::Relaxed);
    }

    pub fn clip_start_clear(&self) {
        self.controls
            .clip_start
            .store(usize::MAX, Ordering::Relaxed);
    }

    pub fn clip_end(&self) -> Option<Duration> {
        let samples = self.controls.clip_end.load(Ordering::Relaxed);
        if samples == usize::MAX {
            None
        } else {
            Some(self.samples_to_duration(samples))
        }
    }

    pub fn clip_end_set(&self) {
        let now = self.controls.playhead.load(Ordering::Relaxed);
        self.controls.clip_end.store(now, Ordering::Relaxed);
    }

    pub fn clip_end_clear(&self) {
        self.controls.clip_end.store(usize::MAX, Ordering::Relaxed);
    }

    pub fn seek_back(&self, duration: Duration) {
        let samples = (self.controls.sample_rate as f64 * duration.as_secs_f64()) as usize;
        let end = self.controls.total_samples.load(Ordering::Relaxed);
        let _ = self
            .controls
            .playhead
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |prev| {
                Some(prev.min(end).saturating_sub(samples))
            });
    }

    pub fn seek_forward(&self, duration: Duration) {
        let samples = (self.controls.sample_rate as f64 * duration.as_secs_f64()) as usize;
        let end = self.controls.total_samples.load(Ordering::Relaxed);
        let _ = self
            .controls
            .playhead
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |prev| {
                Some(prev.saturating_add(samples).min(end))
            });
    }

    pub fn seek_start(&self) {
        self.controls.playhead.store(0, Ordering::Relaxed);
    }

    pub fn seek_end(&self) {
        let end = self.controls.total_samples.load(Ordering::Relaxed);
        self.controls.playhead.store(end, Ordering::Relaxed);
    }

    pub fn playhead(&self) -> Duration {
        self.samples_to_duration(self.controls.playhead.load(Ordering::Relaxed))
    }

    pub fn duration(&self) -> Duration {
        self.samples_to_duration(self.controls.total_samples.load(Ordering::Relaxed))
    }

    pub fn volume(&self) -> f32 {
        use sample::Sample;
        self.controls.volume.load(Ordering::Relaxed).to_sample()
    }

    pub fn volume_add(&self, value: f32) {
        use sample::Sample;
        let volume = self.volume() + value;
        let volume: i16 = volume.to_sample();
        let volume = volume.max(0);
        self.controls.volume.store(volume, Ordering::Relaxed);
    }

    pub fn mute_toggle(&self, idx: usize) {
        let tracks = self.controls.tracks.load();
        if let Some(track) = tracks.get(idx) {
            track.is_muted.fetch_xor(true, Ordering::Relaxed);
        }
    }

    pub fn solo_toggle(&self, idx: usize) {
        let tracks = self.controls.tracks.load();
        if let Some(track) = tracks.get(idx) {
            let prev = track.is_soloed.fetch_xor(true, Ordering::Relaxed);
            if prev {
                self.controls.soloed_count.fetch_sub(1, Ordering::Relaxed);
            } else {
                self.controls.soloed_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn tracks(&self) -> TracksIter {
        let tracks = self.controls.tracks.load();
        TracksIter { guard: tracks }
    }

    fn samples_to_duration(&self, samples: usize) -> Duration {
        let secs =
            samples as f64 / self.controls.channels as f64 / self.controls.sample_rate as f64;
        Duration::from_secs_f64(secs)
    }
}

impl Stream {
    pub fn with_manifest(
        device: &Device,
        config: &SupportedStreamConfig,
        manifest: Manifest,
    ) -> Result<Self> {
        let controls;
        let target = manifest.project()?.timeline_path().to_owned();

        macro_rules! build {
            ($(($format:ident, $sample:ty)),* $(,)?) => {
                match config.sample_format() {
                    $(
                        cpal::SampleFormat::$format => {
                            let sample_rate = config.sample_rate().0;

                            if sample_rate != 48_000 {
                                return Err(anyhow!("unsupported sample rate: {}", sample_rate));
                            }

                            controls = Arc::new(Controls {
                                sample_rate: 48_000,
                                channels: config.channels() as _,
                                ..Default::default()
                            });

                            let tracks = Tracks::new(controls.clone());

                            match config.channels() {
                                1 => {
                                    let subscriber: Subscriber<1, $sample> = Subscriber {
                                        tracks: tracks.clone(),
                                        midi_output: midi::Output::new(controls.clone()),
                                        midi_groups: vec![],
                                        target,
                                    };

                                    manifest.watch(subscriber);
                                }
                                2 => {
                                    let subscriber: Subscriber<2, $sample> = Subscriber {
                                        tracks: tracks.clone(),
                                        midi_output: midi::Output::new(controls.clone()),
                                        midi_groups: vec![],
                                        target,
                                    };

                                    manifest.watch(subscriber);
                                }
                                channels => {
                                    // TODO support this
                                    return Err(anyhow!("unsupported channel count: {}", channels));
                                }
                            }

                            device.build_output_stream(
                                &config.config(),
                                move |buffer: &mut [$sample], _info| {
                                    tracks.fill(buffer)
                                },
                                |err| {
                                    log::error!("could not open output: {}", err);
                                }
                            )?
                        }
                    )*
                }
            }
        }

        let inner = build!((F32, f32), (I16, i16), (U16, u16));

        Ok(Self { inner, controls })
    }
}

pub struct Subscriber<const CHANNELS: usize, Sample: Copy> {
    tracks: Tracks<Sample>,
    midi_output: midi::Output,
    midi_groups: Vec<Option<midi::Group>>,
    target: PathBuf,
}

impl<Sample> watcher::Subscriptions<Manifest> for Subscriber<1, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64> + Send + Sync,
{
    fn on_update(&mut self, updates: &mut HashSet<PathBuf>, manifest: &mut Manifest) {
        let store = manifest.project().unwrap().store();

        self.update(updates, store, |hash, writer, store| {
            let mut mixer = Mono::new(writer);

            if let Err(err) = store.mix_group(hash, &mut mixer) {
                // TODO log
                dbg!(err);
            }

            mixer.finish()
        });
    }
}

impl<Sample> watcher::Subscriptions<Manifest> for Subscriber<2, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64> + Send + Sync,
{
    fn on_update(&mut self, updates: &mut HashSet<PathBuf>, manifest: &mut Manifest) {
        let store = manifest.project().unwrap().store();

        self.update(updates, store, |hash, writer, store| {
            let mut mixer = Stereo::new(writer);

            if let Err(err) = store.mix_group(hash, &mut mixer) {
                // TODO log
                dbg!(err);
            }

            mixer.finish()
        });
    }
}

impl<const CHANNELS: usize, Sample> Subscriber<CHANNELS, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64> + Send + Sync,
{
    fn update<F>(&mut self, updates: &mut HashSet<PathBuf>, store: &Store, mix: F)
    where
        F: Fn(&Hash, TrackWriter<CHANNELS, Sample>, &Store) -> TrackWriter<CHANNELS, Sample> + Sync,
    {
        if !updates.contains(&self.target) {
            return;
        }

        let tracks = self.tracks.tracks.load();
        let controls = self.tracks.controls.tracks.load();
        let midi_groups = &self.midi_groups;

        let (tracks, (controls, midi_groups)): (Vec<_>, (Vec<_>, Vec<_>)) = store
            .timeline
            .groups
            .par_iter()
            .map(|group| {
                let hash = *group.entries;

                let (controls, idx) = if let Some((idx, control)) = controls
                    .iter()
                    .enumerate()
                    .find(|(_, track)| track.name == group.name)
                {
                    (control.clone(), Some(idx))
                } else {
                    let control = Arc::new(TrackControl {
                        name: group.name.to_string(),
                        is_muted: AtomicBool::new(false),
                        is_soloed: AtomicBool::new(false),
                        end: AtomicUsize::new(0),
                    });
                    (control, None)
                };

                let track = if let Some(track) =
                    idx.and_then(|idx| tracks.get(idx).filter(|track| track.hash == hash))
                {
                    track.clone()
                } else {
                    let writer = <TrackWriter<CHANNELS, Sample>>::new(hash);
                    let writer = mix(&hash, writer, store);
                    let track = writer.track;

                    Arc::new(track)
                };

                let midi = midi::Group::read(group.midi.as_deref(), &store.storage, midi_groups);

                let track_end = track.start + track.buffer.len();
                let midi_end = midi.as_ref().map(|m| m.end()).unwrap_or_default() as usize;
                let end = track_end.max(midi_end * CHANNELS);

                controls.end.store(end, Ordering::Relaxed);

                (track, (controls, midi))
            })
            .collect();

        let mut soloed_count = 0;
        let mut total_samples = 0;

        for control in controls.iter() {
            if control.is_soloed.load(Ordering::Relaxed) {
                soloed_count += 1;
            }
            total_samples = total_samples.max(control.end());
        }

        self.tracks
            .controls
            .total_samples
            .store(total_samples, Ordering::Relaxed);
        self.tracks
            .controls
            .soloed_count
            .store(soloed_count, Ordering::Relaxed);

        self.midi_groups = midi_groups;
        self.midi_output.update(&controls, &self.midi_groups);

        self.tracks.controls.tracks.swap(Arc::new(controls));
        self.tracks.tracks.swap(Arc::new(tracks));

        let was_loaded = self.tracks.controls.is_loaded.load(Ordering::Relaxed);

        // give the MIDI connections time to register
        if !was_loaded {
            std::thread::sleep(core::time::Duration::from_millis(500));
        }

        self.tracks
            .controls
            .is_loaded
            .store(true, Ordering::Relaxed);
    }
}

type TrackList<Sample> = Arc<ArcSwap<Vec<Arc<Track<Sample>>>>>;

#[derive(Clone, Debug)]
struct Tracks<Sample: Copy> {
    tracks: TrackList<Sample>,
    controls: Arc<Controls>,
}

impl<Sample: Copy> Tracks<Sample> {
    pub fn new(controls: Arc<Controls>) -> Self {
        Self {
            tracks: Default::default(),
            controls,
        }
    }
}

impl<Sample> Tracks<Sample>
where
    Sample: sample::Sample,
    Sample::Float: sample::FromSample<i16>,
{
    #[inline]
    fn fill(&self, buffer: &mut [Sample]) {
        // make sure the whole thing is clear first
        for sample in buffer.iter_mut() {
            *sample = Sample::EQUILIBRIUM;
        }

        if !self.controls.is_playing.load(Ordering::Relaxed)
            || !self.controls.is_loaded.load(Ordering::Relaxed)
        {
            return;
        }

        let playhead = self.controls.playhead.load(Ordering::Relaxed);
        let controls = self.controls.tracks.load();
        let is_clipped = self.controls.is_clipped.load(Ordering::Relaxed);
        let is_soloed = self.controls.soloed_count.load(Ordering::Relaxed) > 0;
        let tracks = self.tracks.load();

        let mut len = 0;
        {
            let buffer = if is_clipped {
                // we don't need to check for max here since it'll just play like normal
                let end = self.controls.clip_end.load(Ordering::Relaxed);
                let remaining = end.saturating_sub(playhead);
                let len = remaining.min(buffer.len());
                &mut buffer[..len]
            } else {
                &mut buffer[..]
            };

            for (track, controls) in tracks.iter().zip(controls.iter()) {
                let adv = if controls.can_play(is_soloed) {
                    track.fill(playhead, buffer)
                } else {
                    track.advance(playhead, buffer.len())
                };
                len = adv.max(len);
            }
        }

        if len > 0 {
            self.controls.playhead.fetch_add(len, Ordering::Relaxed);

            let volume = self.controls.volume.load(Ordering::Relaxed);

            if volume != i16::MAX {
                use sample::Sample;

                for sample in buffer[..len].iter_mut() {
                    *sample = (*sample).mul_amp(volume.to_sample());
                }
            }
        }

        let full_buffer = len == buffer.len();

        if full_buffer {
            self.controls.is_playing.store(true, Ordering::Relaxed);
        } else if self.controls.is_looping.load(Ordering::Relaxed) {
            let lower = if is_clipped {
                let v = self.controls.clip_start.load(Ordering::Relaxed);
                if v == usize::MAX {
                    0
                } else {
                    v
                }
            } else {
                0
            };
            self.controls.playhead.store(lower, Ordering::Relaxed);
        } else {
            self.controls.is_playing.store(false, Ordering::Relaxed);
        }
    }
}

#[derive(Debug)]
struct Track<Sample> {
    hash: Hash,
    buffer: Vec<Sample>,
    start: usize,
}

impl<Sample> Track<Sample> {
    pub fn new(hash: Hash) -> Self {
        Self {
            hash,
            buffer: vec![],
            start: 0,
        }
    }
}

impl<Sample> Track<Sample>
where
    Sample: sample::Sample,
{
    #[inline]
    fn fill(&self, playhead: usize, buffer: &mut [Sample]) -> usize {
        let playhead = if let Some(p) = playhead.checked_sub(self.start) {
            p
        } else {
            return buffer.len();
        };

        let view = &self.buffer;
        if let Some(view) = view.get(playhead..) {
            let len = buffer.len().min(view.len());
            for (from, to) in buffer[..len].iter_mut().zip(&view[..len]) {
                *from = from.add_amp(to.to_signed_sample());
            }
            len
        } else {
            0
        }
    }

    #[inline]
    fn advance(&self, playhead: usize, buffer_len: usize) -> usize {
        let playhead = if let Some(p) = playhead.checked_sub(self.start) {
            p
        } else {
            return buffer_len;
        };

        let view = &self.buffer;
        if let Some(view) = view.get(playhead..) {
            view.len().min(buffer_len)
        } else {
            0
        }
    }
}

struct TrackWriter<const CHANNELS: usize, Sample: Copy> {
    track: Track<Sample>,
}

impl<const CHANNELS: usize, Sample: euphony_compiler::sample::Sample>
    TrackWriter<CHANNELS, Sample>
{
    #[inline]
    fn new(hash: Hash) -> Self {
        Self {
            track: Track::new(hash),
        }
    }
}

impl<const CHANNELS: usize, Sample: euphony_compiler::sample::Sample> euphony_mix::Writer
    for TrackWriter<CHANNELS, Sample>
where
    Sample: sample::Sample + sample::FromSample<f64>,
    [Sample; CHANNELS]: frame::Frame<Sample = Sample>,
{
    type Error = std::io::Error;
    type Sample = Sample;
    type Frame = [Sample; CHANNELS];

    #[inline]
    fn skip(&mut self, frames: usize) -> Result<(), Self::Error> {
        let samples = CHANNELS * frames;
        if self.track.buffer.is_empty() {
            self.track.start += samples;
        } else {
            let new_len = self.track.buffer.len() + samples;
            self.track.buffer.resize(new_len, Sample::EQUILIBRIUM);
        }

        Ok(())
    }

    #[inline]
    fn write(&mut self, frame: Self::Frame) -> Result<(), Self::Error> {
        for sample in frame.channels() {
            self.track.buffer.push(sample);
        }

        Ok(())
    }
}
