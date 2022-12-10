use super::{Controls, TrackControl};
use arc_swap::ArcSwap;
use euphony_compiler::{midi, Hash};
use euphony_store::storage::Storage;
use midir::{os::unix::VirtualOutput, MidiOutput, MidiOutputConnection as Connection};
use std::{
    cell::UnsafeCell,
    collections::{hash_map, BTreeMap, HashMap},
    ops::Range,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

const FPS: u64 = 500;

#[derive(Clone)]
pub struct Group {
    events: Arc<[(Sample, [u8; 3])]>,
    hash: Hash,
}

impl Group {
    pub fn read<S: Storage>(
        hash: Option<&Hash>,
        storage: &S,
        prev: &[Option<Self>],
    ) -> Option<Self> {
        let hash = *hash?;

        if let Some(g) = prev
            .iter()
            .find(|g| g.as_ref().map_or(false, |g| g.hash == hash))
        {
            return g.clone();
        }

        let group = storage.open_raw(&hash).ok()?;
        let reader = midi::Reader::new(group);

        let mut events = vec![];

        for event in reader {
            let (sample, _beat, data) = event.ok()?;
            let sample: u64 = sample.into();
            let sample = sample / (48_000 / FPS);
            events.push((sample, data));
        }

        let events = events.into();

        Some(Self { events, hash })
    }

    pub fn end(&self) -> Sample {
        self.events.last().map_or(0, |e| e.0)
    }
}

pub struct Output {
    indexes: HashMap<String, usize>,
    state: Arc<ArcSwap<State>>,
    #[allow(dead_code)] // used to notify the work thread
    handle: Handle,
}

impl Output {
    pub(super) fn new(controls: Arc<Controls>) -> Self {
        let state = Arc::new(ArcSwap::default());
        let handle = Handle::default();

        let w_state = state.clone();
        let w_handle = handle.clone();
        std::thread::spawn(move || worker(w_state, w_handle, controls));

        Self {
            indexes: Default::default(),
            state,
            handle,
        }
    }

    pub fn update(&mut self, new_controls: &[Arc<TrackControl>], groups: &[Option<Group>]) {
        let s = self.state.load();

        let mut events: BTreeMap<u64, Vec<TrackEvents>> = Default::default();
        let mut connections = s.connections.clone();

        for (track, (control, group)) in new_controls.iter().zip(groups).enumerate() {
            if let Some(group) = group {
                let index = match self.indexes.entry(control.name.clone()) {
                    hash_map::Entry::Occupied(e) => Some(*e.get()),
                    hash_map::Entry::Vacant(e) => {
                        let idx = connections.len();
                        if let Some(connection) = MidiOutput::new("euphony").ok().and_then(|o| {
                            let name = format!("euphony ({})", control.name);
                            o.create_virtual(&name).ok()
                        }) {
                            connections.push(Arc::new(connection.into()));
                            Some(*e.insert(idx))
                        } else {
                            None
                        }
                    }
                };

                if let Some(connection) = index {
                    let mut buffer = vec![];
                    let mut prev_sample = 0;
                    for (sample, data) in group.events.iter().copied() {
                        if prev_sample == sample {
                            buffer.push(data);
                            continue;
                        }

                        let pending = core::mem::take(&mut buffer);
                        if !pending.is_empty() {
                            events.entry(prev_sample).or_default().push(TrackEvents {
                                track,
                                connection,
                                events: pending,
                            });
                        }

                        buffer.push(data);
                        prev_sample = sample;
                    }
                }
            }
        }

        let state = Arc::new(State {
            events,
            connections,
        });
        self.state.swap(state);
    }
}

#[derive(Clone, Default)]
struct Handle {
    closed: Arc<AtomicBool>,
}

impl Handle {
    fn is_open(&self) -> bool {
        !self.closed.load(Ordering::Relaxed)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.closed.store(true, Ordering::Relaxed);
    }
}

type Sample = u64;

#[derive(Default)]
pub struct State {
    connections: Vec<Arc<Track>>,
    events: BTreeMap<Sample, Vec<TrackEvents>>,
}

struct Track {
    connection: UnsafeCell<Connection>,
}

// tracks are only read in the real-time thread
unsafe impl Send for Track {}
unsafe impl Sync for Track {}

impl Track {
    fn send(&self, message: &[u8]) {
        if let Err(err) = unsafe { &mut *self.connection.get() }.send(message) {
            log::error!("{}", err)
        }
    }
}

impl From<Connection> for Track {
    fn from(conn: Connection) -> Self {
        Self {
            connection: UnsafeCell::new(conn),
        }
    }
}

struct TrackEvents {
    track: usize,
    connection: usize,
    events: Vec<[u8; 3]>,
}

fn worker(state: Arc<ArcSwap<State>>, handle: Handle, controls: Arc<Controls>) {
    let mut looper = spin_sleep::LoopHelper::builder().build_with_target_rate(FPS as f64);

    let sample_scale = controls.sample_rate as u64 / FPS;

    let mut prev = u64::MAX;

    while handle.is_open() {
        looper.loop_start();

        if !controls.is_loaded.load(Ordering::Relaxed)
            || !controls.is_playing.load(Ordering::Relaxed)
        {
            looper.loop_sleep();
            continue;
        }

        let s = state.load();

        let playhead = controls.playhead() as u64 / sample_scale;

        if prev == playhead {
            looper.loop_sleep();
            continue;
        }

        let is_soloed = controls.soloed_count.load(Ordering::Relaxed) > 0;
        let track_controls = controls.tracks.load();

        let reset = || {
            for connection in s.connections.iter() {
                // reset the midi state
                connection.send(&[0xFF]);
            }
        };

        let perform_range = |range: Range<u64>| {
            for (_idx, events) in s.events.range(range) {
                for track_events in events {
                    // check if we can play
                    if !track_controls
                        .get(track_events.track)
                        .map_or(false, |track| track.can_play(is_soloed))
                    {
                        continue;
                    }

                    if let Some(track) = s.connections.get(track_events.connection) {
                        for message in &track_events.events {
                            track.send(message);
                        }
                    }
                }
            }
        };

        if prev == u64::MAX {
            reset();
            perform_range(0..playhead);
        } else if prev < playhead {
            perform_range(prev..playhead);
        } else {
            reset();
            perform_range(prev..prev.saturating_add(sample_scale));
        }

        prev = playhead;

        looper.loop_sleep();
    }
}
