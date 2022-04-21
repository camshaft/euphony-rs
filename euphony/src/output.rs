use core::fmt;
pub use euphony_command::*;
use std::{fs, io, path::Path, time::Duration};

bach::scope::define!(scope, Box<dyn io::Write>);

pub fn set_file(path: &Path) {
    let file = fs::File::create(path).unwrap();
    let file = io::BufWriter::new(file);
    let output = Box::new(file);
    scope::set(Some(output));
}

pub fn set_stdout() {
    let io = io::stdout();
    let output = Box::new(io);
    scope::set(Some(output));
}

pub(crate) fn emit<M: Codec + fmt::Display>(message: M) {
    scope::try_borrow_mut_with(|output| {
        if let Some(output) = output.as_mut() {
            message.encode(output).unwrap();
        } else {
            println!("{}", message);
        }
    });
}

pub(crate) fn create_group(id: u64, name: String) {
    emit(CreateGroup { id, name });
}

pub(crate) fn advance_time(ticks: u64) {
    emit(AdvanceTime { ticks })
}

pub(crate) fn set_tick_duration(duration: Duration) {
    emit(SetNanosPerTick {
        nanos: duration.as_nanos() as _,
    })
}

pub(crate) fn finish() {
    scope::try_borrow_mut_with(|output| {
        if let Some(output) = output.as_mut() {
            output.flush().unwrap();
        }
    });
}
