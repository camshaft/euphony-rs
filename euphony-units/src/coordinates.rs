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

    #[inline]
    pub fn stereo_weights(&self) -> (f64, f64) {
        let x = self.x;
        let angle = (x.clamp(-1.0, 1.0) + 1.0) * (core::f64::consts::PI * 0.25);
        (angle.cos(), angle.sin())
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

        let azimuth = azimuth * core::f64::consts::PI / 2.0;
        let inclination = inclination * core::f64::consts::PI / 2.0;

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

    fn round(v: f64) -> f64 {
        (v * 100.0).round() / 100.0
    }

    fn round_coord(coord: Cartesian<f64>) -> Cartesian<f64> {
        Cartesian {
            x: round(coord.x),
            y: round(coord.y),
            z: round(coord.z),
        }
    }

    fn nums() -> impl Iterator<Item = f64> {
        let nums = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        nums.into_iter().rev().map(|v| -1.0 * v).chain(nums)
    }

    #[test]
    fn conv() {
        let mut results = vec![];

        for azimuth in nums() {
            let polar = Polar {
                azimuth,
                inclination: 0.0,
                radius: 1.0,
            };
            let coord = round_coord(polar.into());
            results.push((polar, coord));
        }
        for inclination in nums() {
            let polar = Polar {
                azimuth: 0.0,
                inclination,
                radius: 1.0,
            };
            let coord = round_coord(polar.into());
            results.push((polar, coord));
        }

        insta::assert_debug_snapshot!(results);
    }

    #[test]
    fn stereo() {
        fn gen(x: f64) -> (f64, f64) {
            let coord = Cartesian { x, y: 0.0, z: 0.0 };
            let (l, r) = coord.stereo_weights();
            (round(l), round(r))
        }

        let results = nums().map(|x| (x, gen(x))).collect::<Vec<_>>();

        insta::assert_debug_snapshot!(results);
    }
}
