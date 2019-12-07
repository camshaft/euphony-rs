use crate::pitch::mode::intervals::ModeIntervals;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModeSystem(&'static [ModeIntervals]);

impl ModeSystem {
    pub const fn new(intervals: &'static [ModeIntervals]) -> Self {
        Self(intervals)
    }
}

impl core::ops::Deref for ModeSystem {
    type Target = [ModeIntervals];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[macro_export]
macro_rules! mode_system {
    ($([$($step:expr),* $(,)?]),* $(,)?) => {
        $crate::pitch::mode::system::ModeSystem::new(&[
            $(
                $crate::mode_intervals!($($step),*)
            ),*
        ])
    };
}

#[macro_export]
macro_rules! mode_system_rotation {
    ($($step:expr),*) => {
        $crate::mode_system_rotation!(@__next, [$($step),*], [], []);
    };
    (@__next, [$next:expr $(, $head:expr)*], [$($tail:expr,)*], [$($mode:expr,)*]) => {
        $crate::mode_system_rotation!(
            @__next,
            [$($head),*],
            [$($tail,)* $next,],
            [$($mode,)* $crate::mode_intervals!([$next, $($head,)* $($tail,)*]),]
        );
    };
    (@__next, [], [$($tail:expr,)*], [$($mode:expr,)*]) => {
        $crate::pitch::mode::system::ModeSystem::new(&[$($mode),*])
    }
}
