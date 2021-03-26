use crate::track::Track;
use core::time::Duration;
use dashmap::DashMap;
use euphony_runtime::{output::Output, time::Handle as Scheduler};
use euphony_sc::{project, track};
use lasso::{Key, Spur, ThreadedRodeo};
use rayon::prelude::*;
use std::{collections::BTreeMap, io, path::PathBuf, sync::Arc};

#[derive(Debug)]
pub struct Project {
    db: sled::Db,
    scheduler: Scheduler,
    track_names: ThreadedRodeo<Spur>,
    tracks: DashMap<Spur, Arc<Track>>,
    output: PathBuf,
}

impl Project {
    pub fn new(scheduler: Scheduler, output: PathBuf) -> Self {
        Self {
            db: sled::Config::new()
                .temporary(true)
                .open()
                .expect("could not open db"),
            scheduler,
            track_names: Default::default(),
            tracks: Default::default(),
            output,
        }
    }

    pub fn finish(&self) -> BTreeMap<String, io::Result<PathBuf>> {
        self.tracks
            .par_iter()
            .map(|entry| {
                let track = entry.value();
                track.dump(&self.output, Duration::from_secs(5))
            })
            .collect()
    }
}

impl Output for Project {
    fn finish(&self) -> io::Result<()> {
        // TODO
        Ok(())
    }
}

impl project::Project for Project {
    fn track(&self, name: &str) -> track::Handle {
        let db = &self.db;
        let key = self.track_names.get_or_intern(name);
        let scheduler = &self.scheduler;

        let track = self
            .tracks
            .entry(key)
            .or_insert_with(|| {
                let id = (key.into_usize() as u32).to_be_bytes();

                let mut events = [0u8; 5];
                events[..4].copy_from_slice(&id);
                let events = db.open_tree(events).unwrap();

                let mut synths = [1u8; 5];
                synths[..4].copy_from_slice(&id);
                let synths = db.open_tree(synths).unwrap();

                Arc::new(Track::new(
                    events,
                    synths,
                    scheduler.clone(),
                    name.to_string(),
                ))
            })
            .value()
            .clone();

        track::Handle(track)
    }
}
