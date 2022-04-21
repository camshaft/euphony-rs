use crate::output;
use bach::executor::{Environment, Executor, Handle};
use core::{future::Future, task::Poll};
use euphony_units::time::Tempo;
use structopt::StructOpt;

pub use bach::{
    executor::JoinHandle,
    rand,
    task::{spawn, spawn_primary},
};

pub mod group;
pub mod time;

pub struct Runtime {
    executor: Executor<Env>,
}

impl Runtime {
    pub fn from_env() -> Self {
        let args = crate::args::Args::from_args();

        if let Some(tempo) = args.tempo {
            let tempo = Tempo(tempo, 1);
            time::set_tempo(tempo);
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
    scheduler: time::Scheduler,
    handle: Handle,
    rand: crate::rand::Scope,
}

impl Env {
    fn new(handle: &Handle, seed: u64) -> Self {
        let scheduler = time::Scheduler::new();
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
