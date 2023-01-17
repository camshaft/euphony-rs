use crate::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Default)]
struct Buffer {
    samples: VecDeque<Sample>,
}

impl Buffer {
    fn write(&mut self, sample: Sample, delay: Sample) {
        let sample_delay = (delay * Rate::VALUE).round() as usize;

        if self.samples.len() <= sample_delay {
            self.samples.resize(sample_delay + 1, 0.0);
        }

        self.samples[sample_delay] += sample;
    }

    fn next(&mut self) -> Sample {
        self.samples.pop_front().unwrap_or(0.0)
    }
}

#[derive(Debug, Default, Node)]
#[node(id = 250, module = "delay")]
#[input(signal, default = 0.0)]
#[input(delay, default = 0.0)]
pub struct Delay {
    buffer: Buffer,
}

impl Delay {
    #[inline]
    pub fn render(&mut self, signal: Input, delay: Input, output: &mut [Sample]) {
        for (signal, delay, output) in (signal, delay, output.iter_mut()).zip() {
            self.buffer.write(signal, delay);
            *output = self.buffer.next();
        }
    }
}

#[derive(Debug, Default, Node)]
#[node(id = 251, module = "delay")]
#[input(signal, default = 0.0)]
#[input(delay, default = 0.0)]
#[input(decay, default = 0.0)]
pub struct Feedback {
    buffer: Buffer,
}

impl Feedback {
    #[inline]
    pub fn render(&mut self, signal: Input, delay: Input, decay: Input, output: &mut [Sample]) {
        for (signal, delay, decay, output) in (signal, delay, decay, output.iter_mut()).zip() {
            *output = self.buffer.next();
            if decay != 0.0 {
                self.buffer.write(*output * decay + signal, delay);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use euphony_node::LEN;

    #[test]
    fn delay_test() {
        let mut delay = Delay::default();

        let mut sine = crate::osc::Sine::default();
        let mut signal = [0.0; LEN];
        sine.render(440.0.into(), &mut signal);

        let mut output = [0.0; LEN];
        delay.render((&signal).into(), (10.0 / Rate::VALUE).into(), &mut output);

        insta::assert_debug_snapshot!(&output[..20]);
    }

    #[test]
    fn feedback_test() {
        let mut delay = Feedback::default();

        let mut sine = crate::osc::Sine::default();
        let mut signal = [0.0; LEN];
        sine.render(440.0.into(), &mut signal);

        let mut output = [0.0; LEN];
        delay.render(
            (&signal).into(),
            (5.0 / Rate::VALUE).into(),
            2.0.into(),
            &mut output,
        );

        insta::assert_debug_snapshot!(&output[..50]);
    }
}
