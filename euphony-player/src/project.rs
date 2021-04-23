use crate::timeline::Buffer;
use anyhow::Result;
use arc_swap::{ArcSwap, ArcSwapAny};
use lru::LruCache;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub type BufferHandle = Arc<ArcSwapAny<Buffer>>;
pub type TracksHandle = Arc<ArcSwap<Tracks>>;

pub struct Project {
    finished: Arc<AtomicBool>,
}

impl Project {
    pub fn new(input: PathBuf) -> Result<(Self, BufferHandle, TracksHandle)> {
        let finished = Arc::new(AtomicBool::new(false));
        let state = State::new(input)?;

        let buffer = state.buffer.clone();
        let tracks = state.tracks.clone();

        let worker_finished = finished.clone();
        std::thread::spawn(move || worker::create(state, worker_finished));

        Ok((Self { finished }, buffer, tracks))
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst);
    }
}

pub struct Tracks {
    tracks: Vec<Arc<Track>>,
    needs_update: AtomicBool,
}

impl Tracks {
    fn new(tracks: Vec<Arc<Track>>) -> Self {
        Self {
            tracks,
            needs_update: AtomicBool::new(false),
        }
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn mute(&self, idx: usize) {
        if let Some(track) = self.tracks.get(idx) {
            track.mute();
            self.on_change();
        }
    }

    pub fn mute_all(&self) {
        for track in &self.tracks {
            track.mute();
        }
        self.on_change();
    }

    pub fn solo(&self, idx: usize) {
        if let Some(track) = self.tracks.get(idx) {
            track.solo();
            self.on_change();
        }
    }

    pub fn solo_all(&self) {
        for track in &self.tracks {
            track.solo();
        }
        self.on_change();
    }

    pub fn tracks(&self) -> &[Arc<Track>] {
        &self.tracks
    }

    fn buffer(&self) -> Buffer {
        let mut out = vec![];

        let mut any_solo = false;

        for track in self.tracks.iter() {
            if out.len() < track.buffer.len() {
                out.resize(track.buffer.len(), 0.0);
            }
            any_solo |= track.is_solo();
        }

        for track in self.tracks.iter() {
            if !track.is_muted() && (any_solo == track.is_solo()) {
                for (a, b) in out.iter_mut().zip(track.buffer.iter().copied()) {
                    *a += b;
                }
            }
        }

        Arc::new(out)
    }

    fn on_change(&self) {
        self.needs_update.store(true, Ordering::Relaxed);
    }
}

pub struct Track {
    name: String,
    mute: AtomicBool,
    solo: AtomicBool,
    buffer: Buffer,
}

impl Track {
    fn new(
        name: String,
        track: manifest::Track,
        cache: &mut LruCache<PathBuf, Buffer>,
    ) -> Result<Self> {
        let path = track.path.canonicalize()?;
        let buffer = if let Some(buffer) = cache.get(&path) {
            buffer.clone()
        } else {
            open(&path)?
        };

        Ok(Self {
            name,
            mute: AtomicBool::new(false),
            solo: AtomicBool::new(false),
            buffer,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn mute(&self) -> bool {
        self.mute.fetch_xor(true, Ordering::Relaxed)
    }

    pub fn is_muted(&self) -> bool {
        self.mute.load(Ordering::Relaxed)
    }

    fn solo(&self) -> bool {
        self.solo.fetch_xor(true, Ordering::Relaxed)
    }

    pub fn is_solo(&self) -> bool {
        self.solo.load(Ordering::Relaxed)
    }
}

fn open(path: &Path) -> Result<Buffer> {
    use rodio::{Decoder, Source};

    let file = File::open(path)?;
    let file = BufReader::new(file);
    let source = Decoder::new(file)?;
    // TODO turn into error
    assert_eq!(source.channels(), 2);
    assert_eq!(source.sample_rate(), 48000);
    let source = source.convert_samples();
    let out = Arc::new(source.collect());
    Ok(out)
}

struct State {
    input: PathBuf,
    buffer: BufferHandle,
    tracks: TracksHandle,
    track_handles: HashMap<String, Arc<Track>>,
    cache: LruCache<PathBuf, Buffer>,
}

impl State {
    fn new(input: PathBuf) -> Result<Self> {
        let manifest = Self::manifest(&input)?;

        let mut cache = LruCache::new(manifest.tracks.len() * 4);

        let mut track_handles = HashMap::new();

        let mut tracks = vec![];
        for (name, track) in manifest.tracks {
            let track = Arc::new(Track::new(name.clone(), track, &mut cache)?);
            track_handles.insert(name.clone(), track.clone());
            tracks.push(track);
        }

        let tracks = Tracks::new(tracks);
        let buffer = tracks.buffer();

        let tracks = Arc::new(ArcSwap::new(Arc::new(tracks)));
        let buffer = Arc::new(ArcSwap::new(buffer));

        let state = Self {
            input,
            buffer,
            tracks,
            track_handles,
            cache,
        };

        Ok(state)
    }

    fn reload(&mut self) -> Result<()> {
        let manifest = Self::manifest(&self.input)?;

        let mut tracks = vec![];
        for (name, track) in manifest.tracks {
            let track = Track::new(name.clone(), track, &mut self.cache)?;

            // copy the old settings
            if let Some(old_track) = self.track_handles.get(&name) {
                track.solo.store(old_track.is_solo(), Ordering::SeqCst);
                track.mute.store(old_track.is_muted(), Ordering::SeqCst);
            }

            let track = Arc::new(track);

            tracks.push(track.clone());
            self.track_handles.insert(name, track);
        }

        self.track_handles
            .retain(|_, track| Arc::strong_count(track) > 1);

        let tracks = Tracks::new(tracks);
        let buffer = tracks.buffer();

        self.tracks.swap(Arc::new(tracks));
        self.buffer.swap(buffer);

        Ok(())
    }

    fn recompute(&mut self) -> Result<()> {
        let tracks = self.tracks.load();
        if tracks.needs_update.swap(false, Ordering::Relaxed) {
            let buffer = tracks.buffer();
            self.buffer.swap(buffer);
        }

        Ok(())
    }

    fn manifest(input: &Path) -> Result<manifest::Manifest> {
        let file = File::open(input)?;
        let file = BufReader::new(file);

        let manifest = if input.extension().map_or(false, |ext| ext == "json") {
            serde_json::from_reader(file)?
        } else {
            manifest::Manifest {
                tracks: {
                    let mut tracks = BTreeMap::new();
                    let track = manifest::Track {
                        path: input.to_owned(),
                    };
                    tracks.insert("main".to_string(), track);
                    tracks
                },
            }
        };

        Ok(manifest)
    }
}

mod manifest {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Debug, Default, Deserialize)]
    pub struct Manifest {
        pub tracks: BTreeMap<String, Track>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Track {
        pub path: PathBuf,
    }
}

mod worker {
    use super::State;
    use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::channel,
            Arc,
        },
        time::Duration,
    };

    pub(super) fn create(mut state: State, finished: Arc<AtomicBool>) {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

        watcher
            .watch(&state.input, RecursiveMode::NonRecursive)
            .unwrap();

        loop {
            while let Ok(event) = rx.recv_timeout(Duration::from_millis(50)) {
                match event {
                    DebouncedEvent::Write(_) | DebouncedEvent::Chmod(_) => {
                        if let Err(err) = state.reload() {
                            // TODO log
                            let _ = err;
                        }
                    }
                    _ => {}
                }
            }

            if finished.load(Ordering::Relaxed) {
                return;
            }

            let _ = state.recompute();
        }
    }
}
