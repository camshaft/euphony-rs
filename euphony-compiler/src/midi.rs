use crate::{sample::Offset, Hash};
use core::ops::Deref;
use euphony_units::time::Beat;
use std::io;

mod smf;

#[derive(Debug, Default)]
pub struct Writer {
    buffer: Vec<u8>,
    hash: Hash,
    last_sample: Option<(usize, Offset)>,
}

impl Writer {
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub(crate) fn write(&mut self, sample: Offset, beat: Beat, data: [u8; 3]) {
        if let Some((len_offset, offset)) = self.last_sample {
            if offset == sample {
                let count = &mut self.buffer[len_offset];
                *count += 1;
                if *count == u8::MAX {
                    self.last_sample = None;
                }

                self.buffer.extend_from_slice(&data);
                return;
            }
        }

        self.buffer.extend_from_slice(&sample.to_bytes());
        self.buffer.extend_from_slice(&beat.0.to_le_bytes());
        self.buffer.extend_from_slice(&beat.1.to_le_bytes());
        self.last_sample = Some((self.buffer.len(), sample));
        self.buffer.extend_from_slice(&0u8.to_le_bytes());
        self.buffer.extend_from_slice(&data);
    }

    pub(crate) fn finish(&mut self) {
        if self.is_empty() {
            return;
        }

        self.hash = *blake3::hash(&self.buffer).as_bytes();
    }
}

impl Deref for Writer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer[..]
    }
}

pub struct Reader<R: io::Read> {
    r: R,
    offset: Offset,
    beat: Beat,
    remaining: u16,
}

impl<R: io::Read> Reader<R> {
    pub fn new(r: R) -> Self {
        Self {
            r,
            offset: Default::default(),
            beat: Default::default(),
            remaining: 0,
        }
    }

    pub fn write_smf<W: io::Write + io::Seek>(&mut self, out: W) -> io::Result<()> {
        smf::write(self, out)
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        let mut data = [0u8; 8];
        self.r.read_exact(&mut data)?;
        Ok(u64::from_le_bytes(data))
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        let mut data = [0u8; 1];
        self.r.read_exact(&mut data)?;
        Ok(data[0])
    }

    fn read_data(&mut self) -> io::Result<[u8; 3]> {
        let mut data = [0u8; 3];
        self.r.read_exact(&mut data)?;
        Ok(data)
    }

    fn try_next(&mut self) -> io::Result<Option<(Offset, Beat, [u8; 3])>> {
        loop {
            if let Some(remaining) = self.remaining.checked_sub(1) {
                self.remaining = remaining;
                let data = self.read_data()?;
                return Ok(Some((self.offset, self.beat, data)));
            }

            let offset = match self.read_u64() {
                Ok(value) => value,
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
                Err(err) => return Err(err),
            };
            let beat_0 = self.read_u64()?;
            let beat_1 = self.read_u64()?;
            let beats = Beat(beat_0, beat_1);
            let remaining = self.read_u8()?;

            self.offset = Offset::new(offset);
            self.beat = beats;
            self.remaining = remaining as u16 + 1;
        }
    }
}

impl<R: io::Read> Iterator for Reader<R> {
    type Item = io::Result<(Offset, Beat, [u8; 3])>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}
