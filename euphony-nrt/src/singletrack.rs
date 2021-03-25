use crate::track::Track;
use core::time::Duration;
use euphony_runtime::time::Handle as Scheduler;
use euphony_sc::{project, track};
use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug)]
pub struct Project {
    db: sled::Db,
    scheduler: Scheduler,
    track: Arc<Track>,
}

impl Project {
    pub fn new(scheduler: Scheduler) -> Self {
        let db = sled::Config::new()
            .temporary(true)
            .open()
            .expect("could not open db");

        let events = db.open_tree(&[0]).unwrap();
        let synths = db.open_tree(&[1]).unwrap();

        let track = Arc::new(Track::new(
            events,
            synths,
            scheduler.clone(),
            "main".to_string(),
        ));

        Self {
            db,
            scheduler,
            track,
        }
    }

    pub fn finish(&self, outdir: &Path) -> BTreeMap<String, io::Result<PathBuf>> {
        let (k, v) = self.track.dump(outdir, Duration::from_secs(10));

        let mut map = BTreeMap::new();
        map.insert(k, v);
        map
    }
}

impl project::Project for Project {
    fn track(&self, _name: &str) -> track::Handle {
        track::Handle(self.track.clone())
    }
}
