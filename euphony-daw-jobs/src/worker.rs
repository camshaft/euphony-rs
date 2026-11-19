use crate::{Assignment, FromWorker};
use euphony_daw_render::render;
use euphony_daw_sample::Registry;
use euphony_daw_store::{ObjectKind, Store};

/// Performs an assignment, as every local and remote worker does, and
/// returns the messages that answer it: the result's bytes in order, then
/// its digest, or the reason it failed.
pub fn perform(assignment: &Assignment, registry: &Registry, store: &dyn Store) -> Vec<FromWorker> {
    let attempt = assignment.attempt;
    let audio = match render(&assignment.descriptor, registry, store) {
        Ok(audio) => audio,
        Err(error) => {
            let reason = error.to_string();
            return vec![FromWorker::Failed { attempt, reason }];
        }
    };
    let bytes = audio.to_canonical_bytes();
    let result = ObjectKind::WorkingAudio.digest(&bytes);
    let (head, tail) = bytes.split_at(bytes.len() / 2);
    vec![
        FromWorker::Chunk {
            attempt,
            bytes: head.to_vec(),
        },
        FromWorker::Chunk {
            attempt,
            bytes: tail.to_vec(),
        },
        FromWorker::Finished {
            attempt,
            epoch: assignment.epoch,
            descriptor: assignment.descriptor.id(),
            result,
        },
    ]
}
