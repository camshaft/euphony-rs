use crate::{BoxProcessor, Buffers, Input, Inputs, Node, Output, LEN};

pub trait Sink: 'static + Send + Sized {
    #[inline]
    fn spawn(self) -> BoxProcessor {
        Wrapper::spawn(self)
    }

    fn advance(&mut self, samples: u64);

    fn write(&mut self, ty: SampleType, samples: &[f64]);

    #[inline]
    fn write_full(&mut self, ty: SampleType, samples: &Output) {
        self.write(ty, samples);
    }

    #[inline]
    fn write_const(&mut self, ty: SampleType, value: f64, count: usize) {
        let mut values = [0.0; LEN];
        for to in values[..count].iter_mut() {
            *to = value;
        }
        if count == LEN {
            self.write_full(ty, &values)
        } else {
            self.write(ty, &values[..count])
        }
    }
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
    #[inline]
    fn process(&mut self, inputs: Inputs<4>, _buffer: Buffers<0>, samples: &mut [f64]) {
        macro_rules! input {
            ($ty:expr) => {{
                match inputs.get($ty as u8 as usize) {
                    Input::Constant(v) => self.inner.write_const($ty, v, samples.len()),
                    Input::Buffer(b) => self.inner.write($ty, &b[..samples.len()]),
                };
            }};
        }

        input!(SampleType::Pcm);
        input!(SampleType::Azimuth);
        input!(SampleType::Inclination);
        input!(SampleType::Radius);

        self.inner.advance(samples.len() as u64);
    }

    #[inline]
    fn process_full(&mut self, inputs: Inputs<4>, _buffer: Buffers<0>, samples: &mut Output) {
        macro_rules! input {
            ($ty:expr) => {{
                match inputs.get($ty as u8 as usize) {
                    Input::Constant(v) => self.inner.write_const($ty, v, LEN),
                    Input::Buffer(b) => self.inner.write_full($ty, b),
                };
            }};
        }

        input!(SampleType::Pcm);
        input!(SampleType::Azimuth);
        input!(SampleType::Inclination);
        input!(SampleType::Radius);

        self.inner.advance(samples.len() as u64);
    }
}
