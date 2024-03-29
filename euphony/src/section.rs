use crate::{
    ext::{DelayExt, SpawnExt},
    time::Timer,
};
use bach::executor::JoinHandle;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub fn section<T: DelayExt>(time: T) -> Section {
    Section::new(time.delay())
}

pub struct Section {
    handles: Vec<JoinHandle<()>>,
    timer: Timer,
}

impl Section {
    pub fn new(timer: Timer) -> Self {
        Self {
            handles: vec![],
            timer,
        }
    }

    pub fn with<T: 'static + Future<Output = ()> + Send>(&mut self, task: T) -> &mut Self {
        self.handles.push(task.spawn());
        self
    }
}

impl Future for Section {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = Pin::new(&mut self.timer).poll(cx);
        if res.is_ready() {
            for handle in self.handles.drain(..) {
                handle.cancel();
            }
        }
        res
    }
}
