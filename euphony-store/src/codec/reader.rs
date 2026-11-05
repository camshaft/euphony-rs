use crate::{
    record::{self, ChunkReader, SinkIndex, StreamKind},
    storage::Storage,
};
use euphony_compiler::Hash;
use euphony_mix::SpatialSample;
use euphony_units::coordinates::Cartesian;
use std::io;

/// Reads a sink's samples, each with its coordinate, from the sink's index record and its two
/// streams. It reads each stream one chunk record at a time, so it holds at most one chunk of each.
pub struct Reader<R> {
    samples: Values<R>,
    runs: Values<R>,
    coord_buffer: Option<(Cartesian<f64>, u32)>,
}

impl<R: io::Read> Reader<R> {
    #[inline]
    pub fn new<S: Storage<Reader = R>>(storage: &S, hash: &Hash) -> io::Result<Self> {
        let index: SinkIndex = storage.read_record(hash)?;
        let samples = Values::new(storage.open_raw(&index.samples)?, StreamKind::Samples);
        let runs = Values::new(
            storage.open_raw(&index.coordinates)?,
            StreamKind::Coordinates,
        );

        Ok(Self {
            samples,
            runs,
            coord_buffer: None,
        })
    }

    #[inline]
    fn read_sample(&mut self) -> io::Result<Option<SpatialSample>> {
        let Some(bytes) = self.samples.next()? else {
            return Ok(None);
        };
        let value = f64::from_le_bytes(bytes.try_into().unwrap());

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

        let Some(run) = self.runs.next()? else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "the coordinate stream ends before the sample stream",
            ));
        };
        let (count, coord) = record::read_coordinate_run(run);

        if let Some(count) = count.checked_sub(1) {
            self.coord_buffer = Some((coord, count));
        }

        Ok(coord)
    }
}

impl<R: io::Read> Iterator for Reader<R> {
    type Item = io::Result<SpatialSample>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.read_sample().transpose()
    }
}

/// The values of one stream, read one chunk record at a time.
struct Values<R> {
    chunks: ChunkReader<R>,
    chunk: Vec<u8>,
    at: usize,
}

impl<R: io::Read> Values<R> {
    #[inline]
    fn new(reader: R, kind: StreamKind) -> Self {
        Self {
            chunks: ChunkReader::new(reader, kind),
            chunk: vec![],
            at: 0,
        }
    }

    /// The bytes of the next value, or `None` at the stream's end. It reads the next chunk record
    /// in place of the current one once the current one is read whole.
    #[inline]
    fn next(&mut self) -> io::Result<Option<&[u8]>> {
        while self.at == self.chunk.len() {
            let Some(chunk) = self.chunks.next_chunk()? else {
                return Ok(None);
            };
            self.chunk = chunk;
            self.at = 0;
        }

        // a chunk's body holds whole values, which `ChunkReader` checks
        let len = self.chunks.kind().value_len();
        let value = &self.chunk[self.at..self.at + len];
        self.at += len;
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{codec::Writer, storage::fs::Directory};
    use euphony_node::Sink;
    use euphony_units::coordinates::Polar;

    #[test]
    fn a_reader_holds_at_most_one_chunk_of_each_stream() {
        let dir = tempfile::tempdir().unwrap();
        let directory = Directory::new(dir.path().to_owned());
        std::fs::create_dir_all(directory.path()).unwrap();
        let hash = [5; 32];

        // more samples than three chunks hold, each at a coordinate of its own, so that each
        // stream spans several chunks
        let len = 3 * record::CHUNK / record::SAMPLE + 1;
        {
            let mut writer = Writer::new(directory.clone(), hash);
            Sink::write(
                &mut writer,
                (0..len).map(|i| {
                    (
                        0.0,
                        Polar {
                            azimuth: i as f64,
                            inclination: 0.0,
                            radius: 1.0,
                        },
                    )
                }),
            );
        }

        let mut reader = directory.open_sink(&hash).unwrap();
        let mut read = 0;
        while let Some(sample) = reader.next() {
            sample.unwrap();
            read += 1;
            assert!(reader.samples.chunk.len() <= record::CHUNK);
            assert!(reader.runs.chunk.len() <= record::CHUNK);
        }
        assert_eq!(read, len);
    }
}
