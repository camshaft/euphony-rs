pub use euphony_macros::*;
pub use euphony_units as units;

mod args;
pub mod ext;
pub mod output;
pub mod rand;
pub mod runtime;
pub mod section;
pub mod synth;

pub mod prelude {
    pub use crate::{
        ext::*,
        rand,
        runtime::{
            spawn, spawn_primary,
            time::{now, set_tempo, tempo},
        },
        section::section,
        units::{
            pitch::Interval,
            time::{Beat, Tempo},
        },
    };
    pub use futures::{FutureExt, StreamExt};
}

#[cfg(test)]
mod tests;
