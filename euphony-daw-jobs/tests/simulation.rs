//! Dozens of simulated workers render one project through the scheduler in a
//! deterministic discrete-event simulation, with workers that leave
//! mid-upload, answer twice, answer after their link failed, corrupt an
//! answer, or refuse work for capacity.

mod common;

use bach::{
    ext::*,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::Instant,
};
use euphony_daw_identity::Digest;
use euphony_daw_jobs::{
    perform, Assignment, AttemptId, FromWorker, Scheduler, Stats, ToWorker, WorkerId,
};
use euphony_daw_render::builtin_registry;
use euphony_daw_sample::Registry;
use euphony_daw_store::{MemStore, ObjectKind};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    time::Duration,
};

/// The controller's work for one event, in simulated time.
const SERVICE: Duration = Duration::from_micros(2);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fault {
    None,
    Leaves,
    Duplicates,
    SlowLink,
    Corrupts,
}

enum Event {
    Joined(WorkerId, UnboundedSender<ToWorker>),
    Message(WorkerId, FromWorker),
    /// The connection's failure detector declared an attempt lost.
    Lost(WorkerId, AttemptId),
    Closed(WorkerId),
}

#[derive(Clone)]
struct Run {
    id: WorkerId,
    fault: Fault,
    events: UnboundedSender<Event>,
    store: Rc<MemStore>,
    registry: Rc<Registry>,
    busy: Rc<Cell<usize>>,
    gone: Rc<Cell<bool>>,
}

impl Run {
    fn send(&self, event: Event) {
        if !self.gone.get() {
            let _ = self.events.send(event);
        }
    }
}

struct Report {
    workers: usize,
    elapsed: Duration,
    events: u64,
    stats: Stats,
    results: BTreeMap<Digest, Option<Digest>>,
}

async fn attempt(run: Run, assignment: Assignment, leave: bool) {
    let half = Duration::from_micros(assignment.descriptor.frames) / 2;
    let attempt = assignment.attempt;
    half.sleep().await;
    let mut messages = perform(&assignment, &run.registry, &*run.store);
    if run.fault == Fault::Corrupts && attempt.0 % 3 == 0 {
        if let Some(FromWorker::Chunk { bytes, .. }) = messages.first_mut() {
            if let Some(byte) = bytes.last_mut() {
                *byte ^= 1;
            }
        }
    }
    let mut messages = messages.into_iter();
    if let Some(first) = messages.next() {
        run.send(Event::Message(run.id, first));
    }
    if leave {
        run.send(Event::Closed(run.id));
        run.gone.set(true);
        return;
    }
    half.sleep().await;
    if run.fault == Fault::SlowLink && attempt.0 % 2 == 0 {
        run.send(Event::Lost(run.id, attempt));
        (half * 8).sleep().await;
    }
    let rest: Vec<_> = messages.collect();
    for message in &rest {
        run.send(Event::Message(run.id, message.clone()));
    }
    if run.fault == Fault::Duplicates {
        if let Some(last) = rest.last() {
            run.send(Event::Message(run.id, last.clone()));
        }
    }
    run.busy.set(run.busy.get() - 1);
}

async fn worker(run: Run, mut inbox: UnboundedReceiver<ToWorker>, capacity: usize) {
    let mut assigned = 0u64;
    while let Some(message) = inbox.recv().await {
        if run.gone.get() {
            break;
        }
        let ToWorker::Assign(assignment) = message else {
            break;
        };
        assigned += 1;
        if run.busy.get() >= capacity {
            let attempt = assignment.attempt;
            run.send(Event::Message(run.id, FromWorker::Busy { attempt }));
            continue;
        }
        run.busy.set(run.busy.get() + 1);
        let leave = run.fault == Fault::Leaves && assigned == 3;
        attempt(run.clone(), assignment, leave).spawn();
    }
}

struct Controller {
    scheduler: Scheduler,
    store: Rc<MemStore>,
    jobs: Vec<Digest>,
    workers: usize,
    report: Rc<RefCell<Option<Report>>>,
}

async fn controller(context: Controller, mut events: UnboundedReceiver<Event>) {
    let Controller {
        mut scheduler,
        store,
        jobs,
        workers,
        report,
    } = context;
    let mut senders = BTreeMap::new();
    let mut handled = 0u64;
    while !scheduler.is_complete() {
        let Some(event) = events.recv().await else {
            break;
        };
        SERVICE.sleep().await;
        handled += 1;
        match event {
            Event::Joined(id, sender) => {
                scheduler.join(id);
                senders.insert(id, sender);
            }
            Event::Message(id, FromWorker::Chunk { attempt, bytes }) => {
                scheduler.chunk(id, attempt, &bytes);
            }
            Event::Message(
                id,
                FromWorker::Finished {
                    attempt,
                    epoch,
                    descriptor,
                    result,
                },
            ) => {
                let finished = scheduler.finish(id, attempt, epoch, descriptor, result, &*store);
                finished.unwrap();
            }
            Event::Message(id, FromWorker::Failed { attempt, .. }) | Event::Lost(id, attempt) => {
                scheduler.fail(id, attempt);
            }
            Event::Message(id, FromWorker::Busy { attempt }) => scheduler.busy(id, attempt),
            Event::Closed(id) => {
                scheduler.leave(id);
                senders.remove(&id);
            }
        }
        for (id, sender) in &senders {
            while let Some(assignment) = scheduler.assign(*id) {
                if sender.send(ToWorker::Assign(assignment)).is_err() {
                    break;
                }
            }
        }
    }
    let results = jobs.iter().map(|job| (*job, scheduler.result(job)));
    *report.borrow_mut() = Some(Report {
        workers,
        elapsed: Instant::now().elapsed_since_start(),
        events: handled,
        stats: *scheduler.stats(),
        results: results.collect(),
    });
}

type Outcome = (Report, Rc<MemStore>, BTreeMap<Digest, Digest>);

fn simulate(workers: usize, faults: bool) -> Outcome {
    let store = Rc::new(MemStore::new());
    let descriptors = common::fixture(&*store);
    let references = common::references(&descriptors, &*store);
    let dir = tempfile::tempdir().unwrap();
    let mut scheduler = Scheduler::open(&dir.path().join("jobs")).unwrap();
    let mut jobs = Vec::new();
    for descriptor in descriptors {
        jobs.push(scheduler.submit(descriptor).unwrap());
    }

    let report = Rc::new(RefCell::new(None));
    let registry = Rc::new(builtin_registry());
    let context = Controller {
        scheduler,
        store: store.clone(),
        jobs,
        workers,
        report: report.clone(),
    };
    bach::sim(|| {
        let (events, inbox) = unbounded_channel();
        for index in 0..workers {
            let id = WorkerId(index as u64);
            let (sender, receiver) = unbounded_channel();
            assert!(events.send(Event::Joined(id, sender)).is_ok());
            let fault = match (faults, index % 8) {
                (false, _) => Fault::None,
                (true, 0) => Fault::Leaves,
                (true, 1) => Fault::Duplicates,
                (true, 2) => Fault::SlowLink,
                (true, 3) => Fault::Corrupts,
                (true, _) => Fault::None,
            };
            let run = Run {
                id,
                fault,
                events: events.clone(),
                store: store.clone(),
                registry: registry.clone(),
                busy: Rc::new(Cell::new(0)),
                gone: Rc::new(Cell::new(false)),
            };
            worker(run, receiver, 1 + index % 3).spawn();
        }
        drop(events);
        controller(context, inbox).primary().spawn();
    });
    let report = report.borrow_mut().take().expect("the controller finished");
    (report, store, references)
}

fn check(report: &Report, store: &MemStore, references: &BTreeMap<Digest, Digest>) {
    assert_eq!(report.results.len(), references.len());
    for (job, result) in &report.results {
        assert_eq!(result.as_ref(), references.get(job), "job {job}");
    }
    let expected: BTreeSet<_> = references.values().copied().collect();
    let assets = 4;
    let mut published = 0;
    for (kind, digest, _) in store.objects() {
        assert_eq!(kind, ObjectKind::WorkingAudio);
        if expected.contains(&digest) {
            published += 1;
        }
    }
    assert_eq!(store.len(), assets + published);
}

#[test]
fn dozens_of_workers_survive_loss_duplicates_stale_and_corrupt_answers() {
    let (report, store, references) = simulate(48, true);
    check(&report, &store, &references);
    let stats = report.stats;
    eprintln!("faults: {stats:?}");
    assert!(stats.lost > 0, "no attempt was lost");
    assert!(stats.duplicates > 0, "no answer was duplicated");
    assert!(stats.stale > 0, "no answer was stale");
    assert!(stats.rejected > 0, "no answer was refused");
    assert!(stats.requeued > 0, "no job was queued again");
}

#[test]
fn throughput_grows_with_the_worker_count() {
    let mut runs = Vec::new();
    eprintln!("workers  elapsed_us  jobs_per_s  events  controller_busy  received  max_ready");
    for workers in [1, 2, 4, 8, 16, 32, 48] {
        let (report, store, references) = simulate(workers, false);
        check(&report, &store, &references);
        let elapsed = report.elapsed.as_secs_f64();
        let jobs = report.results.len() as f64;
        let busy = (SERVICE * report.events as u32).as_secs_f64() / elapsed;
        eprintln!(
            "{:>7}  {:>10}  {:>10.0}  {:>6}  {:>15.3}  {:>8}  {:>9}",
            report.workers,
            report.elapsed.as_micros(),
            jobs / elapsed,
            report.events,
            busy,
            report.stats.bytes_received,
            report.stats.max_ready,
        );
        runs.push(report);
    }
    let one = runs[0].elapsed;
    let many = runs[5].elapsed;
    assert!(
        many * 8 < one,
        "32 workers took {many:?} against {one:?} for one: independent work was serialized"
    );
}
