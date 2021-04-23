use bach::executor::{Environment, Executor, Handle};
use core::{future::Future, task::Poll};
use euphony_core::time::{Beat, Tempo};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use structopt::StructOpt;

pub use bach::executor::JoinHandle;
pub use euphony_runtime::rng;

pub mod time {
    pub use euphony_runtime::time::*;

    pub(crate) mod scope {
        euphony_runtime::scope!(scheduler, euphony_runtime::time::Handle);
    }

    pub fn scheduler() -> euphony_runtime::time::Handle {
        scope::borrow(|h| h.clone())
    }
}

pub mod output {
    pub use euphony_runtime::output::*;
    use euphony_sc::track::Handle as Track;

    pub mod scope {
        euphony_runtime::scope!(output, super::Handle);
    }

    pub fn track(name: &str) -> Track {
        scope::borrow(|p| p.track(name))
    }
}

mod scope {
    euphony_runtime::scope!(runtime, bach::executor::Handle);
}

pub fn spawn<F: 'static + Future<Output = T> + Send, T: 'static + Send>(
    future: F,
) -> JoinHandle<T> {
    scope::borrow(|h| h.spawn(future))
}

pub struct Runtime {
    executor: Executor<Env>,
    seed: u64,
}

impl Runtime {
    pub fn from_env() -> Self {
        let args = crate::args::Args::from_args();

        let seed = if let Some(seed) = args.seed {
            seed
        } else {
            let seed = *euphony_runtime::rng::EUPHONY_SEED;
            eprintln!("EUPHONY_SEED={:x?}", seed);
            seed
        };

        let executor = Executor::new(|handle| Env::from_args(&args, handle), None);

        Self { executor, seed }
    }

    pub fn block_on<F, Output>(&mut self, task: F) -> Output
    where
        F: 'static + Future<Output = Output> + Send,
        Output: 'static + Send,
    {
        use euphony_runtime::rng::Task as _;

        // make sure the task has the rng seed
        let task = task.seed(self.seed);

        let result = self.executor.block_on(task, |executor| {
            let env = executor.environment();
            if let Some(time) = env.scheduler.advance() {
                let _ = time;
                env.scheduler.wake();
            }
        });

        self.executor
            .environment()
            .output
            .finish()
            .expect("could not finish output");

        result
    }
}

struct Env {
    output: euphony_runtime::output::Handle,
    scheduler: euphony_runtime::time::Scheduler,
    pool: rayon::ThreadPool,
    is_ready: Arc<AtomicBool>,
}

impl Env {
    fn from_args(args: &crate::args::Args, handle: &Handle) -> Self {
        let tempo = Tempo(args.tempo.unwrap_or(120), 1);

        let scheduler = euphony_runtime::time::Scheduler::new(tempo, Beat(1, 256), None);

        let handle = handle.clone();
        let scheduler_handle = scheduler.handle();

        let output: euphony_runtime::output::Handle = match &args.subcommand {
            Some(crate::args::Cmd::Render(r)) => {
                if r.multitrack {
                    Arc::new(euphony_nrt::multitrack::Project::new(
                        scheduler_handle.clone(),
                        r.output(),
                    ))
                } else {
                    Arc::new(euphony_nrt::singletrack::Project::new(
                        scheduler_handle.clone(),
                        r.output(),
                    ))
                }
            }
            None => Arc::new(euphony_nrt::singletrack::Project::new(
                scheduler_handle.clone(),
                crate::args::Render::default_path(),
            )),
        };

        let output_handle = output.clone();

        let builder = rayon::ThreadPoolBuilder::new().start_handler(move |_| {
            output::scope::set(Some(output_handle.clone()));
            time::scope::set(Some(scheduler_handle.clone()));
            scope::set(Some(handle.clone()));
        });

        let builder = if !args.non_deterministic {
            builder.num_threads(1)
        } else {
            builder
        };

        let pool = builder.build().expect("could not build thread pool");

        Self {
            output,
            scheduler,
            pool,
            is_ready: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Environment for Env {
    fn run<Tasks, F>(&mut self, tasks: Tasks) -> Poll<()>
    where
        Tasks: Iterator<Item = F> + Send,
        F: 'static + FnOnce() -> Poll<()> + Send,
    {
        let is_ready = self.is_ready.clone();

        self.pool.scope(move |s| {
            for task in tasks {
                let is_ready = is_ready.clone();
                s.spawn(move |_| is_ready.store(task().is_ready(), Ordering::Relaxed));
            }
        });

        if self.is_ready.swap(false, Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
