use anyhow::Result;
use structopt::StructOpt;

mod build;
mod compiler;
mod disasm;
mod export;
mod gc;
mod manifest;
mod play;
mod render;
mod watcher;
mod workspace;

#[cfg(feature = "remote")]
mod serve;

#[derive(Debug, StructOpt)]
enum Arguments {
    Build(build::Build),
    #[structopt(alias = "p")]
    Play(play::Play),
    #[cfg(feature = "remote")]
    Serve(serve::Serve),
    Disasm(disasm::Disasm),
    Export(export::Export),
    Render(render::Render),
    Gc(gc::Gc),
    #[structopt(alias = "ws")]
    Workspace(workspace::Workspace),
}

pub fn main() {
    let args = Arguments::from_args();
    match args {
        Arguments::Build(args) => args.run(),
        Arguments::Play(args) => args.run(),
        Arguments::Disasm(args) => args.run(),
        #[cfg(feature = "remote")]
        Arguments::Serve(args) => args.run(),
        Arguments::Export(args) => args.run(),
        Arguments::Render(args) => args.run(),
        Arguments::Gc(args) => args.run(),
        Arguments::Workspace(args) => args.run(),
    }
    // TODO better error message
    .unwrap()
}
