//! Durable render jobs and the one contract local and remote workers share.
//!
//! The scheduler orders render work but doesn't serialize it: each worker
//! renders as many attempts at once as its discovered window allows, and a
//! result becomes playable only after the controller validates and
//! publishes it.

mod protocol;
mod scheduler;
mod window;
mod worker;

pub use protocol::{read_frame, write_frame, Assignment, AttemptId, FromWorker, ToWorker, WorkerId};
pub use scheduler::{Completion, Rejection, Scheduler, SchedulerError, Stats};
pub use window::Window;
pub use worker::perform;
