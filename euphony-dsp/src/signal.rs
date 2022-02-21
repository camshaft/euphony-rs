use crate::{
    buffer::{Batch, Buffer},
    frame::Frame,
};

mod ext;
pub use ext::SignalExt;
pub mod generator;

pub trait Signal {
    type Frame: Frame;

    /// Fills the provides slice of frames with samples
    fn fill<B: Batch>(&mut self, buffer: &mut Buffer<Self::Frame>);

    /// Returns the number of samples in this signal
    fn remaining(&self) -> Option<u64>;
}

impl<F: Frame> Signal for F {
    type Frame = F;

    #[inline]
    fn fill<B: Batch>(&mut self, buffer: &mut Buffer<Self::Frame>) {
        for frame in buffer.iter_mut() {
            frame.write(*self);
        }
    }

    #[inline]
    fn remaining(&self) -> Option<u64> {
        None
    }
}
