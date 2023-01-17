use crate::prelude::*;

#[derive(Debug, Node)]
#[node(id = 200, module = "env")]
#[input(target, default = 0.0)]
#[input(duration, default = 0.01)]
#[input(value, default = 0.0, trigger = set_value)]
pub struct Linear {
    value: f64,
    target: f64,
    duration: f64,
    step: f64,
    samples: usize,
}

impl Default for Linear {
    fn default() -> Self {
        Self {
            value: 0.0,
            target: core::f64::NAN,
            duration: core::f64::NAN,
            step: 0.0,
            samples: 0,
        }
    }
}

impl Linear {
    #[inline]
    fn set_value(&mut self, value: f64) {
        self.value = value;
        self.update();
    }

    #[inline]
    fn set_target(&mut self, value: f64) {
        if self.target == value {
            return;
        }

        self.target = value;
        self.update();
    }

    #[inline]
    fn set_duration(&mut self, value: f64) {
        if self.duration == value {
            return;
        }

        self.duration = value;
        self.update();
    }

    #[inline]
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
    fn next(&mut self) -> Sample {
        let frame = self.value;

        if let Some(samples) = self.samples.checked_sub(1) {
            self.samples = samples;
            self.value += self.step;
        } else {
            self.value = self.target;
        }

        frame
    }

    #[inline]
    pub fn render(&mut self, target: Input, duration: Input, output: &mut [f64]) {
        match (duration, target) {
            (Input::Constant(duration), Input::Constant(target)) => {
                self.set_duration(duration);
                self.set_target(target);
                for output in output {
                    *output = self.next();
                }
            }
            (Input::Constant(duration), target) => {
                self.set_duration(duration);
                for (target, frame) in (target, output).zip() {
                    self.set_target(target);
                    *frame = self.next();
                }
            }
            (duration, Input::Constant(target)) => {
                self.set_target(target);
                for (duration, frame) in (duration, output).zip() {
                    self.set_duration(duration);
                    *frame = self.next();
                }
            }
            _ => {
                for (target, duration, output) in (target, duration, output).zip() {
                    let mut needs_update = false;

                    if self.duration != duration {
                        self.duration = duration;
                        needs_update = true;
                    }
                    if self.target != target {
                        self.target = target;
                        needs_update = true;
                    }

                    if needs_update {
                        self.update();
                    }

                    *output = self.next();
                }
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

        let mut out = [0.0; 512];
        env.render(1.0.into(), 0.01.into(), &mut out[..]);
        assert_eq!(out[0], 0.0);
        assert_eq!(out[511], 1.0);
        assert_ne!(out[1], 0.0);

        env.render(0.0.into(), 0.01.into(), &mut out[..]);
        assert_eq!(out[0], 1.0);
        assert_eq!(out[511], 0.0);
    }
}
