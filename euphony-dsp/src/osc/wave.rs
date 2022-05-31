use crate::{fun::math::lerp, prelude::*};

#[derive(Debug, Clone, Copy, Default, Node)]
#[node(id = 114, module = "osc")]
#[input(frequency, default = 440.0)]
#[input(phase, trigger = set_phase)]
#[buffer(buffer)]
pub struct Wave {
    phase: super::Phase,
}

impl Wave {
    #[inline]
    pub fn set_phase(&mut self, phase: f64) {
        self.phase.set(phase);
    }

    #[inline]
    pub fn render(&mut self, frequency: Input, buffer: Buffer, output: &mut [Sample]) {
        if buffer.samples.is_empty() {
            output.fill(0.0);
            return;
        }

        let len = buffer.samples.len() as f64;

        for (freq, output) in (frequency, output.iter_mut()).zip() {
            let phase = self.phase.next(freq);
            // interpolate the phase onto the buffer's len
            let position = phase * len;
            *output = lerp_buffer(buffer.samples, position);
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Node)]
#[node(id = 115, module = "osc")]
#[input(frequency, default = 440.0)]
#[input(position, default = 0.0)]
#[input(phase, trigger = set_phase)]
#[buffer(a)]
#[buffer(b)]
pub struct Morph {
    phase: super::Phase,
}

impl Morph {
    #[inline]
    pub fn set_phase(&mut self, phase: f64) {
        self.phase.set(phase);
    }

    #[inline]
    pub fn render(
        &mut self,
        frequency: Input,
        position: Input,
        a: Buffer,
        b: Buffer,
        output: &mut [Sample],
    ) {
        if a.samples.is_empty() && b.samples.is_empty() {
            output.fill(0.0);
            return;
        }

        let a = if a.samples.is_empty() {
            &[0.0]
        } else {
            a.samples
        };

        let b = if b.samples.is_empty() {
            &[0.0]
        } else {
            b.samples
        };

        let a_len = a.len() as f64;
        let b_len = b.len() as f64;

        for (freq, position, output) in (frequency, position, output.iter_mut()).zip() {
            let phase = self.phase.next(freq);
            let position = position.fract();
            // interpolate the phase onto the buffer's len
            let a_position = phase * a_len;
            let b_position = phase * b_len;
            let a_value = lerp_buffer(a, a_position);
            let b_value = lerp_buffer(b, b_position);

            *output = (a_value * (1.0 - position)) + (b_value * position);
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Node)]
#[node(id = 317, module = "filter")]
#[input(signal)]
#[buffer(buffer)]
pub struct Shaper;

impl Shaper {
    #[inline]
    pub fn render(&mut self, signal: Input, buffer: Buffer, output: &mut [Sample]) {
        if buffer.samples.is_empty() {
            output.fill(0.0);
            return;
        }

        let len = buffer.samples.len() as f64;

        for (t, output) in (signal, output.iter_mut()).zip() {
            // interpolate the signal onto the buffer's len
            let t = (t * 0.5 + 0.5).fract();
            let position = t * len;
            *output = lerp_buffer(buffer.samples, position);
        }
    }
}

#[inline(always)]
fn lerp_buffer(buffer: &[Sample], position: f64) -> Sample {
    // round down to the nearest index
    let index = position as usize;

    // get index and index+1 wrapped to the beginning
    let a = *unsafe { buffer.get_unchecked(index) };
    let b = *if let Some(s) = buffer.get(index + 1) {
        s
    } else {
        unsafe { buffer.get_unchecked(0) }
    };

    // interpolate between the two indexes
    let fract = position.fract();
    lerp(a, b, fract)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wave_test() {
        let mut osc = Wave::new();
        let mut out = [0.0; 500];
        let freq = 480.0.into();
        let buffer = (&[-1.0, 1.0][..]).into();
        osc.render(freq, buffer, &mut out);
        eprintln!("{:?}", out);
        // panic!();
    }
}
