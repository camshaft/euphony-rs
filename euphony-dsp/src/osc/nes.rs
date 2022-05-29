use crate::prelude::*;
use core::num::Wrapping;

#[derive(Copy, Clone, Debug, Default)]
pub struct Phase(Wrapping<u16>);

impl Phase {
    #[inline]
    pub fn set(&mut self, phase: f64) {
        self.0 = Wrapping((phase.fract().abs() * u16::MAX as f64) as u16);
    }

    #[inline(always)]
    fn next(&mut self, freq: f64) -> u16 {
        let value = self.0;
        let step = (Rate::PERIOD * u16::MAX as f64 * freq) as u16;
        self.0 += step.max(1);
        value.0
    }
}

#[derive(Debug, Clone, Copy, Default, Node)]
#[node(id = 108, module = "osc::nes")]
#[input(frequency, default = 440.0)]
#[input(duty_cycle, default = 0.0)]
#[input(decay, default = 0.001)]
#[input(phase, trigger = set_phase)]
pub struct Pulse {
    phase: Phase,
    is_positive: bool,
    value: f64,
}

// From http://wiki.nesdev.com/w/index.php/APU_Pulse
const SQUARE_DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

impl Pulse {
    #[inline]
    pub fn set_phase(&mut self, phase: f64) {
        self.phase.set(phase);
    }

    #[inline(always)]
    fn next(&mut self, freq: f64, duty_cycle: f64, decay: f64) -> f64 {
        let sample = self.phase.next(freq);

        if sample < 8 {
            return f64::EQUILIBRIUM;
        }

        let mut duty_cycle = duty_cycle as usize;
        duty_cycle %= SQUARE_DUTY_TABLE.len();
        let duty_cycle = SQUARE_DUTY_TABLE[duty_cycle];

        let counter = sample as usize / (u16::MAX as usize / 8 + 1);
        let value = duty_cycle[counter];

        let is_positive = value == 1;
        // update the direction
        if self.is_positive != is_positive {
            self.is_positive = is_positive;
            self.value = if is_positive { 1.0 } else { -1.0 };
        }

        let value = self.value;
        let diff = 0.0 - value;
        let samples = (decay * Rate::VALUE).round();
        let step = diff / samples;

        if value.abs() > step {
            self.value += step;
        } else {
            self.value = 0.0;
        }

        value
    }

    #[inline]
    pub fn render(
        &mut self,
        frequency: Input,
        duty_cycle: Input,
        decay: Input,
        output: &mut [Sample],
    ) {
        for (freq, duty_cycle, decay, frame) in
            (frequency, duty_cycle, decay, output.iter_mut()).zip()
        {
            *frame = self.next(freq, duty_cycle, decay);
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Node)]
#[node(id = 109, module = "osc::nes")]
#[input(frequency, default = 440.0)]
#[input(phase, trigger = set_phase)]
pub struct Triangle {
    phase: super::Phase,
}

impl Triangle {
    #[inline]
    pub fn set_phase(&mut self, phase: f64) {
        self.phase.set(phase);
    }

    #[inline(always)]
    fn next(&mut self, freq: f64) -> f64 {
        const VAR_TABLE: [isize; 32] = {
            let mut table = [0; 32];

            let magnitude = 4;

            let mut idx = 0;
            while idx < 16 {
                let i = match idx {
                    0..=2 => -2,
                    3..=5 => -1,
                    10..=12 => 1,
                    13..=15 => 2,
                    _ => 0,
                } * magnitude;

                table[idx] += i;
                table[idx + 16] += i;
                idx += 1;
            }

            table
        };

        const TABLE: [f64; 512] = {
            let mut table = [0.0; 512];

            let mut idx = 0;
            let mut phase = 8;
            while idx < table.len() {
                let mut sample = phase;
                let entries = VAR_TABLE[sample as usize];
                let mut entries = (entries + (table.len() / 32) as isize) as usize;

                if sample & 0x10 == 0x10 {
                    sample ^= 0x1f;
                }

                let value = (sample as f64 - 7.5) / 7.5;

                while entries > 0 {
                    table[idx] = value;
                    idx += 1;
                    entries -= 1;
                }

                phase += 1;
                phase %= 32;
            }

            table
        };

        let phase = self.phase.next(freq);
        let idx = (phase * TABLE.len() as f64) as usize;
        TABLE[idx]
    }

    #[inline]
    pub fn render(&mut self, frequency: Input, output: &mut [Sample]) {
        for (freq, frame) in (frequency, output.iter_mut()).zip() {
            *frame = self.next(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulse_test() {
        let mut osc = Pulse::new();
        let mut samples = [0.0; 500];
        osc.render(440.0.into(), 2.0.into(), 0.001.into(), &mut samples);

        eprintln!("{:?}", samples);
        //panic!();
    }

    #[test]
    fn triangle_test() {
        let mut osc = Triangle::new();
        let mut samples = [0.0; 500];
        osc.render((480.0).into(), &mut samples);

        eprintln!("{:?}", samples);
        //panic!();
    }
}
