use crate::{storage::Storage, Hash};
use core::mem::size_of;
use euphony_mix::{Mixer, SpatialSample};
use std::{collections::VecDeque, io};

const SAMPLE_CAPACITY: usize = 4096 / size_of::<SpatialSample>();

pub fn mix<S: Storage, M: Mixer<Error = E>, E: Into<io::Error>>(
    storage: &S,
    group: &Hash,
    mixer: &mut M,
) -> io::Result<()> {
    let mut sample_offset = 0;
    let group = storage.open_group(group)?;

    let mut exporter: Exporter<S, M, E> = Exporter {
        samples: Vec::with_capacity(SAMPLE_CAPACITY),
        entries: VecDeque::with_capacity(16),
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
    mixer: &'a mut M,
}

impl<'a, S: Storage, W: Mixer<Error = E>, E: Into<io::Error>> Exporter<'a, S, W, E> {
    #[inline]
    pub fn insert(&mut self, entry: S::Sink) {
        self.entries.push_back(entry);
    }

    #[inline]
    pub fn flush(&mut self, samples: u64) -> io::Result<()> {
        self.write(Some(samples))?;
        Ok(())
    }

    #[inline]
    pub fn finalize(&mut self) -> io::Result<()> {
        self.write(None)?;
        Ok(())
    }

    #[inline]
    fn write(&mut self, samples: Option<u64>) -> io::Result<()> {
        let mut remaining = samples.unwrap_or(u64::MAX);
        let mut entries_len = self.entries.len();
        let mut finished = vec![];

        while remaining > 0 && entries_len > 0 {
            remaining -= 1;

            for (idx, entry) in self.entries.iter_mut().enumerate() {
                if let Some(sample) = entry.next() {
                    self.samples.push(sample?);
                } else {
                    finished.push(idx);
                }
            }

            // mix the buffer when we change the number of active entries or hit the capacity
            if finished.is_empty() && self.samples.len() < SAMPLE_CAPACITY {
                continue;
            }

            self.mix_buffer(entries_len)?;

            for idx in finished.drain(..).rev() {
                self.entries.remove(idx);
            }

            entries_len = self.entries.len();

            // if the last iteration cleared all of the entires, we wouldn't have pushed a sample
            // so we need to undo the decrement
            if entries_len == 0 {
                remaining += 1;
            }
        }

        if !self.samples.is_empty() {
            self.mix_buffer(entries_len)?;
        }

        if samples.is_some() && remaining > 0 {
            self.mixer.skip(remaining as _).map_err(|e| e.into())?;
        }

        Ok(())
    }

    #[inline]
    fn mix_buffer(&mut self, entries_len: usize) -> io::Result<()> {
        for chunk in self.samples.chunks(entries_len) {
            self.mixer.mix(chunk).map_err(|e| e.into())?;
        }

        self.samples.clear();

        Ok(())
    }
}
