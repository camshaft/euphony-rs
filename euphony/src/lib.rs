pub use euphony_macros::main;
pub use euphony_units as units;

#[macro_use]
mod processor;

mod args;
pub mod ext;
pub mod group;
mod node;
mod output;
pub mod parameter;
pub mod rand;
pub mod runtime;
pub mod section;
pub mod set;
mod sink;
pub mod time;

mod processors;

pub mod prelude {
    pub use crate::{
        ext::*,
        group::*,
        processor::Processor,
        processors::{ext::*, input::*, *},
        rand,
        runtime::{primary, spawn},
        section::section,
        time::{now, set_tempo, tempo},
        units::{
            pitch::{frequency::*, mode, tuning, Interval},
            time::{Beat, Tempo},
        },
    };
    pub use futures::{FutureExt, StreamExt};
}

#[cfg(test)]
mod tests;
