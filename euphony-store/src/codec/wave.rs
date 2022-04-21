use crate::codec::Codec;
use core::fmt;
use euphony_compiler::{DefaultSampleRate, Sample, SampleRate};
use euphony_node::{SampleType, Sink};
use hound::{WavSpec, WavWriter};
use std::io;

pub struct Writer<W: io::Write + io::Seek + Send> {
    writer: WavWriter<W>,
}

impl<W: io::Write + io::Seek + Send> fmt::Debug for Writer<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WaveWriter").finish()
    }
}

impl<W: 'static + io::Write + io::Seek + Send> Codec<W> for Writer<W> {
    const EXTENSION: &'static str = "wav";

    fn new(writer: W) -> io::Result<Self> {
        Ok(Self {
            writer: WavWriter::new(
                writer,
                WavSpec {
                    channels: 1,
                    sample_rate: DefaultSampleRate::COUNT as _,
                    bits_per_sample: 32,
                    sample_format: hound::SampleFormat::Float,
                },
            )
            .map_err(|err| match err {
                hound::Error::IoError(err) => err,
                err => io::Error::new(io::ErrorKind::Other, err),
            })?,
        })
    }
}

impl<W: 'static + io::Write + io::Seek + Send> Sink for Writer<W> {
    #[inline]
    fn advance(&mut self, _: u64) {
        // no-op
    }

    #[inline]
    fn write(&mut self, ty: SampleType, samples: &[Sample]) {
        // we only support PCM currently
        if ty != SampleType::Pcm {
            return;
        }

        for sample in samples.iter().copied() {
            let _ = self.writer.write_sample(sample as f32);
        }
    }
}

impl<W: io::Write + io::Seek + Send> Drop for Writer<W> {
    fn drop(&mut self) {
        if let Err(err) = self.writer.flush() {
            eprintln!("{:?}", err);
        }
    }
}
