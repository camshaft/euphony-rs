use crate::{
    dc::LeakDc,
    record::{self, Digest, SinkIndex, StreamKind},
    storage::{Output as _, Storage},
};
use core::fmt;
use euphony_compiler::Hash;
use euphony_node::Sink;
use euphony_units::coordinates::{Cartesian, Polar};

/// Writes a sink's samples and coordinates as two streams and, as it drops, the sink's index record
/// under the hash the compiler gave the sink. Where either stream fails, the renderer discards the
/// writer, or the writer drops while a panic unwinds, the index isn't written, so the sink isn't
/// cached and is rendered again.
pub struct Writer<S: Storage> {
    leak_dc: LeakDc,
    samples: S::Output,
    coordinates: S::Output,
    coord_buffer: Option<(Cartesian<f64>, u32)>,
    storage: S,
    hash: Hash,
    discarded: bool,
}

impl<S: Storage> fmt::Debug for Writer<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawWriter").finish()
    }
}

impl<S: Storage> Writer<S> {
    pub fn new(storage: S, hash: Hash) -> Self {
        Self {
            leak_dc: Default::default(),
            samples: storage.create(StreamKind::Samples),
            coordinates: storage.create(StreamKind::Coordinates),
            coord_buffer: None,
            storage,
            hash,
            discarded: false,
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
        }

        self.coord_buffer = Some((coord, 0));
    }

    #[inline]
    fn flush_coord(&mut self, coord: Cartesian<f64>, count: u32) {
        self.coordinates
            .write(&record::coordinate_run(count, coord));
    }
}

impl<S: Storage + Send + 'static> Sink for Writer<S> {
    #[inline]
    fn write<I: Iterator<Item = (f64, Polar<f64>)>>(&mut self, samples: I) {
        for (sample, coord) in samples {
            let sample = self.leak_dc.apply(sample);
            self.samples.write(&sample.to_le_bytes());

            self.write_coord(coord.into());
        }
    }

    #[inline]
    fn discard(&mut self) {
        self.discarded = true;
    }
}

impl<S: Storage> Drop for Writer<S> {
    #[inline]
    fn drop(&mut self) {
        // a panic that unwinds through the writer, such as one in a node while the sink renders,
        // leaves the streams holding only the samples rendered so far: neither stream is finished,
        // so each drops without being stored, and the sink is rendered again
        if std::thread::panicking() {
            log::error!("a sink's rendering panicked, so its index isn't written");
            return;
        }

        // a render that failed or was interrupted leaves the sink unfinished: as above, neither
        // stream is finished, and the sink is rendered again
        if self.discarded {
            log::warn!("a sink's rendering didn't finish, so its index isn't written");
            return;
        }

        let samples = self.samples.finish();

        if let Some((coord, count)) = self.coord_buffer.take() {
            self.flush_coord(coord, count);
        }

        let coordinates = self.coordinates.finish();

        let (samples, coordinates) = match (samples, coordinates) {
            (Ok(samples), Ok(coordinates)) => (samples, coordinates),
            (Err(err), _) | (_, Err(err)) => {
                log::error!("could not write a sink's streams, so its index isn't written: {}", err);
                return;
            }
        };

        let index = SinkIndex {
            samples: Digest(samples),
            coordinates: Digest(coordinates),
        };

        let result = record::encode(&index)
            .and_then(|bytes| self.storage.write_record(&self.hash, &bytes));

        if let Err(err) = result {
            log::error!("could not write a sink's index: {}", err);
        }
    }
}
