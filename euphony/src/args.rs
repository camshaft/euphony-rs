use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    #[structopt(long, short)]
    pub seed: Option<u64>,

    #[structopt(long, short)]
    pub tempo: Option<u64>,

    #[structopt(subcommand)]
    pub subcommand: Option<Cmd>,
}

#[derive(Clone, Debug, StructOpt)]
pub enum Cmd {
    Render(Render),
}

impl Default for Cmd {
    fn default() -> Self {
        Self::Render(Default::default())
    }
}

#[derive(Clone, Debug, Default, StructOpt)]
pub struct Render {
    #[structopt(long, short)]
    pub multitrack: bool,

    #[structopt(long, short)]
    pub out: Option<PathBuf>,
}

impl Render {
    pub fn output(&self) -> PathBuf {
        self.out.clone().unwrap_or_else(Self::default_path)
    }

    pub fn default_path() -> PathBuf {
        PathBuf::new()
            .join("target")
            .join("euphony")
            .join(std::env::current_exe().unwrap().file_name().unwrap())
    }
}
