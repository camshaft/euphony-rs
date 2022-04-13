use crate::{frame::Frame, sample};
// mod ext;
// pub use ext::BufferExt;
use core::{marker::PhantomData, mem::MaybeUninit};

pub type Buffer<F> = [MaybeUninit<F>];
#[cfg(test)]
pub type TestBatch = ArrayBatch<sample::Rate48000, 1024>;

pub trait Batch {
    type SampleRate: sample::Rate;
    const LEN: usize;

    fn buffer<Child, Init, Finish>(init: Init, finish: Finish)
    where
        Child: Frame,
        Init: FnOnce(&mut Buffer<Child>),
        Finish: FnOnce(&[Child]);
}

pub struct ArrayBatch<S: sample::Rate, const LEN: usize>(PhantomData<S>);

impl<S: sample::Rate, const LEN: usize> Batch for ArrayBatch<S, LEN> {
    type SampleRate = S;

    const LEN: usize = LEN;

    fn buffer<Child, Init, Finish>(init: Init, finish: Finish)
    where
        Child: Frame,
        Init: FnOnce(&mut Buffer<Child>),
        Finish: FnOnce(&[Child]),
    {
        let mut buffer = [MaybeUninit::uninit(); LEN];
        init(&mut buffer);

        let buffer = unsafe {
            // TODO use MaybeUninit::assume_init_array when stable
            let mut out = [Child::EQUILIBRIUM; LEN];

            for (out, from) in out.iter_mut().zip(buffer.iter()) {
                *out = from.assume_init();
            }

            out
        };
        finish(&buffer)
    }
}
