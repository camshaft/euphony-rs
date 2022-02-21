use crate::{
    buffer::{Batch, Buffer},
    frame::Frame,
    signal::Signal,
};
use core::marker::PhantomData;

pub fn silence<F: Frame>() -> Silence<F> {
    Silence { frame: PhantomData }
}

pub struct Silence<F> {
    frame: PhantomData<F>,
}

impl<F: Frame> Signal for Silence<F> {
    type Frame = F;

    #[inline]
    fn fill<Bat: Batch>(&mut self, buffer: &mut Buffer<Self::Frame>) {
        unsafe {
            unsafe_assert!(buffer.len() == Bat::LEN);
        }
        for frame in buffer.iter_mut() {
            frame.write(F::EQUILIBRIUM);
        }
    }

    #[inline]
    fn remaining(&self) -> Option<u64> {
        None
    }
}

#[cfg(test)]
mod tests {
    generator_test!(silence, |buffer| {
        let mut silence = silence();
        silence.fill::<TestBatch>(buffer);
    });
}
