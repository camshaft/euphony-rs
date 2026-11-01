use crate::{Assignment, AttemptId, Window, WorkerId};
use core::fmt;
use euphony_daw_identity::{DecodeError, Decoder, Digest, Encoder};
use euphony_daw_render::RenderDescriptor;
use euphony_daw_sample::{FormatError, WorkingAudio};
use euphony_daw_store::{Journal, JournalError, ObjectKind, Store, StoreError};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, VecDeque},
    path::Path,
};

const OPENED: u8 = 0;
const SUBMITTED: u8 = 1;
const COMPLETED: u8 = 2;

/// Why the scheduler's journal didn't open or record.
#[derive(Debug)]
pub enum SchedulerError {
    Journal(JournalError),
    /// A record didn't decode.
    Decode(DecodeError),
    /// A completion names a job the journal doesn't hold.
    UnknownJob(Digest),
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Journal(error) => write!(f, "{error}"),
            Self::Decode(error) => write!(f, "a job record didn't decode: {error}"),
            Self::UnknownJob(job) => write!(f, "job {job} isn't held"),
        }
    }
}

impl std::error::Error for SchedulerError {}

impl From<JournalError> for SchedulerError {
    fn from(error: JournalError) -> Self {
        Self::Journal(error)
    }
}

impl From<DecodeError> for SchedulerError {
    fn from(error: DecodeError) -> Self {
        Self::Decode(error)
    }
}

/// Why a finished attempt's result was refused.
#[derive(Debug)]
pub enum Rejection {
    /// The bytes received don't have the digest the worker claimed.
    DigestMismatch,
    /// The bytes aren't working audio.
    Malformed(FormatError),
    /// The audio's format or length differs from the descriptor's.
    WrongFormat,
    /// The audio holds a sample that isn't finite.
    NonFinite,
    /// The worker named another descriptor than the attempt's.
    WrongDescriptor,
    /// The store refused the publication.
    Store(StoreError),
}

/// What a finished attempt's result came to.
#[derive(Debug)]
pub enum Completion {
    /// The result was validated, published and selected for its job.
    Accepted(Digest),
    /// The attempt answered already, or its job had a result already.
    Duplicate,
    /// The attempt belongs to an earlier epoch or was lost and reassigned.
    Stale,
    /// The result was refused, and the job was queued again.
    Rejected(Rejection),
}

/// Counts of what the scheduler did, for measurement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub submitted: u64,
    pub assigned: u64,
    pub accepted: u64,
    pub duplicates: u64,
    pub stale: u64,
    pub rejected: u64,
    pub busy: u64,
    pub lost: u64,
    pub requeued: u64,
    pub bytes_received: u64,
    pub bytes_published: u64,
    /// Calls into the scheduler, the controller's work.
    pub operations: u64,
    /// The most jobs that waited for an attempt at once.
    pub max_ready: usize,
}

struct Job {
    descriptor: RenderDescriptor,
    result: Option<Digest>,
    live: usize,
    queued: bool,
}

impl Job {
    fn new(descriptor: RenderDescriptor) -> Self {
        Self {
            descriptor,
            result: None,
            live: 0,
            queued: false,
        }
    }
}

struct Attempt {
    job: Digest,
    worker: WorkerId,
    received: Vec<u8>,
}

enum Retired {
    Finished,
    Lost,
}

#[derive(Default)]
struct WorkerState {
    window: Window,
    attempts: BTreeSet<AttemptId>,
}

/// The controller's durable record of render jobs and its live attempts.
pub struct Scheduler {
    journal: Journal,
    epoch: u64,
    next_attempt: u64,
    jobs: BTreeMap<Digest, Job>,
    ready: VecDeque<Digest>,
    attempts: HashMap<AttemptId, Attempt>,
    retired: HashMap<AttemptId, Retired>,
    workers: BTreeMap<WorkerId, WorkerState>,
    stats: Stats,
}

impl Scheduler {
    /// Opens the job journal at `path`: each job without a result is queued
    /// again, each result is kept, and a new epoch starts.
    pub fn open(path: &Path) -> Result<Self, SchedulerError> {
        let (journal, records) = Journal::open(path)?;
        let mut scheduler = Self {
            journal,
            epoch: 0,
            next_attempt: 0,
            jobs: BTreeMap::new(),
            ready: VecDeque::new(),
            attempts: HashMap::new(),
            retired: HashMap::new(),
            workers: BTreeMap::new(),
            stats: Stats::default(),
        };
        let mut order = Vec::new();
        for record in &records {
            let mut decoder = Decoder::new(record);
            match decoder.u8()? {
                OPENED => scheduler.epoch += 1,
                SUBMITTED => {
                    let descriptor = RenderDescriptor::decode(decoder.bytes()?)?;
                    let id = descriptor.id();
                    order.push(id);
                    scheduler.jobs.insert(id, Job::new(descriptor));
                }
                COMPLETED => {
                    let id = decoder.digest()?;
                    let result = decoder.digest()?;
                    let job = scheduler.jobs.get_mut(&id);
                    job.ok_or(SchedulerError::UnknownJob(id))?.result = Some(result);
                }
                _ => return Err(DecodeError::Invalid("job record").into()),
            }
            decoder.finish()?;
        }
        for id in order {
            scheduler.requeue(id);
        }
        scheduler.epoch += 1;
        scheduler.next_attempt = scheduler.epoch << 40;
        scheduler.journal.append(&[OPENED])?;
        Ok(scheduler)
    }

    /// Submits a job, once: a descriptor submitted already returns its
    /// identity and changes nothing.
    pub fn submit(&mut self, descriptor: RenderDescriptor) -> Result<Digest, SchedulerError> {
        let id = descriptor.id();
        if self.jobs.contains_key(&id) {
            return Ok(id);
        }
        let mut record = Encoder::new();
        record.u8(SUBMITTED).bytes(&descriptor.canonical_bytes());
        self.journal.append(&record.finish())?;
        self.jobs.insert(id, Job::new(descriptor));
        self.requeue(id);
        self.stats.submitted += 1;
        Ok(id)
    }

    /// Queues a job for an attempt where it has no result, no live attempt,
    /// and no place in the queue.
    fn requeue(&mut self, id: Digest) -> bool {
        let Some(job) = self.jobs.get_mut(&id) else {
            return false;
        };
        if job.result.is_some() || job.live > 0 || job.queued {
            return false;
        }
        job.queued = true;
        self.ready.push_back(id);
        self.stats.max_ready = self.stats.max_ready.max(self.ready.len());
        true
    }

    /// Ends one attempt of a job, and queues the job again where nothing else
    /// will finish it.
    fn release(&mut self, id: Digest) {
        if let Some(job) = self.jobs.get_mut(&id) {
            job.live = job.live.saturating_sub(1);
        }
        if self.requeue(id) {
            self.stats.requeued += 1;
        }
    }

    pub fn join(&mut self, worker: WorkerId) {
        self.stats.operations += 1;
        self.workers.entry(worker).or_default();
    }

    /// Removes a worker whose connection closed, and queues each job it held
    /// again.
    pub fn leave(&mut self, worker: WorkerId) {
        self.stats.operations += 1;
        let Some(state) = self.workers.remove(&worker) else {
            return;
        };
        for attempt in state.attempts {
            if let Some(entry) = self.attempts.remove(&attempt) {
                self.retired.insert(attempt, Retired::Lost);
                self.stats.lost += 1;
                self.release(entry.job);
            }
        }
    }

    /// Assigns the next waiting job to `worker`, where its window has room.
    pub fn assign(&mut self, worker: WorkerId) -> Option<Assignment> {
        let state = self.workers.get(&worker)?;
        if state.attempts.len() >= state.window.limit() {
            return None;
        }
        while let Some(id) = self.ready.pop_front() {
            let Some(job) = self.jobs.get_mut(&id) else {
                continue;
            };
            job.queued = false;
            if job.result.is_some() {
                continue;
            }
            job.live += 1;
            let descriptor = job.descriptor.clone();
            let attempt = AttemptId(self.next_attempt);
            self.next_attempt += 1;
            let entry = Attempt {
                job: id,
                worker,
                received: Vec::new(),
            };
            self.attempts.insert(attempt, entry);
            if let Some(state) = self.workers.get_mut(&worker) {
                state.attempts.insert(attempt);
            }
            self.stats.assigned += 1;
            self.stats.operations += 1;
            return Some(Assignment {
                attempt,
                epoch: self.epoch,
                descriptor,
            });
        }
        None
    }

    /// Adds the next part of a live attempt's result. A part of an attempt
    /// that isn't live is dropped.
    pub fn chunk(&mut self, worker: WorkerId, attempt: AttemptId, bytes: &[u8]) {
        self.stats.operations += 1;
        self.stats.bytes_received += bytes.len() as u64;
        if let Some(entry) = self.attempts.get_mut(&attempt) {
            if entry.worker == worker {
                entry.received.extend_from_slice(bytes);
            }
        }
    }

    /// Ends an attempt the worker reports failed, or that the connection's
    /// failure detector reports lost, and queues its job again.
    pub fn fail(&mut self, worker: WorkerId, attempt: AttemptId) {
        self.stats.operations += 1;
        if self.end(worker, attempt) {
            self.stats.lost += 1;
            self.congestion(worker);
        }
    }

    /// Ends an attempt the worker refused for capacity, and queues its job
    /// again.
    pub fn busy(&mut self, worker: WorkerId, attempt: AttemptId) {
        self.stats.operations += 1;
        if self.end(worker, attempt) {
            self.stats.busy += 1;
            self.congestion(worker);
        }
    }

    fn end(&mut self, worker: WorkerId, attempt: AttemptId) -> bool {
        let live = self.attempts.get(&attempt);
        if !live.is_some_and(|entry| entry.worker == worker) {
            return false;
        }
        let Some(entry) = self.attempts.remove(&attempt) else {
            return false;
        };
        if let Some(state) = self.workers.get_mut(&worker) {
            state.attempts.remove(&attempt);
        }
        self.retired.insert(attempt, Retired::Lost);
        self.release(entry.job);
        true
    }

    fn congestion(&mut self, worker: WorkerId) {
        if let Some(state) = self.workers.get_mut(&worker) {
            state.window.on_congestion();
        }
    }

    fn success(&mut self, worker: WorkerId) {
        if let Some(state) = self.workers.get_mut(&worker) {
            state.window.on_success();
        }
    }

    /// Takes a finished attempt's result: it is selected and published only
    /// where the attempt is live in this epoch, its job has no result yet, and
    /// its bytes are finite working audio of the descriptor's format with the
    /// digest claimed.
    pub fn finish(
        &mut self,
        worker: WorkerId,
        attempt: AttemptId,
        epoch: u64,
        descriptor: Digest,
        result: Digest,
        store: &dyn Store,
    ) -> Result<Completion, SchedulerError> {
        self.stats.operations += 1;
        if epoch != self.epoch {
            self.stats.stale += 1;
            return Ok(Completion::Stale);
        }
        let live = self.attempts.get(&attempt);
        if !live.is_some_and(|entry| entry.worker == worker) {
            return Ok(self.retired_completion(attempt));
        }
        let Some(entry) = self.attempts.remove(&attempt) else {
            return Ok(self.retired_completion(attempt));
        };
        if let Some(state) = self.workers.get_mut(&worker) {
            state.attempts.remove(&attempt);
        }
        self.retired.insert(attempt, Retired::Finished);

        let job = entry.job;
        let checked = match self.jobs.get(&job) {
            Some(held) if descriptor == job => validate(&held.descriptor, &entry.received, &result),
            _ => Err(Rejection::WrongDescriptor),
        };
        if let Err(rejection) = checked {
            self.stats.rejected += 1;
            self.congestion(worker);
            self.release(job);
            return Ok(Completion::Rejected(rejection));
        }
        if self.jobs.get(&job).and_then(|held| held.result).is_some() {
            self.stats.duplicates += 1;
            self.success(worker);
            self.release(job);
            return Ok(Completion::Duplicate);
        }
        let kind = ObjectKind::WorkingAudio;
        if let Err(error) = store.publish_expected(kind, &entry.received, &result) {
            self.stats.rejected += 1;
            self.release(job);
            return Ok(Completion::Rejected(Rejection::Store(error)));
        }
        let mut record = Encoder::new();
        record.u8(COMPLETED).digest(&job).digest(&result);
        self.journal.append(&record.finish())?;
        if let Some(held) = self.jobs.get_mut(&job) {
            held.result = Some(result);
        }
        self.release(job);
        self.stats.accepted += 1;
        self.stats.bytes_published += entry.received.len() as u64;
        self.success(worker);
        Ok(Completion::Accepted(result))
    }

    fn retired_completion(&mut self, attempt: AttemptId) -> Completion {
        match self.retired.get(&attempt) {
            Some(Retired::Finished) => {
                self.stats.duplicates += 1;
                Completion::Duplicate
            }
            _ => {
                self.stats.stale += 1;
                Completion::Stale
            }
        }
    }

    /// The authority epoch, which rises with each open.
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Whether every job has a selected result.
    pub fn is_complete(&self) -> bool {
        self.jobs.values().all(|job| job.result.is_some())
    }

    /// The result selected for a job.
    pub fn result(&self, job: &Digest) -> Option<Digest> {
        self.jobs.get(job).and_then(|job| job.result)
    }

    /// The window of a joined worker.
    pub fn window(&self, worker: WorkerId) -> Option<usize> {
        self.workers.get(&worker).map(|state| state.window.limit())
    }
}

fn validate(
    descriptor: &RenderDescriptor,
    bytes: &[u8],
    result: &Digest,
) -> Result<(), Rejection> {
    if ObjectKind::WorkingAudio.digest(bytes) != *result {
        return Err(Rejection::DigestMismatch);
    }
    let audio = WorkingAudio::from_canonical_bytes(bytes).map_err(Rejection::Malformed)?;
    let header = audio.header();
    if header.sample_rate != descriptor.sample_rate
        || header.layout != descriptor.layout
        || header.frames != descriptor.frames
    {
        return Err(Rejection::WrongFormat);
    }
    audio.check_publishable().map_err(|_| Rejection::NonFinite)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{perform, FromWorker};
    use euphony_daw_render::{builtin_registry, mix_processor};
    use euphony_daw_sample::ChannelLayout;
    use euphony_daw_store::MemStore;

    fn descriptor(frames: u64) -> RenderDescriptor {
        RenderDescriptor {
            sample_rate: 48_000,
            layout: ChannelLayout::Stereo,
            start: 0,
            frames,
            processor: mix_processor().identity(),
            inputs: Vec::new(),
        }
    }

    struct Answer {
        bytes: Vec<u8>,
        epoch: u64,
        descriptor: Digest,
        result: Digest,
    }

    fn answer(assignment: &Assignment, store: &MemStore) -> Answer {
        let mut bytes = Vec::new();
        let mut finished = None;
        for message in perform(assignment, &builtin_registry(), store) {
            match message {
                FromWorker::Chunk { bytes: part, .. } => bytes.extend(part),
                FromWorker::Finished {
                    epoch,
                    descriptor,
                    result,
                    ..
                } => finished = Some((epoch, descriptor, result)),
                other => panic!("unexpected {other:?}"),
            }
        }
        let (epoch, descriptor, result) = finished.unwrap();
        Answer {
            bytes,
            epoch,
            descriptor,
            result,
        }
    }

    fn deliver(
        scheduler: &mut Scheduler,
        worker: WorkerId,
        assignment: &Assignment,
        answer: &Answer,
        store: &MemStore,
    ) -> Completion {
        scheduler.chunk(worker, assignment.attempt, &answer.bytes);
        let attempt = assignment.attempt;
        let (epoch, descriptor, result) = (answer.epoch, answer.descriptor, answer.result);
        let completion = scheduler.finish(worker, attempt, epoch, descriptor, result, store);
        completion.unwrap()
    }

    #[test]
    fn a_repeated_answer_is_a_duplicate() {
        let dir = tempfile::tempdir().unwrap();
        let store = MemStore::new();
        let mut scheduler = Scheduler::open(&dir.path().join("jobs")).unwrap();
        let job = scheduler.submit(descriptor(4)).unwrap();
        let worker = WorkerId(1);
        scheduler.join(worker);
        let assignment = scheduler.assign(worker).unwrap();
        let answer = answer(&assignment, &store);
        let first = deliver(&mut scheduler, worker, &assignment, &answer, &store);
        assert!(matches!(first, Completion::Accepted(_)));
        let again = deliver(&mut scheduler, worker, &assignment, &answer, &store);
        assert!(matches!(again, Completion::Duplicate));
        assert_eq!(scheduler.result(&job), Some(answer.result));
        assert!(scheduler.is_complete());
    }

    #[test]
    fn a_lost_attempt_is_reassigned_and_its_late_answer_is_stale() {
        let dir = tempfile::tempdir().unwrap();
        let store = MemStore::new();
        let mut scheduler = Scheduler::open(&dir.path().join("jobs")).unwrap();
        scheduler.submit(descriptor(4)).unwrap();
        let (slow, other) = (WorkerId(1), WorkerId(2));
        scheduler.join(slow);
        scheduler.join(other);
        let lost = scheduler.assign(slow).unwrap();
        assert!(scheduler.assign(other).is_none());
        scheduler.fail(slow, lost.attempt);
        let again = scheduler.assign(other).unwrap();
        assert_ne!(again.attempt, lost.attempt);

        let late = answer(&lost, &store);
        let late = deliver(&mut scheduler, slow, &lost, &late, &store);
        assert!(matches!(late, Completion::Stale));
        assert!(store.is_empty());

        let answer = answer(&again, &store);
        let current = deliver(&mut scheduler, other, &again, &answer, &store);
        assert!(matches!(current, Completion::Accepted(_)));
    }

    #[test]
    fn a_worker_that_leaves_mid_upload_publishes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let store = MemStore::new();
        let mut scheduler = Scheduler::open(&dir.path().join("jobs")).unwrap();
        scheduler.submit(descriptor(8)).unwrap();
        let worker = WorkerId(1);
        scheduler.join(worker);
        let assignment = scheduler.assign(worker).unwrap();
        let answer = answer(&assignment, &store);
        let half = &answer.bytes[..answer.bytes.len() / 2];
        scheduler.chunk(worker, assignment.attempt, half);
        scheduler.leave(worker);
        assert!(store.is_empty());
        assert!(!scheduler.is_complete());
        assert_eq!(scheduler.stats().requeued, 1);

        let next = WorkerId(2);
        scheduler.join(next);
        assert!(scheduler.assign(next).is_some());
    }

    #[test]
    fn corrupt_and_misformatted_results_are_refused_and_requeued() {
        let dir = tempfile::tempdir().unwrap();
        let store = MemStore::new();
        let mut scheduler = Scheduler::open(&dir.path().join("jobs")).unwrap();
        scheduler.submit(descriptor(4)).unwrap();
        let worker = WorkerId(1);
        scheduler.join(worker);

        let assignment = scheduler.assign(worker).unwrap();
        let mut corrupt = answer(&assignment, &store);
        let last = corrupt.bytes.len() - 1;
        corrupt.bytes[last] ^= 1;
        let refused = deliver(&mut scheduler, worker, &assignment, &corrupt, &store);
        assert!(matches!(
            refused,
            Completion::Rejected(Rejection::DigestMismatch)
        ));

        let assignment = scheduler.assign(worker).unwrap();
        let mut short = assignment.clone();
        short.descriptor = descriptor(2);
        let mut wrong = answer(&short, &store);
        wrong.descriptor = assignment.descriptor.id();
        let refused = deliver(&mut scheduler, worker, &assignment, &wrong, &store);
        assert!(matches!(refused, Completion::Rejected(Rejection::WrongFormat)));
        assert!(store.is_empty());
        assert_eq!(scheduler.stats().rejected, 2);
        assert!(scheduler.assign(worker).is_some());
    }

    #[test]
    fn a_reopened_scheduler_keeps_results_and_refuses_the_old_epoch() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("jobs");
        let store = MemStore::new();
        let worker = WorkerId(1);
        let (done, old) = {
            let mut scheduler = Scheduler::open(&path).unwrap();
            let done = scheduler.submit(descriptor(4)).unwrap();
            scheduler.submit(descriptor(6)).unwrap();
            scheduler.join(worker);
            let first = scheduler.assign(worker).unwrap();
            let answer = answer(&first, &store);
            deliver(&mut scheduler, worker, &first, &answer, &store);
            let old = scheduler.assign(worker).unwrap();
            (done, old)
        };

        let mut scheduler = Scheduler::open(&path).unwrap();
        assert_eq!(scheduler.epoch(), 2);
        assert!(scheduler.result(&done).is_some());
        scheduler.join(worker);
        let late = answer(&old, &store);
        let late = deliver(&mut scheduler, worker, &old, &late, &store);
        assert!(matches!(late, Completion::Stale));

        let fresh = scheduler.assign(worker).unwrap();
        assert_eq!(fresh.descriptor, old.descriptor);
        assert_eq!(fresh.epoch, 2);
        let answer = answer(&fresh, &store);
        let accepted = deliver(&mut scheduler, worker, &fresh, &answer, &store);
        assert!(matches!(accepted, Completion::Accepted(_)));
        assert!(scheduler.is_complete());
    }

    #[test]
    fn a_busy_answer_requeues_and_halves_the_window() {
        let dir = tempfile::tempdir().unwrap();
        let store = MemStore::new();
        let mut scheduler = Scheduler::open(&dir.path().join("jobs")).unwrap();
        for frames in 1..=4 {
            scheduler.submit(descriptor(frames)).unwrap();
        }
        let worker = WorkerId(1);
        scheduler.join(worker);
        for _ in 0..2 {
            let assignment = scheduler.assign(worker).unwrap();
            let answer = answer(&assignment, &store);
            deliver(&mut scheduler, worker, &assignment, &answer, &store);
        }
        assert_eq!(scheduler.window(worker), Some(3));
        let refused = scheduler.assign(worker).unwrap();
        scheduler.busy(worker, refused.attempt);
        assert_eq!(scheduler.window(worker), Some(1));
        assert_eq!(scheduler.stats().busy, 1);
        assert!(scheduler.assign(worker).is_some());
    }
}
