use anyhow::Result;
use structopt::StructOpt;

mod build;
mod compiler;
mod disasm;
mod export;
mod gc;
mod logger;
mod manifest;
#[cfg(feature = "play")]
mod play;
mod render;
mod watcher;
mod workspace;

#[cfg(feature = "remote")]
mod serve;

#[derive(Debug, StructOpt)]
enum Arguments {
    Build(build::Build),
    #[cfg(feature = "play")]
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

static mut IS_ALT_SCREEN: bool = false;

pub fn is_alt_screen() -> bool {
    unsafe { IS_ALT_SCREEN }
}

fn init_logger(args: &Arguments) {
    #[cfg(feature = "play")]
    {
        if let Arguments::Play(args) = args {
            if !args.headless {
                logger::init_tui();
                return;
            }
        }
    }

    logger::init();
    let _ = args;
}

pub fn main() {
    let args = Arguments::from_args();

    init_logger(&args);

    let res = match args {
        Arguments::Build(args) => args.run(),
        #[cfg(feature = "play")]
        Arguments::Play(args) => args.run(),
        Arguments::Disasm(args) => args.run(),
        #[cfg(feature = "remote")]
        Arguments::Serve(args) => args.run(),
        Arguments::Export(args) => args.run(),
        Arguments::Render(args) => args.run(),
        Arguments::Gc(args) => args.run(),
        Arguments::Workspace(args) => args.run(),
    };

    if let Err(err) = res {
        logger::error!("{}", err);
    }
}
