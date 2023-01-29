use anyhow::Result;
use structopt::StructOpt;

pub mod build;
pub mod compiler;
pub mod disasm;
pub mod export;
pub mod gc;
pub mod logger;
pub mod manifest;
#[cfg(feature = "play")]
pub mod play;
pub mod render;
pub mod watcher;
pub mod workspace;

#[cfg(feature = "remote")]
pub mod serve;

#[derive(Debug, StructOpt)]
pub enum Arguments {
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

impl Arguments {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }

    pub fn from_vec<I, S>(iter: I) -> Self
    where
        I: Iterator<Item = S>,
        S: Clone + Into<std::ffi::OsString>,
    {
        StructOpt::from_iter(iter)
    }

    pub fn init_logger(&self) {
        #[cfg(feature = "play")]
        {
            if let Arguments::Play(args) = self {
                if !args.headless {
                    logger::init_tui();
                    return;
                }
            }
        }

        logger::init();
    }

    pub fn run(self) -> Result<()> {
        match self {
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
        }
    }
}

static mut IS_ALT_SCREEN: bool = false;

pub fn is_alt_screen() -> bool {
    unsafe { IS_ALT_SCREEN }
}
