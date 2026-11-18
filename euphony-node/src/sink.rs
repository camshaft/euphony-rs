use euphony_units::coordinates::Polar;

use crate::{BoxProcessor, Buffers, Inputs, Node};

pub trait Sink: 'static + Send + Sized {
    #[inline]
    fn spawn(self) -> BoxProcessor {
        Wrapper::spawn(self)
    }

    fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, samples: S);

    /// Called just before the sink drops where its render didn't finish, so the samples it holds
    /// are only part of the sink and aren't to be kept. The default does nothing.
    #[inline]
    fn discard(&mut self) {}
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
    fn discard(&mut self) {
        self.inner.discard()
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Config, Context, LEN};
    use euphony_graph::Graph;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    /// Counts the samples the sink takes, at most one more than a block holds
    struct Count(Arc<AtomicUsize>);

    impl Sink for Count {
        fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, samples: S) {
            // an unbounded block never ends, so take one sample past it at most
            let count = samples.take(LEN + 1).count();
            self.0.fetch_add(count, Ordering::SeqCst);
        }
    }

    #[test]
    fn a_full_block_of_constant_inputs_writes_one_block_of_samples() {
        //= rendered/0006-distributed-rendering/08-render-blocks.md#EU-0006-098
        //= type=test
        //# A render block MUST hold at most 256 samples of each node output.
        //= rendered/0006-distributed-rendering/08-render-blocks.md#EU-0006-100
        //= type=test
        //# In each render block, each node, each sink among them, MUST take at most as many samples from each of its inputs as the block holds, whatever the input holds.
        let count = Arc::new(AtomicUsize::new(0));
        let mut graph = Graph::<Config>::default();

        // each of the sink's inputs holds its constant default, so no input ends the block
        graph.insert(1, Count(count.clone()).spawn());
        graph.update().unwrap();
        graph.process(&Context::default());

        // a full block holds 256 samples, and the sink takes each of them
        assert_eq!(count.load(Ordering::SeqCst), 256);
    }

    #[test]
    fn a_shorter_block_of_constant_inputs_writes_the_samples_it_holds() {
        //= rendered/0006-distributed-rendering/08-render-blocks.md#EU-0006-100
        //= type=test
        //# In each render block, each node, each sink among them, MUST take at most as many samples from each of its inputs as the block holds, whatever the input holds.
        let count = Arc::new(AtomicUsize::new(0));
        let mut graph = Graph::<Config>::default();

        // a block that ends at the render's next instruction or at its end holds fewer samples
        graph.insert(1, Count(count.clone()).spawn());
        graph.update().unwrap();
        graph.process(&Context {
            partial: Some(100),
            ..Context::default()
        });

        assert_eq!(count.load(Ordering::SeqCst), 100);
    }
}
