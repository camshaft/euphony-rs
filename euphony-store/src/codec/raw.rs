use crate::{codec, dc::LeakDc, storage::Storage};
use core::fmt;
use euphony_compiler::sample::{self, DefaultSample as Sample};
use euphony_node::{SampleType, Sink};

pub struct Writer<O: codec::Output> {
    leak_dc: LeakDc,
    outputs: [O; 4],
    sink: O,
}

impl<W: codec::Output> fmt::Debug for Writer<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawWriter").finish()
    }
}

impl<O: codec::Output> codec::Codec<O> for Writer<O> {
    fn new<S: Storage<Output = O>>(storage: &mut S, sink: O) -> Self {
        Self {
            leak_dc: Default::default(),
            outputs: [
                storage.create(),
                storage.create(),
                storage.create(),
                storage.create(),
            ],
            sink,
        }
    }
}

impl<W: codec::Output> Sink for Writer<W> {
    #[inline]
    fn advance(&mut self, _: u64) {
        // no-op
    }

    #[inline]
    fn write(&mut self, ty: SampleType, samples: &[Sample]) {
        if ty == SampleType::Pcm {
            for sample in samples.iter().copied() {
                let sample = self.leak_dc.apply(sample);
                let sample = sample.to_bits().to_ne_bytes();
                self.outputs[0].write(&sample);
            }
        } else {
            for sample in samples.iter().copied() {
                let sample: i16 = sample::Sample::to_sample(sample);
                self.outputs[ty as u8 as usize].write(&sample.to_le_bytes());
            }
        }
    }

    #[inline]
    fn write_const(&mut self, ty: SampleType, sample: Sample, count: usize) {
        if ty == SampleType::Pcm {
            for _ in 0..count {
                let sample = self.leak_dc.apply(sample);
                let sample = sample.to_bits().to_ne_bytes();
                self.outputs[0].write(&sample);
            }
        } else {
            let sample: i16 = sample::Sample::to_sample(sample);
            let sample = sample.to_le_bytes();
            for _ in 0..count {
                self.outputs[ty as u8 as usize].write(&sample);
            }
        }
    }
}

impl<W: codec::Output> Drop for Writer<W> {
    fn drop(&mut self) {
        let a = self.outputs[0].finish();
        self.sink.write(&a);
        let b = self.outputs[1].finish();
        self.sink.write(&b);
        let c = self.outputs[2].finish();
        self.sink.write(&c);
        let d = self.outputs[3].finish();
        self.sink.write(&d);
    }
}
