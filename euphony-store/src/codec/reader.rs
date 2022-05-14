use crate::{ext::*, storage::Storage};
use euphony_compiler::Hash;
use euphony_mix::SpatialSample;
use std::io;

pub struct Reader<R: io::Read> {
    sample: R,
    coordinate: R,
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

        Ok(Self { sample, coordinate })
    }

    #[inline]
    fn read_sample(&mut self) -> io::Result<SpatialSample> {
        let value = self.sample.read_f64()?;
        let coordinate = self.coordinate.read_coordinate()?;

        Ok(SpatialSample { value, coordinate })
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
