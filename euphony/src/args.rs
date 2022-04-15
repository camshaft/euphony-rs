use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    #[structopt(long, short)]
    pub seed: Option<u64>,

    #[structopt(long, short)]
    pub tempo: Option<u64>,

    #[structopt(long, short)]
    pub output: Option<PathBuf>,
}
