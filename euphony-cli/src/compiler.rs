use crate::Result;
use euphony_store::Store;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Compiler {
    timeline_path: PathBuf,
    compiler: euphony_compiler::Compiler,
    store: Store,
}

impl Compiler {
    pub fn new(contents_dir: PathBuf, timeline: PathBuf) -> Self {
        Self {
            timeline_path: timeline,
            compiler: Default::default(),
            store: Store::new(contents_dir),
        }
    }

    pub fn render<I: io::Read>(&mut self, input: &mut I) -> Result<()> {
        let _ = fs::create_dir_all(self.store.storage.path());
        let _ = fs::create_dir_all(self.timeline_path.parent().unwrap());

        self.store.timeline.reset();
        self.compiler.compile(input, &mut self.store)?;

        let timeline = fs::File::create(&self.timeline_path)?;
        let mut timeline = io::BufWriter::new(timeline);
        self.store.timeline.to_json(&mut timeline)?;
        io::Write::flush(&mut timeline)?;

        Ok(())
    }

    pub fn timeline_path(&self) -> &Path {
        &self.timeline_path
    }

    pub fn store(&self) -> &Store {
        &self.store
    }
}
