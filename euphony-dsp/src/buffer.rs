use crate::prelude::*;

#[derive(Debug, Default, Node)]
#[node(id = 500, module = "buffer")]
#[buffer(buffer)]
#[input(repeat, trigger = set_repeat)]
#[input(reset, trigger = set_reset)]
/// Plays a buffer
pub struct Play {
    position: usize,
    repeat: bool,
}

impl Play {
    fn set_repeat(&mut self, value: f64) {
        self.repeat = value >= 1.0;
    }

    fn set_reset(&mut self, _value: f64) {
        self.position = 0;
    }

    #[inline]
    pub fn render(&mut self, buffer: Buffer, mut output: &mut [Sample]) {
        loop {
            let mut position = self.position;
            let samples = buffer.samples.get(position..).unwrap_or(&[][..]);
            for (from, to) in (samples, output.iter_mut()).zip() {
                *to = *from;
                position += 1;
            }
            self.position = position;

            if samples.len() < output.len() {
                output = &mut output[samples.len()..];
                if self.repeat && !buffer.samples.is_empty() {
                    self.position = 0;
                    continue;
                }

                output.fill(0.0);
            }

            break;
        }
    }
}
