use crate::time::Beat;

pub trait DelayExt {
    fn delay(self) -> crate::runtime::time::Timer;
}

impl DelayExt for Beat {
    fn delay(self) -> crate::runtime::time::Timer {
        crate::runtime::time::scheduler().delay(self)
    }
}
