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
    coord_buffer: Option<(Cartesian<f64>, u32)>,
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
            coord_buffer: None,
            sink,
        }
    }

    #[inline]
    fn write_coord(&mut self, coord: Cartesian<f64>) {
        if let Some((prev, count)) = self.coord_buffer.as_mut() {
            if *prev == coord {
                if let Some(next_count) = count.checked_add(1) {
                    *count = next_count;
                    return;
                }
            }

            let prev = *prev;
            let count = *count;
            self.flush_coord(prev, count);
        } else {
            self.coord_buffer = Some((coord, 0));
        }
    }

    #[inline]
    fn flush_coord(&mut self, coord: Cartesian<f64>, count: u32) {
        let mut bytes = [0u8; 4 * 4];
        bytes[..4].copy_from_slice(&count.to_le_bytes());
        bytes[4..8].copy_from_slice(&(coord.x as f32).to_ne_bytes());
        bytes[8..12].copy_from_slice(&(coord.y as f32).to_ne_bytes());
        bytes[12..].copy_from_slice(&(coord.z as f32).to_ne_bytes());

        self.coordinates.write(&bytes);
    }
}

impl<W: storage::Output> Sink for Writer<W> {
    #[inline]
    fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, samples: S) {
        for (sample, coord) in samples {
            let sample = self.leak_dc.apply(sample);
            let sample = sample.to_ne_bytes();
            self.samples.write(&sample);

            self.write_coord(coord.into());
        }
    }
}

impl<W: storage::Output> Drop for Writer<W> {
    #[inline]
    fn drop(&mut self) {
        let a = self.samples.finish();
        self.sink.write(&a);

        if let Some((coord, count)) = self.coord_buffer.take() {
            self.flush_coord(coord, count);
        }

        let b = self.coordinates.finish();
        self.sink.write(&b);
    }
}
