pub use crate::{
    buffer::{self, Buffer, BufferExt},
    cell::Cell,
    ext::*,
    group::*,
    processor::Processor,
    processors::{
        ext::*,
        input::{self, *},
        *,
    },
    rand,
    runtime::{primary, spawn},
    section::section,
    time::{now, set_tempo, tempo},
    units::{
        pitch::{
            frequency::*,
            mode::{self, Mode},
            tuning, Interval,
        },
        time::{Beat, Tempo},
        zip::Zip as ZipExt,
    },
};
pub use euphony_samples as samples;
pub use futures::{FutureExt, StreamExt};

#[macro_export]
macro_rules! delay {
    ($n:literal) => {
        $crate::prelude::delay!($crate::prelude::Beat($n, 1))
    };
    ($n:literal / $d:literal) => {
        $crate::prelude::delay!($crate::prelude::Beat($n, $d))
    };
    ($expr:expr) => {
        $crate::prelude::DelayExt::delay($expr).await
    };
}
pub use delay;

pub mod western {
    pub use super::{mode::western::*, tuning::western::*};
}
