pub use crate::{
    buffer::{self, Buffer, BufferExt},
    cell::Cell,
    ext::*,
    group::*,
    midi,
    parameter::{Buffer as BufferParameter, Parameter, Trigger},
    pitch::{
        mode::{self, Mode},
        tuning,
    },
    processor::Processor,
    processors::{
        ext::*,
        input::{self, *},
        *,
    },
    rand,
    runtime::{primary, spawn},
    section::section,
    sink::Sink,
    time::{now, set_tempo, tempo},
    units::{
        pitch::{frequency::*, Interval},
        time::{Beat, Tempo},
        zip::Zip as ZipExt,
    },
    value,
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
    ($n:literal / $d:ident) => {
        $crate::prelude::delay!($crate::prelude::Beat($n, $d))
    };
    ($n:ident / $d:literal) => {
        $crate::prelude::delay!($crate::prelude::Beat($n, $d))
    };
    ($n:ident / $d:ident) => {
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
