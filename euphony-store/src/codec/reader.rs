use crate::{
    record::{self, SinkIndex, Stream, StreamKind},
    storage::Storage,
};
use euphony_compiler::Hash;
use euphony_mix::SpatialSample;
use euphony_units::coordinates::Cartesian;
use std::io;

/// Reads a sink's samples, each with its coordinate, from the sink's index record and its two
/// stream records.
pub struct Reader {
    samples: Vec<u8>,
    sample: usize,
    runs: Vec<u8>,
    run: usize,
    coord_buffer: Option<(Cartesian<f64>, u32)>,
}

impl Reader {
    #[inline]
    pub fn new<S: Storage>(storage: &S, hash: &Hash) -> io::Result<Self> {
        let index: SinkIndex = storage.read_record(hash)?;
        let samples = storage
            .read_record::<Stream>(&index.samples)?
            .into_body(StreamKind::Samples)?;
        let runs = storage
            .read_record::<Stream>(&index.coordinates)?
            .into_body(StreamKind::Coordinates)?;

        Ok(Self {
            samples,
            sample: 0,
            runs,
            run: 0,
            coord_buffer: None,
        })
    }

    #[inline]
    fn read_sample(&mut self) -> io::Result<Option<SpatialSample>> {
        let Some(bytes) = self.samples.get(self.sample..self.sample + record::SAMPLE) else {
            return Ok(None);
        };
        let value = f64::from_le_bytes(bytes.try_into().unwrap());
        self.sample += record::SAMPLE;

        let coordinate = self.read_coordinate()?;

        Ok(Some(SpatialSample { value, coordinate }))
    }

    #[inline]
    fn read_coordinate(&mut self) -> io::Result<Cartesian<f64>> {
        if let Some((coord, count)) = self.coord_buffer.as_mut() {
            let res = *coord;

            if let Some(next) = count.checked_sub(1) {
                *count = next;
            } else {
                self.coord_buffer = None;
            }

            return Ok(res);
        }

        let Some(run) = self.runs.get(self.run..self.run + record::COORDINATE_RUN) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "the coordinate stream ends before the sample stream",
            ));
        };
        let (count, coord) = record::read_coordinate_run(run);
        self.run += record::COORDINATE_RUN;

        if let Some(count) = count.checked_sub(1) {
            self.coord_buffer = Some((coord, count));
        }

        Ok(coord)
    }
}

impl Iterator for Reader {
    type Item = io::Result<SpatialSample>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.read_sample().transpose()
    }
}
