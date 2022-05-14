use crate::{Sample, SpatialSample};
use euphony_units::coordinates::Cartesian;

#[derive(Clone, Copy, Debug, Default)]
pub struct Weights {
    w: f64,
    coordinate: Cartesian<Sample>,
}

impl Weights {
    pub fn new(coordinate: Cartesian<Sample>, directionality: f64) -> Self {
        let Cartesian {
            mut x,
            mut y,
            mut z,
        } = coordinate;

        let radius = (x * x + y * y + z * z).sqrt();
        let m = (1.0 - directionality) / radius;
        x *= m;
        y *= m;
        z *= m;

        Self {
            w: directionality * 2f64.sqrt(),
            coordinate: Cartesian { x, y, z },
        }
    }

    #[inline]
    pub fn dot(&self, sample: &SpatialSample) -> f64 {
        self.w * sample.value + self.coordinate.dot(&sample.coordinate)
    }
}
