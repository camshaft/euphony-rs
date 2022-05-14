use crate::{codec, ext::*, storage};
use base64::URL_SAFE_NO_PAD;
use blake3::Hasher;
use euphony_compiler::{Entry, Hash, Writer};
use euphony_node::{BoxProcessor, Sink};
use euphony_units::coordinates::Polar;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::{NamedTempFile, TempPath};

use super::Storage;

pub type File = io::BufWriter<fs::File>;

#[derive(Clone, Debug)]
pub struct Directory {
    state: State,
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

impl storage::Output for Output {
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

impl Default for Directory {
    fn default() -> Self {
        Self::new(PathBuf::from("target/euphony/contents"))
    }
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            state: State {
                path: Arc::new(path),
                hasher: Hasher::new(),
            },
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

impl Storage for Directory {
    type Output = Output;
    type Reader = io::BufReader<fs::File>;
    type Group = GroupReader;
    type Sink = codec::Reader<Self::Reader>;

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

    fn open_raw(&self, hash: &Hash) -> io::Result<Self::Reader> {
        let path = self.hash_path(hash);
        let file = fs::File::open(path)?;
        let file = io::BufReader::new(file);
        Ok(file)
    }

    fn open_group(&self, hash: &Hash) -> io::Result<Self::Group> {
        let group = self.open_raw(hash)?;
        Ok(GroupReader(group))
    }

    fn open_sink(&self, hash: &Hash) -> io::Result<Self::Sink> {
        codec::Reader::new(self, hash)
    }
}

impl Writer for Directory {
    fn is_cached(&self, hash: &Hash) -> bool {
        self.hash_path(hash).exists()
    }

    fn sink(&mut self, hash: &Hash) -> BoxProcessor {
        if let Some(file) = self.open(hash).unwrap() {
            let output = Output(OState::PreHashed { file, hash: *hash });
            codec::Writer::new(self, output).spawn()
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
    fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, _samples: S) {}
}

pub struct GroupReader(io::BufReader<fs::File>);

impl Iterator for GroupReader {
    type Item = io::Result<Entry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.read_u64() {
            Ok(sample_offset) => Some(self.0.read_hash().map(|hash| Entry {
                sample_offset,
                hash,
            })),
            Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(err) => Some(Err(err)),
        }
    }
}
