use crate::{
    codec,
    ext::*,
    storage::{self, Output as _, Storage},
};
use base64::prelude::*;
use blake3::Hasher;
use euphony_compiler::{midi, Entry, Hash, Writer};
use euphony_node::{BoxProcessor, Sink};
use euphony_units::coordinates::Polar;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::{NamedTempFile, TempPath};

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
        let len = BASE64_URL_SAFE_NO_PAD.encode_slice(hash, &mut out).unwrap();
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
        file: fs::File,
        buffer: Vec<u8>,
        path: Option<TempPath>,
        state: State,
    },
}

#[inline]
fn write_buffered(
    file: &mut fs::File,
    buffer: &mut Vec<u8>,
    state: &mut State,
    bytes: &[u8],
) -> io::Result<()> {
    if bytes.len() > 1024 {
        flush_buffered(file, buffer, state)?;

        state.hasher.update(bytes);
        file.write_all(bytes)?;
        return Ok(());
    }

    if buffer.capacity() == 0 {
        buffer.reserve(8 * 1024);
    }
    buffer.extend_from_slice(bytes);

    if buffer.len() > 8 * 1024 {
        flush_buffered(file, buffer, state)?;
    }

    Ok(())
}

#[inline]
fn flush_buffered(file: &mut fs::File, buffer: &mut Vec<u8>, state: &mut State) -> io::Result<()> {
    if buffer.is_empty() {
        return Ok(());
    }

    state.hasher.update(buffer);
    file.write_all(buffer)?;
    buffer.clear();

    Ok(())
}

impl storage::Output for Output {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }

        let result = match &mut self.0 {
            OState::PreHashed { file, .. } => file.write_all(bytes),
            OState::Incremental {
                file,
                buffer,
                state,
                ..
            } => write_buffered(file, buffer, state, bytes),
        };

        if let Err(err) = result {
            log::error!("could write samples to output: {}", err);
        }
    }

    #[inline]
    fn finish(&mut self) -> Hash {
        let result = match &mut self.0 {
            OState::PreHashed { file, hash } => file.flush().map(|_| *hash),
            OState::Incremental {
                file,
                buffer,
                state,
                path,
            } => {
                let tmp_path = path.take().expect("cannot finalize twice");

                let result = flush_buffered(file, buffer, state);

                result.and_then(|_| file.flush()).and_then(|_| {
                    let hash = *state.hasher.finalize().as_bytes();
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
        let len = BASE64_URL_SAFE_NO_PAD.encode_slice(hash, &mut out).unwrap();
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

    fn write_midi(&self, midi: &midi::Writer) -> io::Result<()> {
        if midi.is_empty() {
            return Ok(());
        }

        if let Some(file) = self.open(midi.hash())? {
            // no need to buffer the contents
            let mut file = file.into_inner()?;
            file.write_all(midi)?;
        }

        Ok(())
    }
}

impl Storage for Directory {
    type Output = Output;
    type Reader = io::BufReader<fs::File>;
    type Group = GroupReader;
    type Sink = codec::Reader<Self::Reader>;

    fn create(&self) -> Self::Output {
        let (file, path) = NamedTempFile::new().unwrap().into_parts();
        let state = self.state.clone();
        Output(OState::Incremental {
            file,
            buffer: vec![],
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

    fn group<I: Iterator<Item = Entry>>(
        &mut self,
        name: &str,
        hash: &Hash,
        entries: I,
        midi: &midi::Writer,
    ) {
        match self
            .open(hash)
            .and_then(|file| Self::write_group(file, entries))
            .and_then(|_| self.write_midi(midi))
        {
            Ok(()) => {}
            Err(err) => {
                log::error!("could not create group {:?}: {}", name, err);
            }
        }
    }

    fn buffer<
        F: FnOnce(
            Box<dyn euphony_compiler::BufferReader>,
        ) -> euphony_compiler::Result<Vec<euphony_compiler::ConvertedBuffer>, E>,
        E,
    >(
        &self,
        path: &str,
        sample_rate: u64,
        init: F,
    ) -> euphony_compiler::Result<Vec<euphony_compiler::CachedBuffer>, E> {
        let mut hasher = Hasher::new();
        hasher.update(&sample_rate.to_le_bytes());
        hasher.update(path.as_bytes());
        let path_hash = *hasher.finalize().as_bytes();
        let contents = std::fs::File::open(path).unwrap();

        let mut buffers = vec![];

        if let Ok(mut reader) = self.open_raw(&path_hash) {
            let contents_modified = contents.metadata().unwrap().modified().unwrap();
            let hashes_modified = reader.get_ref().metadata().unwrap().modified().unwrap();

            // only return if the hashes are newer than the original contents
            if hashes_modified >= contents_modified {
                while let Some(hash) = read_result(reader.read_hash()).unwrap() {
                    let mut input = self.open_raw(&hash).unwrap();
                    let mut samples = vec![];
                    while let Some(sample) = read_result(input.read_f64()).unwrap() {
                        samples.push(sample);
                    }
                    let samples = Arc::from(samples);
                    buffers.push(euphony_compiler::CachedBuffer { samples, hash });
                }

                return Ok(buffers);
            }
        }

        let res = init(Box::new(contents))?;

        let mut hashes = self.open(&path_hash).unwrap();
        for samples in res {
            let mut out = self.create();
            let ptr = samples.as_ptr();
            let len = samples.len() * 8;
            let bytes = unsafe { core::slice::from_raw_parts(ptr as _, len) };
            out.write(bytes);

            let hash = out.finish();
            if let Some(hashes) = hashes.as_mut() {
                hashes.write_all(&hash).unwrap();
            }

            let samples = Arc::from(samples);
            buffers.push(euphony_compiler::CachedBuffer { samples, hash });
        }

        Ok(buffers)
    }
}

fn read_result<T>(result: io::Result<T>) -> io::Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(err) => Err(err),
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
