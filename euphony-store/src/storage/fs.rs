use crate::{
    codec,
    record::{self, BufferList, Digest, GroupEntry, GroupIndex, Stream, StreamKind},
    storage::{self, Output as _, Storage},
};
use base64::prelude::*;
use blake3::Hasher;
use euphony_compiler::{midi, CachedBuffer, Entry, Hash, Writer};
use euphony_node::{BoxProcessor, Sink};
use euphony_units::coordinates::Polar;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
    vec,
};
use tempfile::NamedTempFile;

/// The directory under a store's path that holds the files of the typed record format. The files
/// an earlier format wrote stay beside it, so a project's earlier output is rendered again once.
pub const FORMAT_DIR: &str = "records-1";

#[derive(Clone, Debug)]
pub struct Directory {
    path: Arc<PathBuf>,
}

/// A stream that its sink writes as it renders. Its record is written once it finishes, under the
/// content hash of the record's bytes.
pub struct Output {
    kind: StreamKind,
    body: Vec<u8>,
    directory: Directory,
    hash: Option<Hash>,
}

impl storage::Output for Output {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.body.extend_from_slice(bytes);
    }

    #[inline]
    fn finish(&mut self) -> Hash {
        if let Some(hash) = self.hash {
            return hash;
        }

        let body = core::mem::take(&mut self.body);
        let bytes = record::encode(&Stream::new(self.kind, body)).expect("a stream encodes");
        let hash = record::content_hash(&bytes);

        self.directory
            .write_new(&hash, &bytes)
            .expect("could not finish file");

        self.hash = Some(hash);
        hash
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
            path: Arc::new(path.join(FORMAT_DIR)),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn hash_path(&self, hash: &Hash) -> PathBuf {
        let mut out = [b'A'; 64];
        let len = BASE64_URL_SAFE_NO_PAD.encode_slice(hash, &mut out).unwrap();
        let out = unsafe { core::str::from_utf8_unchecked_mut(&mut out) };
        let out = &out[..len];
        self.path.join(out)
    }

    /// Writes `bytes` under `hash` where no file holds that name yet. The bytes go to a temporary
    /// file in the same directory first, so a reader never reads part of a record.
    fn write_new(&self, hash: &Hash, bytes: &[u8]) -> io::Result<()> {
        let path = self.hash_path(hash);
        if path.exists() {
            return Ok(());
        }

        match self.temporary(bytes)?.persist_noclobber(path) {
            Ok(_) => Ok(()),
            Err(err) if err.error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            Err(err) => Err(err.error),
        }
    }

    /// Writes `bytes` under `hash` in place of any file that holds that name.
    fn write_replace(&self, hash: &Hash, bytes: &[u8]) -> io::Result<()> {
        self.temporary(bytes)?
            .persist(self.hash_path(hash))
            .map(|_| ())
            .map_err(|err| err.error)
    }

    fn temporary(&self, bytes: &[u8]) -> io::Result<NamedTempFile> {
        let mut file = NamedTempFile::new_in(&*self.path)?;
        file.write_all(bytes)?;
        file.flush()?;
        Ok(file)
    }

    fn write_group<I: Iterator<Item = Entry>>(&self, hash: &Hash, entries: I) -> io::Result<()> {
        let index = GroupIndex {
            entries: entries
                .map(|entry| GroupEntry {
                    sample_offset: entry.sample_offset,
                    sink: Digest(entry.hash),
                })
                .collect(),
        };

        self.write_new(hash, &record::encode(&index)?)
    }

    fn write_midi(&self, midi: &midi::Writer) -> io::Result<()> {
        if midi.is_empty() {
            return Ok(());
        }

        self.write_new(midi.hash(), &midi[..])
    }

    /// The buffers the list named `list` holds, where the list is newer than the source's contents.
    fn cached_buffers(
        &self,
        list: &Hash,
        contents: &fs::File,
    ) -> io::Result<Option<Vec<CachedBuffer>>> {
        let Ok(list_metadata) = fs::metadata(self.hash_path(list)) else {
            return Ok(None);
        };

        if list_metadata.modified()? < contents.metadata()?.modified()? {
            return Ok(None);
        }

        let list: BufferList = self.read_record(list)?;
        let mut buffers = vec![];

        for Digest(hash) in list.channels {
            let body = self
                .read_record::<Stream>(&hash)?
                .into_body(StreamKind::Samples)?;
            let samples = record::samples_from_body(&body).collect();
            buffers.push(CachedBuffer { samples, hash });
        }

        Ok(Some(buffers))
    }
}

impl Storage for Directory {
    type Output = Output;
    type Reader = io::BufReader<fs::File>;
    type Group = GroupReader;
    type Sink = codec::Reader;

    fn create(&self, kind: StreamKind) -> Self::Output {
        Output {
            kind,
            body: vec![],
            directory: self.clone(),
            hash: None,
        }
    }

    fn write_record(&self, hash: &Hash, bytes: &[u8]) -> io::Result<()> {
        self.write_new(hash, bytes)
    }

    fn open_raw(&self, hash: &Hash) -> io::Result<Self::Reader> {
        let path = self.hash_path(hash);
        let file = fs::File::open(path)?;
        let file = io::BufReader::new(file);
        Ok(file)
    }

    fn open_group(&self, hash: &Hash) -> io::Result<Self::Group> {
        let index: GroupIndex = self.read_record(hash)?;
        Ok(GroupReader(index.entries.into_iter()))
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
        if self.is_cached(hash) {
            NoopSink.spawn()
        } else {
            codec::Writer::new(self.clone(), *hash).spawn()
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
            .write_group(hash, entries)
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
    ) -> euphony_compiler::Result<Vec<CachedBuffer>, E> {
        let mut hasher = Hasher::new();
        hasher.update(&sample_rate.to_le_bytes());
        hasher.update(path.as_bytes());
        let path_hash = *hasher.finalize().as_bytes();
        let contents = std::fs::File::open(path).unwrap();

        match self.cached_buffers(&path_hash, &contents) {
            Ok(Some(buffers)) => return Ok(buffers),
            Ok(None) => {}
            Err(err) => {
                log::warn!("converting the buffer {:?} again: {}", path, err);
            }
        }

        let res = init(Box::new(contents))?;

        let mut buffers = vec![];
        let mut channels = vec![];

        for samples in res {
            let mut out = self.create(StreamKind::Samples);
            out.write(&record::samples_body(&samples));
            let hash = out.finish();
            channels.push(Digest(hash));

            let samples = Arc::from(samples);
            buffers.push(CachedBuffer { samples, hash });
        }

        let list = record::encode(&BufferList { channels }).unwrap();
        self.write_replace(&path_hash, &list).unwrap();

        Ok(buffers)
    }
}

struct NoopSink;

impl Sink for NoopSink {
    #[inline]
    fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, _samples: S) {}
}

pub struct GroupReader(vec::IntoIter<GroupEntry>);

impl Iterator for GroupReader {
    type Item = io::Result<Entry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.0.next()?;
        Some(Ok(Entry {
            sample_offset: entry.sample_offset,
            hash: entry.sink.0,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dc::LeakDc, record::SinkIndex};
    use euphony_compiler::ConvertedBuffer;
    use euphony_units::coordinates::Cartesian;
    use std::{
        cell::Cell,
        time::{Duration, SystemTime},
    };

    fn directory() -> (tempfile::TempDir, Directory) {
        let dir = tempfile::tempdir().unwrap();
        let directory = Directory::new(dir.path().to_owned());
        fs::create_dir_all(directory.path()).unwrap();
        (dir, directory)
    }

    #[test]
    fn the_records_go_in_a_directory_of_their_format() {
        let directory = Directory::new(PathBuf::from("contents"));
        assert_eq!(directory.path(), Path::new("contents").join(FORMAT_DIR));
    }

    #[test]
    fn a_sink_reads_back_its_samples_and_coordinates() {
        let (_dir, directory) = directory();
        let hash = [7; 32];
        let near = Polar {
            azimuth: 0.0,
            inclination: 0.0,
            radius: 1.0,
        };
        let far = Polar {
            azimuth: 0.25,
            inclination: 0.0,
            radius: 2.0,
        };
        let input = [
            (0.5, near),
            (-0.5, near),
            (0.25, far),
            (0.125, far),
            (1.0, near),
        ];

        assert!(!directory.is_cached(&hash));
        {
            let mut writer = codec::Writer::new(directory.clone(), hash);
            Sink::write(&mut writer, input.iter().copied());
        }
        assert!(directory.is_cached(&hash));

        let mut leak_dc = LeakDc::default();
        let expected: Vec<f64> = input.iter().map(|(value, _)| leak_dc.apply(*value)).collect();

        let read: Vec<_> = directory
            .open_sink(&hash)
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(read.len(), input.len());

        for ((sample, value), (_, coord)) in read.iter().zip(&expected).zip(input) {
            assert_eq!(sample.value, *value);
            let coord: Cartesian<f64> = coord.into();
            assert_eq!(
                (
                    sample.coordinate.x,
                    sample.coordinate.y,
                    sample.coordinate.z
                ),
                (
                    coord.x as f32 as f64,
                    coord.y as f32 as f64,
                    coord.z as f32 as f64
                )
            );
        }

        // the index and its streams are typed records, the streams' bodies in little-endian order
        let index: SinkIndex = directory.read_record(&hash).unwrap();
        let samples = directory
            .read_record::<Stream>(&index.samples)
            .unwrap()
            .into_body(StreamKind::Samples)
            .unwrap();
        assert_eq!(samples, record::samples_body(&expected));

        let runs = directory
            .read_record::<Stream>(&index.coordinates)
            .unwrap()
            .into_body(StreamKind::Coordinates)
            .unwrap();
        // near twice, far twice, then near once
        assert_eq!(runs.len(), 3 * record::COORDINATE_RUN);
        assert_eq!(record::read_coordinate_run(&runs[..record::COORDINATE_RUN]).0, 1);

        // a stream's name is the content hash of its record's canonical bytes
        let bytes = fs::read(directory.hash_path(&index.samples)).unwrap();
        assert_eq!(record::content_hash(&bytes), *index.samples);
        let decoded: Stream = record::decode(&bytes).unwrap();
        assert_eq!(record::encode(&decoded).unwrap(), bytes);
    }

    #[test]
    fn a_group_index_reads_back_its_entries() {
        let (_dir, mut directory) = directory();
        let hash = [3; 32];
        let entries = [
            Entry {
                sample_offset: 0,
                hash: [1; 32],
            },
            Entry {
                sample_offset: 96_000,
                hash: [2; 32],
            },
        ];

        directory.group("group", &hash, entries.iter().copied(), &midi::Writer::default());

        let read: Vec<_> = directory
            .open_group(&hash)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                (entry.sample_offset, entry.hash)
            })
            .collect();
        assert_eq!(read, vec![(0, [1; 32]), (96_000, [2; 32])]);

        let index: GroupIndex = directory.read_record(&hash).unwrap();
        assert_eq!(
            index.entries,
            vec![
                GroupEntry {
                    sample_offset: 0,
                    sink: Digest([1; 32]),
                },
                GroupEntry {
                    sample_offset: 96_000,
                    sink: Digest([2; 32]),
                },
            ]
        );
    }

    #[test]
    fn a_damaged_or_missing_file_is_an_error() {
        let (_dir, directory) = directory();
        let hash = [9; 32];

        assert_eq!(
            directory.open_group(&hash).err().unwrap().kind(),
            io::ErrorKind::NotFound
        );

        fs::write(directory.hash_path(&hash), b"not a record").unwrap();
        assert_eq!(
            directory.open_group(&hash).err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            directory.open_sink(&hash).err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn a_sink_whose_coordinates_end_early_is_an_error() {
        let (_dir, directory) = directory();
        let hash = [4; 32];

        let mut samples = directory.create(StreamKind::Samples);
        samples.write(&record::samples_body(&[0.5, 0.25]));
        let samples = samples.finish();

        let mut coordinates = directory.create(StreamKind::Coordinates);
        coordinates.write(&record::coordinate_run(0, Cartesian::default()));
        let coordinates = coordinates.finish();

        let index = record::SinkIndex {
            samples: Digest(samples),
            coordinates: Digest(coordinates),
        };
        directory
            .write_record(&hash, &record::encode(&index).unwrap())
            .unwrap();

        let mut sink = directory.open_sink(&hash).unwrap();
        assert!(sink.next().unwrap().is_ok());
        assert_eq!(
            sink.next().unwrap().err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn a_buffer_is_cached_as_a_list_of_sample_streams() {
        let (dir, directory) = directory();
        let source = dir.path().join("source.wav");
        fs::write(&source, b"source").unwrap();
        let source = source.to_str().unwrap();

        let converted = directory
            .buffer(source, 48_000, |_| {
                Ok::<_, io::Error>(vec![vec![0.5, -0.25], vec![1.0]])
            })
            .unwrap();
        let cached = directory
            .buffer(
                source,
                48_000,
                |_| -> euphony_compiler::Result<Vec<ConvertedBuffer>, io::Error> {
                    panic!("the cached list is read in place of the source")
                },
            )
            .unwrap();

        assert_eq!(converted.len(), 2);
        assert_eq!(cached.len(), 2);
        for (converted, cached) in converted.iter().zip(&cached) {
            assert_eq!(converted.hash, cached.hash);
            assert_eq!(converted.samples, cached.samples);
        }
        assert_eq!(&*cached[0].samples, &[0.5, -0.25]);
    }

    /// The path of the buffer list that `buffer` keeps for `source` at `sample_rate`.
    fn list_path(directory: &Directory, source: &str, sample_rate: u64) -> PathBuf {
        let mut hasher = Hasher::new();
        hasher.update(&sample_rate.to_le_bytes());
        hasher.update(source.as_bytes());
        directory.hash_path(hasher.finalize().as_bytes())
    }

    fn set_modified(path: &Path, time: SystemTime) {
        fs::OpenOptions::new()
            .write(true)
            .open(path)
            .unwrap()
            .set_modified(time)
            .unwrap();
    }

    fn read_from_the_cache(
        _: Box<dyn euphony_compiler::BufferReader>,
    ) -> euphony_compiler::Result<Vec<ConvertedBuffer>, io::Error> {
        panic!("the cached list is read in place of the source")
    }

    #[test]
    fn a_buffer_whose_source_is_newer_than_its_list_is_converted_again() {
        let (dir, directory) = directory();
        let source_path = dir.path().join("source.wav");
        fs::write(&source_path, b"source").unwrap();
        let source = source_path.to_str().unwrap();

        directory
            .buffer(source, 48_000, |_| {
                Ok::<_, io::Error>(vec![vec![0.5, -0.25]])
            })
            .unwrap();

        // the source changes after its list was written
        let now = SystemTime::now();
        let hour = Duration::from_secs(3600);
        set_modified(&list_path(&directory, source, 48_000), now - hour);
        set_modified(&source_path, now - hour / 2);

        let converted = Cell::new(false);
        let again = directory
            .buffer(source, 48_000, |_| {
                converted.set(true);
                Ok::<_, io::Error>(vec![vec![0.75, 0.125]])
            })
            .unwrap();
        assert!(converted.get(), "a list older than its source is converted again");
        assert_eq!(again.len(), 1);
        assert_eq!(&*again[0].samples, &[0.75, 0.125]);

        // the list written again is newer than the source, so the new samples come from the cache
        let cached = directory
            .buffer(source, 48_000, read_from_the_cache)
            .unwrap();
        assert_eq!(cached.len(), 1);
        assert_eq!(cached[0].hash, again[0].hash);
        assert_eq!(&*cached[0].samples, &[0.75, 0.125]);
    }

    #[test]
    fn a_damaged_buffer_list_is_converted_again() {
        let (dir, directory) = directory();
        let source_path = dir.path().join("source.wav");
        fs::write(&source_path, b"source").unwrap();
        let source = source_path.to_str().unwrap();

        directory
            .buffer(source, 48_000, |_| {
                Ok::<_, io::Error>(vec![vec![0.5, -0.25]])
            })
            .unwrap();

        // the list is damaged, and stays newer than the source, so `buffer` reads it
        let list = list_path(&directory, source, 48_000);
        fs::write(&list, b"not a record").unwrap();
        set_modified(&source_path, SystemTime::now() - Duration::from_secs(3600));

        let converted = Cell::new(false);
        let again = directory
            .buffer(source, 48_000, |_| {
                converted.set(true);
                Ok::<_, io::Error>(vec![vec![0.75, 0.125]])
            })
            .unwrap();
        assert!(converted.get(), "a damaged list is converted again");
        assert_eq!(again.len(), 1);
        assert_eq!(&*again[0].samples, &[0.75, 0.125]);

        // the damaged list was written again as a record
        let cached = directory
            .buffer(source, 48_000, read_from_the_cache)
            .unwrap();
        assert_eq!(cached.len(), 1);
        assert_eq!(&*cached[0].samples, &[0.75, 0.125]);
    }
}
