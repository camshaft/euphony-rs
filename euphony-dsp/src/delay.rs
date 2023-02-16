use crate::prelude::*;
use euphony_node::LEN as BATCH_LEN;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

#[derive(Debug, Default)]
struct Buffer {
    samples: VecDeque<Sample>,
    epoch: u64,
}

impl Buffer {
    #[inline]
    fn write(&mut self, sample: Sample, delay: Sample, idx: usize) {
        let sample_delay = (delay * Rate::VALUE).round() as usize;
        let sample_delay = sample_delay + idx;

        if self.samples.len() <= sample_delay {
            self.samples.resize(sample_delay + 1, 0.0);
        }

        self.samples[sample_delay] += sample;
    }

    #[inline]
    fn read(&self, output: &mut [Sample], epoch: u64, last_offset: usize) {
        let len;

        if self.epoch >= epoch {
            // non-feedback delay
            len = output.len().min(self.samples.len());

            for (sample, output) in (self.samples.iter(), output.iter_mut()).zip() {
                *output = *sample;
            }
        } else {
            // feedback delay
            len = output
                .len()
                .min(self.samples.len().saturating_sub(BATCH_LEN));

            for (sample, output) in (self.samples.iter().skip(last_offset), output.iter_mut()).zip()
            {
                *output = *sample;
            }
        };

        for output in &mut output[len..] {
            *output = 0.0;
        }
    }
}

#[derive(Debug, Default, Node)]
#[node(id = 250, module = "delay", fork = "feedback")]
#[input(signal, default = 0.0)]
#[input(delay, default = 0.0)]
pub struct Bus {
    buffer: Arc<RwLock<Buffer>>,
    last_len: usize,
}

impl Bus {
    #[inline]
    pub fn render(&mut self, signal: Input, delay: Input, output: &mut [Sample]) {
        let mut buffer = self.buffer.write().unwrap();

        buffer.epoch += 1;
        let drain_len = self.last_len.min(buffer.samples.len());

        buffer.samples.drain(..drain_len);
        self.last_len = output.len();

        for (idx, (signal, delay)) in (signal, delay).zip().take(output.len()).enumerate() {
            buffer.write(signal, delay, idx);
        }

        let epoch = buffer.epoch;
        buffer.read(output, epoch, 0);
    }

    #[inline]
    pub fn fork_node(&self) -> Option<euphony_node::BoxProcessor> {
        let buffer = self.buffer.clone();
        let epoch = buffer.read().unwrap().epoch;
        let reader = Reader {
            buffer,
            epoch,
            last_offset: 0,
        };
        Some(euphony_node::spawn(reader))
    }
}

#[derive(Debug, Default)]
struct Reader {
    buffer: Arc<RwLock<Buffer>>,
    epoch: u64,
    last_offset: usize,
}

impl Reader {
    #[inline]
    pub fn render(&mut self, output: &mut [Sample]) {
        let buffer = self.buffer.read().unwrap();
        self.epoch += 1;
        buffer.read(output, self.epoch, self.last_offset);
        self.last_offset = output.len();
    }
}

impl euphony_node::Node<0, 0> for Reader {
    #[inline]
    fn process(
        &mut self,
        _inputs: euphony_node::Inputs<0>,
        _buffers: euphony_node::Buffers<0>,
        output: &mut [Sample],
    ) {
        self.render(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use euphony_node::LEN;

    #[test]
    fn delay_test() {
        let mut sine = crate::osc::Sine::default();
        let mut signal = [0.0; LEN];
        sine.render(440.0.into(), &mut signal);

        for offset in [0, 1, 10] {
            let mut write = Bus::default();
            let mut before = Reader {
                buffer: write.buffer.clone(),
                ..Default::default()
            };
            let mut after = Reader {
                buffer: write.buffer.clone(),
                ..Default::default()
            };

            // render all of the delays; they should all be silent for LEN samples
            let mut before_out = [0.0; LEN];
            before.render(&mut before_out);
            assert_eq!(before_out, [0.0; LEN]);

            let delay = (LEN + offset) as f64 / Rate::VALUE;
            let mut output = [0.0; LEN];
            write.render((&signal).into(), delay.into(), &mut output);
            assert_eq!(output, [0.0; LEN]);

            let mut after_out = [0.0; LEN];
            after.render(&mut after_out);
            assert_eq!(after_out, [0.0; LEN]);

            // render the delays again; all of them should replicate the input signal
            before.render(&mut before_out);
            assert_eq!(&before_out[offset..], &signal[..LEN - offset]);

            write.render((&signal).into(), delay.into(), &mut output);
            assert_eq!(&output[offset..], &signal[..LEN - offset]);

            after.render(&mut after_out);
            assert_eq!(&after_out[offset..], &signal[..LEN - offset]);
        }
    }
}
