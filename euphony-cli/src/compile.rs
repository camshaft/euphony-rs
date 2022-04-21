use crate::Result;
use euphony_store::Store;
use std::{fs, io, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Compile {
    #[structopt(long, short)]
    out_dir: Option<PathBuf>,

    input: PathBuf,
}

impl Compile {
    pub fn run(&self) -> Result<()> {
        let mut input: Box<dyn io::Read> = if self.input.to_str() == Some("-") {
            Box::new(io::stdin())
        } else {
            let file = fs::File::open(&self.input)?;
            let file = io::BufReader::new(file);
            Box::new(file)
        };

        let mut compiler = euphony_compiler::Compiler::default();
        let mut store = Store::new();

        if let Some(dir) = self.out_dir.as_ref() {
            store.storage.path = dir.to_owned();
        }

        fs::create_dir_all(&store.storage.path)?;

        compiler.compile(&mut input, &mut store)?;

        // TODO
        /*
        let timeline = root.join(format!("target/euphony/projects/{}.json", name));

        fs::create_dir_all(timeline.parent().unwrap())?;

        let timeline = fs::File::create(timeline)?;
        let mut timeline = io::BufWriter::new(timeline);
        project.store.timeline.to_json(&mut timeline)?;
        io::Write::flush(&mut timeline)?;
        */

        Ok(())
    }
}
