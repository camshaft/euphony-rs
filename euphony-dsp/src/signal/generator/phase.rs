use crate::sample::{self, Sample};

#[derive(Debug, Clone, Copy, Default)]
pub struct Phase<T>(T);

impl Phase<f32> {
    #[inline]
    pub fn set(&mut self, phase: f32) {
        self.0 = phase.fract();
    }

    #[inline(always)]
    pub fn next<R: sample::Rate>(&mut self, freq: f32) -> f32 {
        let value = self.0;
        unsafe {
            unsafe_assert!(!value.is_nan());
            unsafe_assert!(value.is_finite());
            unsafe_assert!((0.0..=1.0).contains(&value));
        }
        self.0 = (value + R::PERIOD * freq).fract();
        value
    }
}

impl Phase<i32> {
    pub fn set(&mut self, phase: i32) {
        self.0 = phase;
    }

    #[inline]
    pub fn next<R: sample::Rate>(&mut self, freq: f32) -> i32 {
        let value = self.0;
        self.0 = i32::from_sample(f32::from_sample(value) + R::PERIOD * freq);
        value
    }
}

macro_rules! phased_generator {
    ($lower:ident, $upper:ident, | $phase:ident : $ty:ty | $f:expr) => {
        pub fn $lower<Freq>(freq: Freq) -> $upper<Freq>
        where
            Freq: crate::Signal,
            Freq::Frame: crate::Frame<Sample = f32, NumChannels = crate::frame::N1>,
        {
            $upper {
                freq,
                phase: super::Phase::default(),
            }
        }

        pub struct $upper<Freq>
        where
            Freq: crate::Signal,
            Freq::Frame: crate::Frame<Sample = f32, NumChannels = crate::frame::N1>,
        {
            freq: Freq,
            phase: super::Phase<$ty>,
        }

        impl<Freq> $upper<Freq>
        where
            Freq: crate::Signal,
            Freq::Frame: crate::Frame<Sample = f32, NumChannels = crate::frame::N1>,
        {
            #[inline]
            pub fn phase(mut self, phase: $ty) -> Self {
                self.phase.set(phase);
                self
            }
        }

        impl<Freq> crate::Signal for $upper<Freq>
        where
            Freq: crate::Signal,
            Freq::Frame: crate::Frame<Sample = f32, NumChannels = crate::frame::N1>,
        {
            type Frame = $ty;

            #[inline]
            fn fill<Bat: crate::buffer::Batch>(
                &mut self,
                buffer: &mut crate::buffer::Buffer<Self::Frame>,
            ) {
                unsafe {
                    unsafe_assert!(buffer.len() == Bat::LEN);
                }
                use crate::Frame;
                Bat::buffer::<Freq::Frame, _, _>(
                    |child| {
                        self.freq.fill::<Bat>(child);
                    },
                    |freq| {
                        unsafe {
                            unsafe_assert!(buffer.len() == freq.len());
                        }
                        for (frame, freq) in buffer.iter_mut().zip(freq.iter()) {
                            let freq = unsafe { *freq.channel_unchecked(0) };
                            let $phase = self.phase.next::<Bat::SampleRate>(freq);
                            frame.write($f);
                        }
                    },
                );
            }

            #[inline]
            fn remaining(&self) -> Option<u64> {
                None
            }
        }
    };
}
