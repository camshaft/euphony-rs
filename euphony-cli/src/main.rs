#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn main() {
    euphony_cli::main();
}
