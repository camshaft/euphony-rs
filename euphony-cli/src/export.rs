use crate::Result;
use std::path::PathBuf;
use structopt::StructOpt;

// TODO export to FCMXL, Logic Pro X, etc

#[derive(Debug, StructOpt)]
pub struct Export {
    #[allow(dead_code)]
    input: PathBuf,
}

impl Export {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
