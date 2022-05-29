use crate::prelude::*;
pub use crate::processors::buffer::*;
use bach::executor::JoinHandle;
use euphony_buffer::AsChannel;
pub use euphony_buffer::Buffer;

pub trait BufferExt {
    fn play(&self) -> JoinHandle<()>;
}

impl<T> BufferExt for T
where
    for<'a> &'a T: AsChannel,
{
    fn play(&self) -> JoinHandle<()> {
        let duration = self.duration();
        let play = play().with_buffer(self).sink();
        async move {
            // TODO delay for duration
            let _ = duration;
            Beat(8, 1).delay().await;
            play.fin();
        }
        .spawn_primary()
    }
}
