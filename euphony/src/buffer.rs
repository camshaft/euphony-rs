use crate::{parameter::Parameter, prelude::*};
use bach::executor::JoinHandle;
use core::{future::Future, time::Duration};
use euphony_buffer::AsChannel;

pub use crate::processors::buffer::*;
pub use euphony_buffer::Buffer;

pub trait BufferExt {
    fn play(&self) -> PlayBuf;
}

impl<T> BufferExt for T
where
    for<'a> &'a T: AsChannel,
{
    fn play(&self) -> PlayBuf {
        PlayBuf::new(self)
    }
}

pub struct PlayBuf {
    play: Play,
    duration: Duration,
}

impl PlayBuf {
    pub fn new<T>(t: &T) -> Self
    where
        for<'a> &'a T: AsChannel,
    {
        let duration = t.duration();
        let play = play().with_buffer(t);
        Self { duration, play }
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    #[inline]
    fn fut(&self) -> impl 'static + Future<Output = ()> + Send {
        let delay = self.delay();
        let sink = self.play.sink();
        async move {
            delay.await;
            sink.fin()
        }
    }
}

impl DelayExt for &PlayBuf {
    fn delay(self) -> crate::time::Timer {
        // TODO delay for the actual duration rather than a beat
        let beat = self.duration / tempo();
        let beat = beat.quantize(Beat(1, 64));
        beat.delay()
    }
}

impl Processor for PlayBuf {
    fn sink(&self) -> crate::sink::Sink {
        self.play.sink()
    }
}

impl From<PlayBuf> for Parameter {
    fn from(play: PlayBuf) -> Self {
        Parameter::from(play.play)
    }
}

impl From<&PlayBuf> for Parameter {
    fn from(play: &PlayBuf) -> Self {
        Parameter::from(&play.play)
    }
}

define_processor_ops!(PlayBuf);

impl SpawnExt for PlayBuf {
    type Output = ();

    fn spawn(self) -> JoinHandle<Self::Output> {
        self.fut().spawn()
    }

    fn spawn_primary(self) -> JoinHandle<Self::Output> {
        self.fut().spawn_primary()
    }
}
