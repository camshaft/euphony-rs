use crate::time::Beat;

pub trait DelayExt {
    fn delay(self) -> crate::runtime::time::Timer;
}

impl DelayExt for Beat {
    fn delay(self) -> crate::runtime::time::Timer {
        crate::runtime::time::scheduler().delay(self)
    }
}

pub trait RngPickExt {
    type Output;

    fn pick(self) -> Self::Output;
}

pub trait SpawnExt {
    type Output;

    fn spawn(self) -> crate::runtime::JoinHandle<Self::Output>;
}

impl<F: 'static + core::future::Future + Send> SpawnExt for F
where
    F::Output: 'static + Send,
{
    type Output = F::Output;

    fn spawn(self) -> crate::runtime::JoinHandle<Self::Output> {
        crate::runtime::spawn(self)
    }
}
