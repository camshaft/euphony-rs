pub use euphony_core::*;
pub use euphony_macros::*;
pub use euphony_sc;

mod args;
pub mod ext;
pub mod runtime;

#[macro_export]
macro_rules! prelude {
    () => {
        #[cfg(euphony_assets)]
        include!(concat!(env!("OUT_DIR"), "/euphony_assets.rs"));

        use euphony::prelude::*;
    };
}

pub mod prelude {
    pub use crate::{
        ext::*,
        pitch::Interval,
        runtime::{output::track, spawn, time::scheduler},
        time::{Beat, Tempo},
    };
    pub use euphony_sc::{self, params, synthdef, track::Track};
}
