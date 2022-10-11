use crate::{ext::*, storage::Storage};
use euphony_compiler::Hash;
use euphony_mix::SpatialSample;
use euphony_units::coordinates::Cartesian;
use std::io;

pub struct Reader<R: io::Read> {
    sample: R,
    coordinate: R,
    coord_buffer: Option<(Cartesian<f64>, u32)>,
}

impl<R: io::Read> Reader<R> {
    #[inline]
    pub fn new<S: Storage<Reader = R>>(storage: &S, hash: &Hash) -> io::Result<Self> {
        let mut group = storage.open_raw(hash)?;

        macro_rules! open {
            () => {{
                let mut hash = Hash::default();
                group.read_exact(&mut hash)?;
                storage.open_raw(&hash)?
            }};
        }

        let sample = open!();
        let coordinate = open!();

        Ok(Self {
            sample,
            coordinate,
            coord_buffer: None,
        })
    }

    #[inline]
    fn read_sample(&mut self) -> io::Result<SpatialSample> {
        let value = self.sample.read_f64()?;
        let coordinate = self.read_coordinate()?;

        Ok(SpatialSample { value, coordinate })
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

        let count = self.coordinate.read_u32()?;
        let coord = self.coordinate.read_coordinate()?;

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
        match self.read_sample() {
            Ok(sample) => Some(Ok(sample)),
            Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(err) => Some(Err(err)),
        }
    }
}
