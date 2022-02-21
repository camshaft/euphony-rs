use crate::frame::Frame;
// mod ext;
// pub use ext::BufferExt;
use core::mem::MaybeUninit;

pub type Buffer<F> = [MaybeUninit<F>];
#[cfg(test)]
pub type TestBatch = ArrayBatch<1024, { crate::sample::PERIOD_48000 }>;

pub trait Batch {
    const SAMPLE_PERIOD: f32;
    const SAMPLE_PERIOD_INT: u32;
    const LEN: usize;

    fn buffer<Child, Init, Finish>(init: Init, finish: Finish)
    where
        Child: Frame,
        Init: FnOnce(&mut Buffer<Child>),
        Finish: FnOnce(&[Child]);
}

pub struct ArrayBatch<const LEN: usize, const SAMPLE_PERIOD: u32>;

impl<const SAMPLE_PERIOD: u32, const LEN: usize> Batch for ArrayBatch<LEN, SAMPLE_PERIOD> {
    const LEN: usize = LEN;
    const SAMPLE_PERIOD: f32 = unsafe { core::mem::transmute(SAMPLE_PERIOD) };
    const SAMPLE_PERIOD_INT: u32 = unsafe { core::mem::transmute(SAMPLE_PERIOD) };

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
