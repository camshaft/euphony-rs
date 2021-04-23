use crate::track::Track;
use euphony_runtime::{output::Output, time::Handle as Scheduler};
use euphony_sc::{project, track};
use std::{io, path::PathBuf, sync::Arc};

#[derive(Debug)]
pub struct Project {
    db: sled::Db,
    scheduler: Scheduler,
    track: Arc<Track>,
    output: PathBuf,
}

impl Project {
    pub fn new(scheduler: Scheduler, output: PathBuf) -> Self {
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
            output,
        }
    }
}

impl Output for Project {
    fn finish(&self) -> io::Result<()> {
        let out_file = self.output.join("main.wav");
        self.track.render(&self.output, Some(&out_file))?;
        Ok(())
    }
}

impl project::Project for Project {
    fn track(&self, _name: &str) -> track::Handle {
        track::Handle(self.track.clone())
    }
}
