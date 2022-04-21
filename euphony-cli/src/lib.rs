use anyhow::Result;
use structopt::StructOpt;

mod compile;
mod disasm;
mod manifest;
mod publish;
mod watcher;

#[cfg(feature = "remote")]
mod serve;

#[derive(Debug, StructOpt)]
enum Arguments {
    Compile(compile::Compile),
    Disasm(disasm::Disasm),
    Publish(publish::Publish),
    #[cfg(feature = "remote")]
    Serve(serve::Serve),
}

pub fn main() {
    let args = Arguments::from_args();
    match args {
        Arguments::Compile(args) => args.run(),
        Arguments::Disasm(args) => args.run(),
        Arguments::Publish(args) => args.run(),
        #[cfg(feature = "remote")]
        Arguments::Serve(args) => args.run(),
    }
    // TODO better error message
    .unwrap()
}
