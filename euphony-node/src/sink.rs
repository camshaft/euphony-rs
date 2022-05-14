use euphony_units::coordinates::Polar;

use crate::{BoxProcessor, Buffers, Inputs, Node, Output};

pub trait Sink: 'static + Send + Sized {
    #[inline]
    fn spawn(self) -> BoxProcessor {
        Wrapper::spawn(self)
    }

    fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, samples: S);
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SampleType {
    Pcm = 0,
    /// The azimuth (or azimuthal angle) is the signed angle measured from
    /// the azimuth reference direction to the orthogonal projection of the
    /// line segment OP on the reference plane.
    Azimuth = 1,
    /// The inclination (or polar angle) is the angle between the zenith
    /// direction and the line segment OP.
    Inclination = 2,
    /// The radius or radial distance is the Euclidean distance from the
    /// origin O to P.
    Radius = 3,
}

#[derive(Clone, Debug, Default)]
struct Wrapper<Inner: Sink> {
    inner: Inner,
}

impl<Inner: Sink> Wrapper<Inner> {
    fn spawn(inner: Inner) -> BoxProcessor {
        crate::spawn(Self { inner })
    }
}

impl<Inner: Sink> Node<4, 0> for Wrapper<Inner> {
    const DEFAULTS: [f64; 4] = [0.0, 0.0, 0.0, 0.0];

    #[inline]
    fn process(&mut self, inputs: Inputs<4>, _buffer: Buffers<0>, samples: &mut [f64]) {
        let pcm = inputs.get(0);
        let pcm = pcm.iter().take(samples.len());
        let azimuth = inputs.get(1);
        let azimuth = azimuth.iter().take(samples.len());
        let inclination = inputs.get(2);
        let inclination = inclination.iter().take(samples.len());
        let radius = inputs.get(3);
        let radius = radius.iter().take(samples.len());

        let coord = azimuth
            .zip(inclination)
            .zip(radius)
            .map(|((azimuth, inclination), radius)| Polar {
                azimuth,
                inclination,
                radius,
            });

        let samples = pcm.zip(coord);

        self.inner.write(samples)
    }

    #[inline]
    fn process_full(&mut self, inputs: Inputs<4>, _buffer: Buffers<0>, _samples: &mut Output) {
        let pcm = inputs.get(0);
        let pcm = pcm.iter();
        let azimuth = inputs.get(1);
        let azimuth = azimuth.iter();
        let inclination = inputs.get(2);
        let inclination = inclination.iter();
        let radius = inputs.get(3);
        let radius = radius.iter();

        let coord = azimuth
            .zip(inclination)
            .zip(radius)
            .map(|((azimuth, inclination), radius)| Polar {
                azimuth,
                inclination,
                radius,
            });

        let samples = pcm.zip(coord);

        self.inner.write(samples)
    }
}
