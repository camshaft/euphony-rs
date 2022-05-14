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
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

#[derive(Debug, Default)]
struct Controls {
    playhead: AtomicUsize,
    is_playing: AtomicBool,
    lower_bound: AtomicUsize,
    upper_bound: AtomicUsize,
    is_looping: AtomicBool,
    sample_rate: u32,
    channels: u16,
    tracks: ArcSwap<Vec<TrackControl>>,
}

#[derive(Debug, Default)]
struct TrackControl {
    name: String,
    hash: Hash,
    is_muted: AtomicBool,
}

pub struct Stream {
    inner: cpal::Stream,
    controls: Arc<Controls>,
}

impl Stream {
    pub fn play(&self) -> Result<()> {
        self.inner.play()?;
        self.controls.is_playing.store(true, Ordering::Relaxed);
        Ok(())
    }

    pub fn set_is_looping(&self, value: bool) {
        self.controls.is_looping.store(value, Ordering::Relaxed);
    }
}

impl Stream {
    pub fn with_manifest(
        device: &Device,
        config: &SupportedStreamConfig,
        manifest: Manifest,
    ) -> Result<Self> {
        let controls = Arc::new(Controls::default());
        let target = manifest.project()?.timeline_path().to_owned();

        macro_rules! build {
            ($(($format:ident, $sample:ty)),* $(,)?) => {
                match config.sample_format() {
                    $(
                        cpal::SampleFormat::$format => {
                            let tracks = Tracks::new(controls.clone());

                            let sample_rate = config.sample_rate().0;

                            if sample_rate != 48_000 {
                                return Err(anyhow!("unsupported sample rate: {}", sample_rate));
                            }

                            match config.channels() {
                                1 => {
                                    let subscriber: Subscriber<1, $sample> = Subscriber {
                                        tracks: tracks.clone(),
                                        target,
                                    };

                                    manifest.watch(subscriber);
                                }
                                2 => {
                                    let subscriber: Subscriber<2, $sample> = Subscriber {
                                        tracks: tracks.clone(),
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
                                    // TODO log
                                    dbg!(err);
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

        let tracks: Vec<_> = store
            .timeline
            .groups
            .par_iter()
            .map(|group| {
                let hash = *group.entries;
                if let Some(prev) = tracks.iter().find(|t| t.hash == hash) {
                    return prev.clone();
                }

                let writer = <TrackWriter<CHANNELS, Sample>>::new(hash);
                let writer = mix(&hash, writer, store);
                let track = writer.track;

                Arc::new(track)
            })
            .collect();

        // TODO preserve state if possible
        let controls = store
            .timeline
            .groups
            .iter()
            .map(|group| TrackControl {
                name: group.name.to_string(),
                hash: *group.entries,
                is_muted: AtomicBool::new(false),
            })
            .collect();

        self.tracks.controls.tracks.swap(Arc::new(controls));
        self.tracks.tracks.swap(Arc::new(tracks));
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
{
    #[inline]
    fn fill(&self, buffer: &mut [Sample]) {
        let playhead = self.controls.playhead.load(Ordering::Relaxed);
        let controls = self.controls.tracks.load();
        let tracks = self.tracks.load();

        let mut len = 0;
        for (track, controls) in tracks.iter().zip(controls.iter()) {
            let adv = if !controls.is_muted.load(Ordering::Relaxed) {
                track.fill(playhead, buffer)
            } else {
                track.advance(playhead, buffer.len())
            };
            len = adv.max(len);
        }

        if len > 0 {
            self.controls.playhead.fetch_add(len, Ordering::Relaxed);
        }

        let full_buffer = len == buffer.len();

        if full_buffer {
            self.controls.is_playing.store(true, Ordering::Relaxed);
        } else if self.controls.is_looping.load(Ordering::Relaxed) {
            let lower = self.controls.lower_bound.load(Ordering::Relaxed);
            self.controls.playhead.store(lower, Ordering::Relaxed);
        } else {
            for sample in &mut buffer[len..] {
                *sample = Sample::EQUILIBRIUM;
            }
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

impl<Sample: Copy> Track<Sample> {
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
            let view = &view[..len];
            let len = view.len();
            buffer[..len].copy_from_slice(view);
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
        if self.track.start == 0 {
            self.track.start = samples as _;
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
