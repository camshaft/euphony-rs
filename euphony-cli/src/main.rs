#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn main() {
    let args = euphony_cli::Arguments::from_args();
    args.init_logger();
    if let Err(err) = args.run() {
        log::error!("{:#}", err)
    }
}
