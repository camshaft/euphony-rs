use crate::track::Track;
use dashmap::DashMap;
use euphony_runtime::{output::Output, time::Handle as Scheduler};
use euphony_sc::{
    project,
    track::{self, Track as _},
};
use lasso::{Key, Spur, ThreadedRodeo};
use rayon::prelude::*;
use std::{
    io,
    path::PathBuf,
    sync::{atomic::AtomicU64, Arc},
};

#[derive(Debug)]
pub struct Project {
    scheduler: Scheduler,
    track_names: ThreadedRodeo<Spur>,
    tracks: DashMap<Spur, Arc<Track>>,
}

impl Project {
    pub fn new(scheduler: Scheduler, output: PathBuf) -> Self {
        Self {
            scheduler,
            track_names: Default::default(),
            tracks: Default::default(),
        }
    }
}

impl Output for Project {
    fn finish(&self) -> io::Result<()> {
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
                let id = key.into_usize() as u64;
                euphony_command::api::set_track_name(id, name);

                Arc::new(Track::new(id, scheduler.clone(), name.to_string()))
            })
            .value()
            .clone();

        track::Handle(track)
    }
}
