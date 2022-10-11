#[derive(Clone, Copy, Debug, Default)]
pub struct Polar<T> {
    pub azimuth: T,
    pub inclination: T,
    pub radius: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cartesian<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Cartesian<f64> {
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[cfg(any(test, feature = "std"))]
impl From<Polar<f64>> for Cartesian<f64> {
    #[inline]
    fn from(coor: Polar<f64>) -> Self {
        let Polar {
            azimuth,
            inclination,
            radius,
        } = coor;

        // https://en.wikipedia.org/wiki/Spherical_coordinate_system#Cartesian_coordinates
        // (radius r, inclination θ, azimuth φ)
        // x = r cos φ sin θ
        // y = r sin φ sin θ
        // z = r cos θ

        // here, we invert all of the trig functions to create 0 as center, forward

        let azimuth = azimuth * core::f64::consts::PI;
        let inclination = inclination * core::f64::consts::PI;

        let sin_inc = inclination.cos();

        let x = radius * azimuth.sin() * sin_inc;
        let y = radius * azimuth.cos() * sin_inc;
        let z = radius * inclination.sin();

        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv() {
        for azimuth in [0.0f64, 0.25, 0.5, 0.6, 0.75, 0.9, 1.0] {
            for m in [-1.0, 1.0] {
                let azimuth = azimuth * m;
                let coord: Cartesian<f64> = Polar {
                    azimuth,
                    inclination: 0.0,
                    radius: 1.0,
                }
                .into();
                eprintln!("{azimuth} {:?}", coord);
            }
        }
    }
}
