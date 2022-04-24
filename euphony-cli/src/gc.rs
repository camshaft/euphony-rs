use crate::Result;
use std::path::PathBuf;
use structopt::StructOpt;

// TODO garbage collect the contents

#[derive(Debug, StructOpt)]
pub struct Gc {
    #[allow(dead_code)]
    input: PathBuf,
}

impl Gc {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
