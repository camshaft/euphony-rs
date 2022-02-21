use crate::{
    buffer::{Batch, Buffer},
    frame::Frame,
    sample::{FromSample, Sample},
    signal::Signal,
};
use core::marker::PhantomData;

pub trait SignalExt: Signal + Sized {
    #[inline]
    fn add_signal<S, F>(self, add: S) -> AddAmp<Self, S>
    where
        S: Signal<Frame = F>,
        S::Frame: Frame<
            Sample = <<Self::Frame as Frame>::Sample as Sample>::Signed,
            NumChannels = <Self::Frame as Frame>::NumChannels,
        >,
    {
        AddAmp { signal: self, add }
    }

    #[inline]
    fn mul_signal<S, F>(self, mul: S) -> MulAmp<Self, S>
    where
        S: Signal<Frame = F>,
        S::Frame: Frame<
            Sample = <<Self::Frame as Frame>::Sample as Sample>::Float,
            NumChannels = <Self::Frame as Frame>::NumChannels,
        >,
    {
        MulAmp { signal: self, mul }
    }

    #[inline]
    fn inv_signal(self) -> Inv<Self> {
        Inv { signal: self }
    }

    #[inline]
    fn convert<Into: Frame>(self) -> Convert<Self, Into> {
        Convert {
            signal: self,
            into: PhantomData,
        }
    }
}

impl<S: Signal> SignalExt for S {}

pub struct AddAmp<A, B> {
    signal: A,
    add: B,
}

impl<A, B> Signal for AddAmp<A, B>
where
    A: Signal,
    B: Signal,
    B::Frame: Frame<
        Sample = <<A::Frame as Frame>::Sample as Sample>::Signed,
        NumChannels = <A::Frame as Frame>::NumChannels,
    >,
{
    type Frame = A::Frame;

    #[inline]
    fn fill<Bat: Batch>(&mut self, buffer: &mut Buffer<Self::Frame>) {
        unsafe {
            unsafe_assert!(buffer.len() == Bat::LEN);
        }
        self.signal.fill::<Bat>(buffer);

        Bat::buffer::<B::Frame, _, _>(
            |child| {
                self.add.fill::<Bat>(child);
            },
            |add| {
                unsafe {
                    unsafe_assert!(buffer.len() == add.len());
                }
                for (frame, add) in buffer.iter_mut().zip(add.iter().copied()) {
                    let frame = unsafe { frame.assume_init_mut() };
                    *frame = frame.add_amp(add);
                }
            },
        );
    }

    #[inline]
    fn remaining(&self) -> Option<u64> {
        match (self.signal.remaining(), self.add.remaining()) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), _) => Some(a),
            (_, Some(b)) => Some(b),
            _ => None,
        }
    }
}

pub struct MulAmp<A, B> {
    signal: A,
    mul: B,
}

impl<A, B> Signal for MulAmp<A, B>
where
    A: Signal,
    B: Signal,
    B::Frame: Frame<
        Sample = <<A::Frame as Frame>::Sample as Sample>::Float,
        NumChannels = <A::Frame as Frame>::NumChannels,
    >,
{
    type Frame = A::Frame;

    #[inline]
    fn fill<Bat: Batch>(&mut self, buffer: &mut Buffer<Self::Frame>) {
        unsafe {
            unsafe_assert!(buffer.len() == Bat::LEN);
        }
        self.signal.fill::<Bat>(buffer);

        Bat::buffer::<B::Frame, _, _>(
            |child| {
                self.mul.fill::<Bat>(child);
            },
            |mul| {
                unsafe {
                    unsafe_assert!(buffer.len() == mul.len());
                }
                for (frame, mul) in buffer.iter_mut().zip(mul.iter().copied()) {
                    let frame = unsafe { frame.assume_init_mut() };
                    *frame = frame.mul_amp(mul);
                }
            },
        );
    }

    #[inline]
    fn remaining(&self) -> Option<u64> {
        match (self.signal.remaining(), self.mul.remaining()) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), _) => Some(a),
            (_, Some(b)) => Some(b),
            _ => None,
        }
    }
}

pub struct Inv<S> {
    signal: S,
}

impl<S> Signal for Inv<S>
where
    S: Signal,
    <S::Frame as Frame>::Sample: FromSample<f32>,
{
    type Frame = S::Frame;

    #[inline]
    fn fill<Bat: Batch>(&mut self, buffer: &mut Buffer<Self::Frame>) {
        unsafe {
            unsafe_assert!(buffer.len() == Bat::LEN);
        }
        self.signal.fill::<Bat>(buffer);

        for frame in buffer.iter_mut() {
            let frame = unsafe { frame.assume_init_mut() };
            *frame = frame.map(|sample| sample.mul_amp(Sample::from_sample(-1.0)));
        }
    }

    #[inline]
    fn remaining(&self) -> Option<u64> {
        self.signal.remaining()
    }
}

pub struct Convert<S, F> {
    signal: S,
    into: PhantomData<F>,
}

impl<S, Into> Signal for Convert<S, Into>
where
    S: Signal,
    Into: Frame<NumChannels = <S::Frame as Frame>::NumChannels>,
    Into::Sample: FromSample<<S::Frame as Frame>::Sample>,
{
    type Frame = Into;

    #[inline]
    fn fill<Bat: Batch>(&mut self, buffer: &mut Buffer<Self::Frame>) {
        unsafe {
            unsafe_assert!(buffer.len() == Bat::LEN);
        }
        Bat::buffer(
            |buffer| {
                self.signal.fill::<Bat>(buffer);
            },
            |samples| {
                unsafe {
                    unsafe_assert!(buffer.len() == samples.len());
                }
                for (to, from) in buffer.iter_mut().zip(samples.iter().copied()) {
                    let value = from.map(Sample::from_sample);
                    to.write(value);
                }
            },
        );
    }

    #[inline]
    fn remaining(&self) -> Option<u64> {
        self.signal.remaining()
    }
}
