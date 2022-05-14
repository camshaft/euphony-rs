use crate::{storage::Storage, Hash};
use euphony_mix::{Mixer, SpatialSample};
use std::{collections::VecDeque, io};

pub fn mix<S: Storage, M: Mixer<Error = E>, E: Into<io::Error>>(
    storage: &S,
    group: &Hash,
    mixer: &mut M,
) -> io::Result<()> {
    let mut sample_offset = 0;
    let group = storage.open_group(group)?;

    let mut exporter: Exporter<S, M, E> = Exporter {
        samples: Vec::with_capacity(16),
        entries: Default::default(),
        tmp: Default::default(),
        mixer,
    };

    for entry in group {
        let entry = entry?;

        if entry.sample_offset > sample_offset {
            let amount = entry.sample_offset - sample_offset;
            exporter.flush(amount)?;
            sample_offset = entry.sample_offset;
        }

        let sink = storage.open_sink(&entry.hash)?;
        exporter.insert(sink);
    }

    exporter.finalize()?;

    Ok(())
}

struct Exporter<'a, S: Storage, M: Mixer<Error = E>, E> {
    samples: Vec<SpatialSample>,
    entries: VecDeque<S::Sink>,
    tmp: VecDeque<S::Sink>,
    mixer: &'a mut M,
}

impl<'a, S: Storage, W: Mixer<Error = E>, E: Into<io::Error>> Exporter<'a, S, W, E> {
    #[inline]
    pub fn insert(&mut self, entry: S::Sink) {
        self.entries.push_back(entry);
    }

    #[inline]
    pub fn flush(&mut self, samples: u64) -> io::Result<()> {
        for _ in 0..samples {
            self.write()?;
        }

        Ok(())
    }

    #[inline]
    pub fn finalize(&mut self) -> io::Result<()> {
        while !self.entries.is_empty() {
            self.write()?;
        }

        Ok(())
    }

    #[inline]
    fn write(&mut self) -> io::Result<()> {
        self.samples.clear();

        for mut entry in self.entries.drain(..) {
            if let Some(sample) = entry.next() {
                self.samples.push(sample?);
                self.tmp.push_back(entry);
            }
        }

        self.mixer.mix(&self.samples).map_err(|e| e.into())?;

        core::mem::swap(&mut self.entries, &mut self.tmp);

        Ok(())
    }
}
