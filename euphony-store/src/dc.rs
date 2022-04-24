/// <https://doc.sccode.org/Classes/LeakDC.html>
pub struct LeakDc {
    x1: f64,
    y1: f64,
    coef: f64,
}

impl Default for LeakDc {
    fn default() -> Self {
        Self::new(0.995)
    }
}

impl LeakDc {
    #[inline]
    pub fn new(coef: f64) -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            coef,
        }
    }

    /// `y[n] = x[n] - x[n-1] + coef * y[n-1]`
    #[inline]
    pub fn apply(&mut self, sample: f64) -> f64 {
        let x = sample;
        let y0 = x - self.x1 + self.coef * self.y1;
        self.x1 = x;
        self.y1 = y0;
        y0
    }
}

// #[test]
// fn average_test() {
//     let mut leak = LeakDc::default();
//     let sample = 0.9;
//     let mut total = sample;
//     for _ in 0..48000 {
//         total += leak.apply(sample);
//     }
//     panic!("{:?}", total);
//     assert!(total / 48000.0 < 0.001);
// }
