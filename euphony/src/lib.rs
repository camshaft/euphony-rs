pub use euphony_core::*;
pub use euphony_macros::*;
pub use euphony_sc;

mod args;
pub mod ext;
pub mod runtime;

#[macro_export]
macro_rules! include_synthdef {
    ($($args:tt)*) => {
        use $crate::euphony_sc;
        $crate::euphony_sc::include_synthdef!($($args)*);
    };
}

pub mod prelude {
    pub use crate::{
        ext::*,
        include_synthdef,
        pitch::Interval,
        runtime::{output::track, time::scheduler},
        time::{Beat, Tempo},
    };
}
