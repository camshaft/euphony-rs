use crate::{
    fun::math::{lerp, lerp11},
    prelude::*,
};

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
            *output = lerp_buffer(&buffer, position);
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

        for (signal, output) in (signal, output.iter_mut()).zip() {
            // interpolate the signal onto the buffer's len
            let position = lerp11(0.0, len, signal);
            *output = lerp_buffer(&buffer, position);
        }
    }
}

#[inline(always)]
fn lerp_buffer(buffer: &Buffer, position: f64) -> Sample {
    // round down to the nearest index
    let index = position as usize;

    // get index and index+1 wrapped to the beginning
    let a = *unsafe { buffer.samples.get_unchecked(index) };
    let b = *if let Some(s) = buffer.samples.get(index + 1) {
        s
    } else {
        unsafe { buffer.samples.get_unchecked(0) }
    };

    // interpolate between the two indexes
    let fract = position.fract();
    lerp(a, b, fract)
}
