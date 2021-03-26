use crate::track::Track;
use core::time::Duration;
use euphony_runtime::{output::Output, time::Handle as Scheduler};
use euphony_sc::{project, track};
use std::{collections::BTreeMap, io, path::PathBuf, sync::Arc};

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

    pub fn finish(&self) -> BTreeMap<String, io::Result<PathBuf>> {
        let (k, v) = self.track.dump(&self.output, Duration::from_secs(10));

        let mut map = BTreeMap::new();
        map.insert(k, v);
        map
    }
}

impl Output for Project {
    fn finish(&self) -> io::Result<()> {
        let (_k, v) = self.track.dump(&self.output, Duration::from_secs(5));
        let commands = v?;

        let output = commands.parent().unwrap().join("render.wav");

        if !output.exists() {
            crate::render::Render {
                commands: &commands,
                input: None,
                output: &output,
                channels: 2, // TODO
            }
            .render()?;
        }

        std::fs::copy(&output, self.output.join("main.wav"))?;

        Ok(())
    }
}

impl project::Project for Project {
    fn track(&self, _name: &str) -> track::Handle {
        track::Handle(self.track.clone())
    }
}
