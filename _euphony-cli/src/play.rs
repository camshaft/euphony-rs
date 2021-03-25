use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Play {
    #[structopt(short = "p", long = "port")]
    pub port: Option<usize>,
}
