use std::{env::var, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Args {
    // #[structopt(long, short)]
    pub seed: Option<u64>,

    // #[structopt(long, short, env = "EUPHONY_TEMPO")]
    pub tempo: Option<u64>,

    // #[structopt(long, short, env = "EUPHONY_OUTPUT")]
    pub output: Option<PathBuf>,
}

impl Args {
    pub fn from_args() -> Self {
        Self {
            seed: var("EUPHONY_SEED").ok().and_then(|v| v.parse().ok()),
            tempo: var("EUPHONY_SEED").ok().and_then(|v| v.parse().ok()),
            output: var("EUPHONY_OUTPUT").ok().map(PathBuf::from),
        }
    }
}
