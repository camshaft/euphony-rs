use bach::scope::define;
use euphony_units::pitch::{mode::Mode, tuning::Tuning};

pub mod tuning {
    use super::*;
    pub use euphony_units::pitch::tuning::*;

    define!(scope, Tuning);

    pub use scope::*;
}

pub fn tuning() -> Tuning {
    tuning::try_borrow_with(|t| t.unwrap_or(tuning::western::ET12))
}

pub mod mode {
    use super::*;
    pub use euphony_units::pitch::mode::*;

    define!(scope, Mode);

    pub use scope::*;
}

pub fn mode() -> Mode {
    mode::try_borrow_with(|t| t.unwrap_or(mode::western::MAJOR))
}
