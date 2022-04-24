use crate::codec::Codec;
use base64::URL_SAFE_NO_PAD;
use euphony_compiler::{Entry, Hash, Sample, Writer};
use euphony_node::{BoxProcessor, SampleType, Sink};
use std::{
    fs,
    io::{self, Write},
    marker::PhantomData,
    path::PathBuf,
};

pub type File = io::BufWriter<fs::File>;

#[derive(Clone, Debug)]
pub struct Directory<C: Codec<File>> {
    pub path: PathBuf,
    codec: PhantomData<C>,
}

impl<C: Codec<File>> Default for Directory<C> {
    fn default() -> Self {
        Self {
            path: PathBuf::from("target/euphony/contents"),
            codec: PhantomData,
        }
    }
}

impl<C: Codec<File>> Directory<C> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    fn path(&self, hash: &Hash) -> PathBuf {
        let mut out = [b'A'; 64];
        let len = base64::encode_config_slice(hash, URL_SAFE_NO_PAD, &mut out);
        let out = unsafe { core::str::from_utf8_unchecked_mut(&mut out) };
        let out = &out[..len];
        self.path.join(out)
    }

    fn create(&self, hash: &Hash) -> io::Result<Option<File>> {
        let path = self.path(hash);
        fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .map(io::BufWriter::new)
            .map(Some)
            .or_else(|err| {
                if err.kind() == io::ErrorKind::AlreadyExists {
                    Ok(None)
                } else {
                    Err(err)
                }
            })
    }

    fn write_group<I: Iterator<Item = Entry>>(file: Option<File>, entries: I) -> io::Result<()> {
        if let Some(mut file) = file {
            for entry in entries {
                file.write_all(&entry.sample_offset.to_be_bytes())?;
                file.write_all(&entry.hash)?;
            }
        }
        Ok(())
    }
}

impl<C: Codec<File> + Sync> Writer for Directory<C> {
    fn is_cached(&self, hash: &Hash) -> bool {
        self.path(hash).exists()
    }

    fn sink(&mut self, hash: &Hash) -> BoxProcessor {
        let file = match self.create(hash) {
            Ok(Some(file)) => file,
            Ok(None) => return NoopSink.spawn(),
            Err(err) => {
                eprintln!("error while creating sink: {}", err);
                return NoopSink.spawn();
            }
        };

        match C::new(file) {
            Ok(c) => c.spawn(),
            Err(err) => {
                eprintln!("error while creating sink: {}", err);
                NoopSink.spawn()
            }
        }
    }

    fn group<I: Iterator<Item = Entry>>(&mut self, _name: &str, hash: &Hash, entries: I) {
        match self
            .create(hash)
            .and_then(|file| Self::write_group(file, entries))
        {
            Ok(()) => {}
            Err(err) => {
                eprintln!("error while creating group: {}", err);
            }
        }
    }
}

struct NoopSink;

impl Sink for NoopSink {
    #[inline]
    fn advance(&mut self, _: u64) {
        // no-op
    }

    #[inline]
    fn write(&mut self, _ty: SampleType, _samples: &[Sample]) {}
}
