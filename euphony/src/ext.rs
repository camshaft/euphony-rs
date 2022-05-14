pub use crate::rand::Ext as RandExt;
use crate::units::time::Beat;

pub trait DelayExt {
    fn delay(self) -> crate::time::Timer;
}

impl DelayExt for Beat {
    fn delay(self) -> crate::time::Timer {
        crate::time::delay(self)
    }
}

pub trait SpawnExt {
    type Output;

    fn spawn(self) -> crate::runtime::JoinHandle<Self::Output>;
    fn spawn_primary(self) -> crate::runtime::JoinHandle<Self::Output>;
}

impl<F: 'static + core::future::Future + Send> SpawnExt for F
where
    F::Output: 'static + Send,
{
    type Output = F::Output;

    fn spawn(self) -> crate::runtime::JoinHandle<Self::Output> {
        crate::runtime::spawn(self)
    }

    fn spawn_primary(self) -> crate::runtime::JoinHandle<Self::Output> {
        crate::runtime::primary::spawn(self)
    }
}
