use crate::{
    dc::LeakDc,
    storage::{self, Storage},
};
use core::fmt;
use euphony_node::Sink;
use euphony_units::coordinates::{Cartesian, Polar};

pub struct Writer<O: storage::Output> {
    leak_dc: LeakDc,
    samples: O,
    coordinates: O,
    sink: O,
}

impl<W: storage::Output> fmt::Debug for Writer<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawWriter").finish()
    }
}

impl<O: storage::Output> Writer<O> {
    pub fn new<S: Storage<Output = O>>(storage: &mut S, sink: O) -> Self {
        Self {
            leak_dc: Default::default(),
            samples: storage.create(),
            coordinates: storage.create(),
            sink,
        }
    }
}

impl<W: storage::Output> Sink for Writer<W> {
    #[inline]
    fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, samples: S) {
        for (sample, coord) in samples {
            let sample = self.leak_dc.apply(sample);
            let sample = sample.to_ne_bytes();
            self.samples.write(&sample);

            let cart: Cartesian<f64> = coord.into();
            let mut bytes = [0u8; 4 * 3];
            bytes[..4].copy_from_slice(&(cart.x as f32).to_ne_bytes());
            bytes[4..8].copy_from_slice(&(cart.y as f32).to_ne_bytes());
            bytes[8..].copy_from_slice(&(cart.z as f32).to_ne_bytes());
            self.coordinates.write(&bytes);
        }
    }
}

impl<W: storage::Output> Drop for Writer<W> {
    fn drop(&mut self) {
        let a = self.samples.finish();
        self.sink.write(&a);
        let b = self.coordinates.finish();
        self.sink.write(&b);
    }
}
