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
        samples: Vec::with_capacity(4096),
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
    fn write(&mut self, limit: Option<u64>) -> io::Result<()> {
        let mut remaining = limit.unwrap_or(u64::MAX);

        while remaining > 0 && !self.entries.is_empty() {
            let mut finished = vec![];

            for (idx, entry) in self.entries.iter_mut().enumerate() {
                if let Some(sample) = entry.next() {
                    self.samples.push(sample?);
                } else {
                    finished.push(idx);
                }
            }

            remaining -= 1;
            if finished.is_empty() {
                continue;
            }

            self.mix_buffer()?;

            for idx in finished.into_iter().rev() {
                self.entries.remove(idx);
            }
        }

        self.mix_buffer()?;

        if limit.is_some() && remaining > 0 {
            self.mixer.skip(remaining as _).map_err(|e| e.into())?;
        }

        Ok(())
    }

    #[inline(never)]
    fn mix_buffer(&mut self) -> io::Result<()> {
        let sample_len = self.samples.len();
        let entries_len = self.entries.len();

        if sample_len == 0 {
            return Ok(());
        }

        let mixed_samples = if entries_len > 0 {
            let mixed_samples = sample_len / entries_len * entries_len;

            if mixed_samples > 0 {
                for chunk in self.samples.chunks(mixed_samples) {
                    self.mixer.mix(chunk).map_err(|e| e.into())?;
                }
            }

            mixed_samples
        } else {
            0
        };

        self.mixer
            .mix(&self.samples[mixed_samples..])
            .map_err(|e| e.into())?;

        self.samples.clear();

        Ok(())
    }
}
