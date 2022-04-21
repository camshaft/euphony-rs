use crate::codec::Codec;
use base64::URL_SAFE_NO_PAD;
use euphony_compiler::{Entry, Hash, Sample, Writer};
use euphony_node::{BoxProcessor, SampleType, Sink};
use std::{fs, io, marker::PhantomData, path::PathBuf};

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
        let name = base64::encode_config(hash, URL_SAFE_NO_PAD);
        let mut path = self.path.join(name);
        path.set_extension(C::EXTENSION);
        path
    }
}

impl<C: Codec<File> + Sync> Writer for Directory<C> {
    fn is_cached(&self, hash: &[u8; 32]) -> bool {
        self.path(hash).exists()
    }

    fn sink(&mut self, hash: [u8; 32]) -> BoxProcessor {
        let path = self.path(&hash);
        match fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .map(io::BufWriter::new)
            .and_then(C::new)
        {
            Ok(file) => file.spawn(),
            Err(err) => {
                eprintln!("error while creating sink: {}", err);
                NoopSink.spawn()
            }
        }
    }

    fn group<I: Iterator<Item = Entry>>(&mut self, _id: u64, _name: &str, _entries: I) {}
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
