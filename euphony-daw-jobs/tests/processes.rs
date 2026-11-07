//! Real worker processes render one project through the scheduler: one
//! answers twice, one corrupts an answer, one exits mid-upload, one never
//! answers and is killed, and the controller restarts halfway.

mod common;

use euphony_daw_identity::Digest;
use euphony_daw_jobs::{
    read_frame, write_frame, Completion, FromWorker, Scheduler, ToWorker, WorkerId,
};
use euphony_daw_store::DirStore;
use std::{
    collections::BTreeMap,
    path::Path,
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc,
    thread,
};

struct Process {
    child: Child,
    stdin: Option<ChildStdin>,
    fault: &'static str,
}

type Events = mpsc::Sender<(WorkerId, Option<FromWorker>)>;

fn spawn(store: &Path, fault: &'static str, id: WorkerId, events: Events) -> Process {
    let mut child = Command::new(env!("CARGO_BIN_EXE_euphony-daw-worker"))
        .arg(store)
        .arg(fault)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    thread::spawn(move || loop {
        let message = match read_frame(&mut stdout) {
            Ok(Some(frame)) => Some(FromWorker::decode(&frame).unwrap()),
            _ => None,
        };
        let closed = message.is_none();
        if events.send((id, message)).is_err() || closed {
            return;
        }
    });
    let stdin = child.stdin.take();
    Process {
        child,
        stdin,
        fault,
    }
}

fn dispatch(scheduler: &mut Scheduler, processes: &mut BTreeMap<WorkerId, Process>) {
    for (id, process) in processes.iter_mut() {
        while let Some(assignment) = scheduler.assign(*id) {
            let Some(stdin) = process.stdin.as_mut() else {
                break;
            };
            let frame = ToWorker::Assign(assignment).encode();
            if write_frame(stdin, &frame).is_err() {
                break;
            }
        }
    }
}

#[derive(Debug, Default)]
struct Seen {
    accepted: usize,
    duplicates: usize,
    stale: usize,
    rejected: usize,
    closed: usize,
}

#[test]
fn worker_processes_survive_duplicates_loss_and_a_controller_restart() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("store");
    let store = DirStore::open(&root).unwrap();
    let descriptors = common::fixture(&store);
    let references = common::references(&descriptors, &store);
    let journal = dir.path().join("jobs");
    let mut scheduler = Scheduler::open(&journal).unwrap();
    let mut jobs: Vec<Digest> = Vec::new();
    for descriptor in &descriptors {
        jobs.push(scheduler.submit(descriptor.clone()).unwrap());
    }

    let (events, inbox) = mpsc::channel();
    let faults = [
        "none",
        "none",
        "none",
        "duplicate",
        "corrupt",
        "exit-mid",
        "stall",
    ];
    let mut processes = BTreeMap::new();
    for (index, fault) in faults.into_iter().enumerate() {
        let id = WorkerId(index as u64);
        processes.insert(id, spawn(&root, fault, id, events.clone()));
        scheduler.join(id);
    }
    drop(events);
    dispatch(&mut scheduler, &mut processes);

    let mut seen = Seen::default();
    let mut restarted = false;
    let mut killed = false;
    while !scheduler.is_complete() {
        let (id, message) = inbox.recv().expect("a worker is still running");
        match message {
            None => {
                seen.closed += 1;
                scheduler.leave(id);
                if let Some(mut process) = processes.remove(&id) {
                    let _ = process.child.wait();
                }
            }
            Some(FromWorker::Chunk { attempt, bytes }) => scheduler.chunk(id, attempt, &bytes),
            Some(FromWorker::Finished {
                attempt,
                epoch,
                descriptor,
                result,
            }) => {
                let completion = scheduler.finish(id, attempt, epoch, descriptor, result, &store);
                match completion.unwrap() {
                    Completion::Accepted(_) => seen.accepted += 1,
                    Completion::Duplicate => seen.duplicates += 1,
                    Completion::Stale => seen.stale += 1,
                    Completion::Rejected(_) => seen.rejected += 1,
                }
            }
            Some(FromWorker::Failed { attempt, .. }) => scheduler.fail(id, attempt),
            Some(FromWorker::Busy { attempt }) => scheduler.busy(id, attempt),
        }
        if seen.accepted > 0 && !killed {
            killed = true;
            for process in processes.values_mut() {
                if process.fault == "stall" {
                    process.child.kill().unwrap();
                }
            }
        }
        if !restarted && seen.accepted * 2 >= jobs.len() {
            restarted = true;
            drop(scheduler);
            scheduler = Scheduler::open(&journal).unwrap();
            assert_eq!(scheduler.epoch(), 2);
            for id in processes.keys() {
                scheduler.join(*id);
            }
        }
        dispatch(&mut scheduler, &mut processes);
    }

    for process in processes.values_mut() {
        if let Some(mut stdin) = process.stdin.take() {
            let _ = write_frame(&mut stdin, &ToWorker::Shutdown.encode());
        }
        let _ = process.child.wait();
    }

    eprintln!("processes: {seen:?}, stale answers: {}", seen.stale);
    for job in &jobs {
        assert_eq!(scheduler.result(job).as_ref(), references.get(job));
    }
    assert!(restarted);
    assert!(seen.duplicates > 0, "no answer was duplicated");
    assert!(seen.rejected > 0, "no corrupt answer was refused");
    assert!(
        seen.closed >= 2,
        "the exiting and the killed workers weren't lost"
    );
    assert_eq!(store.staged_files().unwrap(), 0);
}
