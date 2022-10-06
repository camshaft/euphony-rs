use crate::{group::Grouped, output};
use bach::executor::{Environment, Executor, Handle};
use core::{future::Future, task::Poll};
use euphony_units::time::Tempo;

pub use bach::{executor::JoinHandle, rand};

pub mod primary {
    use super::*;
    pub use bach::task::primary::*;

    pub fn spawn<F: 'static + Future<Output = T> + Send, T: 'static + Send>(
        future: F,
    ) -> JoinHandle<T> {
        // try to inherit the parent group
        crate::group::scope::try_borrow_with(|group| {
            if let Some(group) = group {
                bach::task::primary::spawn(Grouped::new(future, *group))
            } else {
                bach::task::primary::spawn(future)
            }
        })
    }
}

pub fn spawn<F: 'static + Future<Output = T> + Send, T: 'static + Send>(
    future: F,
) -> JoinHandle<T> {
    // try to inherit the parent group
    crate::group::scope::try_borrow_with(|group| {
        if let Some(group) = group {
            bach::task::spawn(Grouped::new(future, *group))
        } else {
            bach::task::spawn(future)
        }
    })
}

pub struct Runtime {
    executor: Executor<Env>,
}

impl Runtime {
    pub fn from_env() -> Self {
        let args = crate::args::Args::from_args();

        if let Some(tempo) = args.tempo {
            let tempo = Tempo(tempo, 1);
            crate::time::set_tempo(tempo);
        }

        if let Some(path) = args.output.as_ref() {
            if path.to_str() != Some("-") {
                output::set_file(path);
            } else {
                // setting the output argument forces binary mode
                output::set_stdout();
            }
        }

        let seed = if let Some(seed) = args.seed {
            seed
        } else {
            *crate::rand::SEED
        };

        Self::new(seed)
    }

    pub fn new(seed: u64) -> Self {
        let executor = Executor::new(|handle| Env::new(handle, seed));
        Self { executor }
    }

    pub fn block_on<F, Output>(&mut self, task: F) -> Output
    where
        F: 'static + Future<Output = Output> + Send,
        Output: 'static + Send,
    {
        let result = self.executor.block_on(task);

        // finish any primary tasks
        self.executor.block_on_primary();

        output::finish();

        result
    }
}

struct Env {
    scheduler: crate::time::Scheduler,
    handle: Handle,
    rand: crate::rand::Scope,
}

impl Env {
    fn new(handle: &Handle, seed: u64) -> Self {
        let scheduler = crate::time::Scheduler::new();
        let rand = crate::rand::Scope::new(seed);

        Self {
            scheduler,
            handle: handle.clone(),
            rand,
        }
    }
}

impl Environment for Env {
    fn run<Tasks, F>(&mut self, tasks: Tasks) -> Poll<()>
    where
        Tasks: Iterator<Item = F> + Send,
        F: 'static + FnOnce() -> Poll<()> + Send,
    {
        let mut is_ready = true;

        let Self {
            ref scheduler,
            ref handle,
            ref rand,
        } = self;

        handle.enter(|| {
            scheduler.enter(|| {
                rand.enter(|| {
                    for task in tasks {
                        is_ready &= task().is_ready();
                    }
                })
            })
        });

        if is_ready {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }

    fn on_macrostep(&mut self, count: usize) {
        // wait until all tasks settle before waking the timer
        if count > 0 {
            return;
        }

        let mut advance = 0;
        while let Some(ticks) = self.scheduler.advance() {
            advance += ticks;
            if self.scheduler.wake() > 0 {
                break;
            }
        }

        if advance > 0 {
            output::advance_time(advance);
        }
    }
}
