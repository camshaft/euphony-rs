use crate::codec::{self, Codec};
use base64::URL_SAFE_NO_PAD;
use blake3::Hasher;
use euphony_compiler::{sample::DefaultSample as Sample, Entry, Hash, Writer};
use euphony_node::{BoxProcessor, SampleType, Sink};
use std::{
    fs,
    io::{self, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::{NamedTempFile, TempPath};

use super::Storage;

pub type File = io::BufWriter<fs::File>;

#[derive(Clone, Debug)]
pub struct Directory<C: Codec<Output>> {
    state: State,
    codec: PhantomData<C>,
}

#[derive(Clone, Debug)]
struct State {
    path: Arc<PathBuf>,
    hasher: Hasher,
}

impl State {
    fn hash_path(&self, hash: &Hash) -> PathBuf {
        let mut out = [b'A'; 64];
        let len = base64::encode_config_slice(hash, URL_SAFE_NO_PAD, &mut out);
        let out = unsafe { core::str::from_utf8_unchecked_mut(&mut out) };
        let out = &out[..len];
        self.path.join(out)
    }
}

pub struct Output(OState);

#[allow(clippy::large_enum_variant)]
enum OState {
    PreHashed {
        file: File,
        hash: Hash,
    },
    Incremental {
        file: File,
        path: Option<TempPath>,
        state: State,
    },
}

impl codec::Output for Output {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let result = match &mut self.0 {
            OState::PreHashed { file, .. } => file.write_all(bytes),
            OState::Incremental { file, state, .. } => {
                state.hasher.update(bytes);
                file.write_all(bytes)
            }
        };

        // TODO log result to slog
        if let Err(err) = result {
            eprintln!("error writing samples to output: {:?}", err);
        }
    }

    #[inline]
    fn finish(&mut self) -> Hash {
        let result = match &mut self.0 {
            OState::PreHashed { file, hash } => file.flush().map(|_| *hash),
            OState::Incremental { file, state, path } => {
                let tmp_path = path.take().expect("cannot finalize twice");
                let hash = *state.hasher.finalize().as_bytes();
                file.flush().and_then(|_| {
                    let new_path = state.hash_path(&hash);

                    tmp_path
                        .persist(new_path)
                        .map(|_| hash)
                        .map_err(|e| e.error)
                })
            }
        };

        result.expect("could not finish file")
    }
}

impl<C: Codec<Output>> Default for Directory<C> {
    fn default() -> Self {
        Self::new(PathBuf::from("target/euphony/contents"))
    }
}

impl<C: Codec<Output>> Directory<C> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            state: State {
                path: Arc::new(path),
                hasher: Hasher::new(),
            },
            codec: PhantomData,
        }
    }

    pub fn path(&self) -> &Path {
        &self.state.path
    }

    fn hash_path(&self, hash: &Hash) -> PathBuf {
        let mut out = [b'A'; 64];
        let len = base64::encode_config_slice(hash, URL_SAFE_NO_PAD, &mut out);
        let out = unsafe { core::str::from_utf8_unchecked_mut(&mut out) };
        let out = &out[..len];
        self.state.path.join(out)
    }

    fn open(&self, hash: &Hash) -> io::Result<Option<File>> {
        let path = self.hash_path(hash);
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
                file.write_all(&entry.sample_offset.to_le_bytes())?;
                file.write_all(&entry.hash)?;
            }
        }
        Ok(())
    }
}

impl<C: Codec<Output>> Storage for Directory<C> {
    type Output = Output;

    fn create(&mut self) -> Self::Output {
        let (file, path) = NamedTempFile::new().unwrap().into_parts();
        let file = io::BufWriter::new(file);
        let state = self.state.clone();
        Output(OState::Incremental {
            file,
            state,
            path: Some(path),
        })
    }
}

impl<C: Codec<Output>> Writer for Directory<C> {
    fn is_cached(&self, hash: &Hash) -> bool {
        self.hash_path(hash).exists()
    }

    fn sink(&mut self, hash: &Hash) -> BoxProcessor {
        if let Some(file) = self.open(hash).unwrap() {
            let output = Output(OState::PreHashed { file, hash: *hash });
            C::new(self, output).spawn()
        } else {
            NoopSink.spawn()
        }
    }

    fn group<I: Iterator<Item = Entry>>(&mut self, _name: &str, hash: &Hash, entries: I) {
        match self
            .open(hash)
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
