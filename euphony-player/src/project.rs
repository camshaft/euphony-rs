use crate::timeline::Buffer;
use anyhow::Result;
use arc_swap::{ArcSwap, ArcSwapAny};
use lru::LruCache;
use rayon::prelude::*;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufReader, BufWriter},
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
    pub fn new(input: String) -> Result<(Self, BufferHandle, TracksHandle)> {
        let finished = Arc::new(AtomicBool::new(false));
        let state = State::new(input)?;

        let buffer = state.buffer.clone();
        let tracks = state.tracks.clone();

        let worker_finished = finished.clone();

        if state.is_remote {
            std::thread::spawn(move || subscriber::create(state, worker_finished));
        } else {
            std::thread::spawn(move || watcher::create(state, worker_finished));
        }

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
    input: String,
    buffer: BufferHandle,
    tracks: TracksHandle,
    track_handles: HashMap<String, Arc<Track>>,
    mem_cache: LruCache<PathBuf, Buffer>,
    tmp_dir: PathBuf,
    is_remote: bool,
}

impl State {
    fn new(input: String) -> Result<Self> {
        let tmp_dir = std::env::temp_dir().join("euphony-player/cache");

        let (manifest, is_remote) = Self::manifest(&input, &tmp_dir)?;

        let mut mem_cache = LruCache::new(manifest.tracks.len() * 4);

        let mut track_handles = HashMap::new();

        let mut tracks = vec![];
        for (name, track) in manifest.tracks {
            let track = Arc::new(Track::new(name.clone(), track, &mut mem_cache)?);
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
            mem_cache,
            tmp_dir,
            is_remote,
        };

        Ok(state)
    }

    fn reload(&mut self) -> Result<()> {
        let (manifest, _) = Self::manifest(&self.input, &self.tmp_dir)?;

        let mut tracks = vec![];
        for (name, track) in manifest.tracks {
            let track = Track::new(name.clone(), track, &mut self.mem_cache)?;

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

    fn manifest(input: &str, tmp_dir: &Path) -> Result<(manifest::Manifest, bool)> {
        let (path, is_remote) = if input.starts_with("http") {
            let path = tmp_dir.join(input);
            fetch(input, &path)?;
            (path, true)
        } else {
            let path = PathBuf::from(input);
            (path, false)
        };

        let file = File::open(&path)?;
        let file = BufReader::new(file);

        let manifest = if input.ends_with(".json") {
            if is_remote {
                let manifest: manifest::Manifest<String> = serde_json::from_reader(file)?;
                let base = input.trim_end_matches("project.json");
                let tracks = manifest
                    .tracks
                    .par_iter()
                    .map(|(name, track)| {
                        let path = tmp_dir.join(&track.path);
                        if !path.exists() {
                            let mut url = base.to_string();
                            url.push_str(&track.path);
                            fetch(&url, &path)?;
                        }
                        let track = manifest::Track { path };
                        Ok((name.clone(), track))
                    })
                    .collect::<Result<_>>()?;

                manifest::Manifest { tracks }
            } else {
                let mut manifest: manifest::Manifest = serde_json::from_reader(file)?;
                if let Some(parent) = path.parent() {
                    for (_, track) in manifest.tracks.iter_mut() {
                        if track.path.is_relative() {
                            track.path = parent.join(&track.path).to_owned();
                        }
                    }
                }
                manifest
            }
        } else {
            manifest::Manifest {
                tracks: {
                    let mut tracks = BTreeMap::new();
                    let track = manifest::Track { path };
                    tracks.insert("main".to_string(), track);
                    tracks
                },
            }
        };

        Ok((manifest, is_remote))
    }
}

fn fetch(url: &str, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(&parent)?;
    }
    // eprintln!("FETCH {}", url);
    let file = File::create(&path)?;
    let mut buf = BufWriter::new(file);
    reqwest::blocking::get(url)?
        .error_for_status()?
        .copy_to(&mut buf)?;
    Ok(())
}

mod manifest {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct Manifest<Path = PathBuf> {
        pub tracks: BTreeMap<String, Track<Path>>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Track<Path = PathBuf> {
        pub path: Path,
    }
}

mod watcher {
    use super::State;
    use notify::{watcher, RecursiveMode, Watcher};
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
            while rx.recv_timeout(Duration::from_millis(50)).is_ok() {
                // clear the queue
                while rx.try_recv().is_ok() {}

                if let Err(err) = state.reload() {
                    // TODO log
                    let _ = err;
                }
            }

            if finished.load(Ordering::Relaxed) {
                return;
            }

            let _ = state.recompute();
        }
    }
}

mod subscriber {
    use super::State;
    use sse_client::EventSource;
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        time::Duration,
    };
    use url::Url;

    pub(super) fn create(mut state: State, finished: Arc<AtomicBool>) {
        let mut url = Url::parse(&state.input).unwrap();
        url.set_path("_updates");
        let event_source = EventSource::new(url.as_str()).unwrap();
        let rx = event_source.receiver();

        loop {
            while rx.recv_timeout(Duration::from_millis(50)).is_ok() {
                // clear the queue
                while rx.try_recv().is_ok() {}

                if let Err(err) = state.reload() {
                    // TODO log
                    let _ = err;
                }
            }

            if finished.load(Ordering::Relaxed) {
                return;
            }

            let _ = state.recompute();
        }
    }
}
