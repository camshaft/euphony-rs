use crate::track::Track;
use dashmap::DashMap;
use euphony_runtime::{output::Output, time::Handle as Scheduler};
use euphony_sc::{
    project,
    track::{self, Track as _},
};
use lasso::{Key, Spur, ThreadedRodeo};
use rayon::prelude::*;
use std::{io, path::PathBuf, sync::Arc};

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
}

impl Output for Project {
    fn finish(&self) -> io::Result<()> {
        let tracks = self
            .tracks
            .par_iter()
            .map(|track| {
                let name = track.name().to_owned();
                let path = track.value().render(&self.output, None)?;
                let path = path.strip_prefix(&self.output).unwrap().to_owned();
                let track = crate::manifest::Track { path };
                Ok((name, track))
            })
            .collect::<Result<_, io::Error>>()?;

        let manifest = crate::manifest::Manifest { tracks };

        let out = self.output.join("project.json");
        let out = std::fs::File::create(out)?;
        let out = io::BufWriter::new(out);
        serde_json::to_writer(out, &manifest)?;

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
