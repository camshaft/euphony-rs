use super::Buffer;
use crate::frame::Frame;
use core::marker::PhantomData;

pub trait BufferExt: Buffer {
    #[inline]
    fn map<M, F>(&mut self, map: M) -> Map<Self, M, F>
    where
        M: FnMut(F) -> Self::Frame,
        F: Frame,
    {
        Map {
            buffer: self,
            map,
            f: PhantomData,
        }
    }
}

impl<B: Buffer> BufferExt for B {}

impl<'a, F: Frame> Buffer for &'a mut [F] {
    type Frame = F;

    #[inline]
    fn fill<Fn: FnMut(usize) -> Self::Frame>(&mut self, mut f: Fn) {
        for (idx, frame) in self.iter_mut().enumerate() {
            *frame = f(idx);
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self[..].len()
    }
}

pub struct Map<'a, B, M, F> {
    buffer: &'a mut B,
    map: M,
    f: PhantomData<F>,
}

impl<'a, B, M, F> Buffer for Map<'a, B, M, F>
where
    B: Buffer,
    M: FnMut(F) -> B::Frame,
    F: Frame,
{
    type Frame = F;

    #[inline]
    fn fill<Fn: FnMut(usize) -> Self::Frame>(&mut self, mut f: Fn) {
        let map = &mut self.map;
        self.buffer.fill(|idx| map(f(idx)))
    }

    #[inline]
    fn len(&self) -> usize {
        self.buffer.len()
    }
}
