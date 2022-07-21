use crate::prelude::*;

#[derive(Debug, Default, Node)]
#[node(id = 200, module = "env")]
#[input(target, default = 0.0, trigger = set_target)]
#[input(duration, default = 0.01, trigger = set_duration)]
#[input(value, default = 0.0, trigger = set_value)]
pub struct Linear {
    value: f64,
    target: f64,
    duration: f64,
    step: f64,
    samples: usize,
}

impl Linear {
    #[inline]
    fn set_value(&mut self, value: f64) {
        self.value = value;
        self.update();
    }

    #[inline]
    fn set_target(&mut self, value: f64) {
        self.target = value;
        self.update();
    }

    #[inline]
    fn set_duration(&mut self, value: f64) {
        self.duration = value;
        self.update();
    }

    fn update(&mut self) {
        let diff = self.target - self.value;
        let samples = (self.duration * Rate::VALUE).round();
        self.samples = samples as _;
        if self.samples > 0 {
            self.step = diff / samples;
        } else {
            self.step = 0.0;
        }
    }

    #[inline]
    pub fn render(&mut self, output: &mut [f64]) {
        for frame in output {
            *frame = self.value;
            if let Some(samples) = self.samples.checked_sub(1) {
                self.samples = samples;
                self.value += self.step;
            } else {
                self.value = self.target;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_test() {
        let mut env = Linear::new();
        env.set_target(1.0);

        let mut out = [0.0; 512];
        env.render(&mut out[..]);
        assert_eq!(out[0], 0.0);
        assert_eq!(out[511], 1.0);
        assert_ne!(out[1], 0.0);

        env.set_target(0.0);
        env.render(&mut out[..]);
        assert_eq!(out[0], 1.0);
        assert_eq!(out[511], 0.0);
    }
}
