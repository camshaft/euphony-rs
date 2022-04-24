use crate::Result;
use std::path::PathBuf;
use structopt::StructOpt;

// TODO read input
// * if local file, compile and watch
// * else if remote file, download, and subscribe
// start player TUI

#[derive(Debug, StructOpt)]
pub struct Play {
    #[allow(dead_code)]
    input: PathBuf,
}

impl Play {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
