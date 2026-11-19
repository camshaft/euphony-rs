//! A render worker process: it reads assignments from its standard input,
//! renders each from the store directory its first argument names, and
//! answers on its standard output.
//!
//! A second argument selects a fault for tests: `duplicate` answers each
//! assignment twice, `corrupt` damages its first answer, `exit-mid` exits
//! after the first part of its first answer, and `stall` never answers.

use euphony_daw_jobs::{perform, read_frame, write_frame, FromWorker, ToWorker};
use euphony_daw_render::builtin_registry;
use euphony_daw_store::DirStore;
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(root) = args.next() else {
        eprintln!("usage: euphony-daw-worker <store> [fault]");
        return ExitCode::from(2);
    };
    let fault = args.next().unwrap_or_default();
    let store = match DirStore::open_to_read(root) {
        Ok(store) => store,
        Err(error) => {
            eprintln!("the store didn't open: {error}");
            return ExitCode::from(1);
        }
    };
    let registry = builtin_registry();
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    let mut answered = 0u64;

    while let Ok(Some(frame)) = read_frame(&mut input) {
        let assignment = match ToWorker::decode(&frame) {
            Ok(ToWorker::Assign(assignment)) => assignment,
            Ok(ToWorker::Shutdown) => return ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("a message didn't decode: {error}");
                return ExitCode::from(1);
            }
        };
        if fault == "stall" {
            continue;
        }
        let mut messages = perform(&assignment, &registry, &store);
        answered += 1;
        if fault == "corrupt" && answered == 1 {
            if let Some(FromWorker::Chunk { bytes, .. }) = messages.first_mut() {
                if let Some(byte) = bytes.last_mut() {
                    *byte ^= 1;
                }
            }
        }
        if fault == "exit-mid" {
            if let Some(first) = messages.first() {
                let _ = write_frame(&mut output, &first.encode());
            }
            let _ = output.flush();
            return ExitCode::from(3);
        }
        let copies = if fault == "duplicate" { 2 } else { 1 };
        for _ in 0..copies {
            for message in &messages {
                if write_frame(&mut output, &message.encode()).is_err() {
                    return ExitCode::from(1);
                }
            }
        }
    }
    ExitCode::SUCCESS
}
