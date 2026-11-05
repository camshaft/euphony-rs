use crate::{
    codec,
    record::{self, BufferList, ChunkReader, Digest, GroupEntry, GroupIndex, Stream, StreamKind},
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
/// `records-1` held each stream as one record; `records-2` holds it as a sequence of chunk records.
pub const FORMAT_DIR: &str = "records-2";

#[derive(Clone, Debug)]
pub struct Directory {
    path: Arc<PathBuf>,
}

/// A stream that its sink writes as it renders. Its values go to a temporary file as a sequence of
/// chunk records, each written once it fills, so the stream holds at most one chunk in memory. Once
/// it finishes, the file is moved under the content hash of its bytes.
///
/// A failed write of the file ends the stream: the temporary file is removed, later values are
/// dropped, and `finish` answers the failure, so that no part of the stream is stored.
pub struct Output {
    kind: StreamKind,
    chunk: Vec<u8>,
    file: Option<NamedTempFile>,
    hasher: Hasher,
    directory: Directory,
    hash: Option<Hash>,
    failure: Option<(io::ErrorKind, String)>,
}

impl storage::Output for Output {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        if self.failure.is_some() {
            return;
        }

        while !bytes.is_empty() {
            let room = record::CHUNK - self.chunk.len();
            let (now, rest) = bytes.split_at(room.min(bytes.len()));
            self.chunk.extend_from_slice(now);
            bytes = rest;

            if self.chunk.len() == record::CHUNK {
                if let Err(err) = self.write_chunk() {
                    self.fail(err);
                    return;
                }
            }
        }
    }

    #[inline]
    fn finish(&mut self) -> io::Result<Hash> {
        if let Some(hash) = self.hash {
            return Ok(hash);
        }

        if let Some(err) = self.failed() {
            return Err(err);
        }

        match self.persist() {
            Ok(hash) => {
                self.hash = Some(hash);
                Ok(hash)
            }
            Err(err) => {
                self.fail(err);
                Err(self.failed().unwrap())
            }
        }
    }
}

impl Output {
    /// Writes the values held so far to the stream's file as one chunk record. The values stay
    /// held until the whole frame is written.
    fn write_chunk(&mut self) -> io::Result<()> {
        if self.chunk.is_empty() {
            return Ok(());
        }

        // one frame of the stream's file, as `record::chunk_frame` lays it out
        let bytes = record::encode(&Stream::new(self.kind, self.chunk.clone()))?;
        let len = (bytes.len() as u64).to_le_bytes();

        if self.file.is_none() {
            self.file = Some(NamedTempFile::new_in(self.directory.path())?);
        }
        let file = self.file.as_mut().unwrap();
        file.write_all(&len)?;
        file.write_all(&bytes)?;
        self.hasher.update(&len);
        self.hasher.update(&bytes);

        // the buffer keeps its capacity for the next chunk
        self.chunk.clear();

        Ok(())
    }

    /// Writes the last chunk and moves the stream's file under the content hash of its bytes.
    fn persist(&mut self) -> io::Result<Hash> {
        self.write_chunk()?;

        let file = match self.file.take() {
            Some(file) => file,
            // a stream without values is a file without chunks
            None => NamedTempFile::new_in(self.directory.path())?,
        };
        let hash = *self.hasher.finalize().as_bytes();

        self.directory.persist_new(&hash, file)?;

        Ok(hash)
    }

    /// Ends the stream on `err`: the temporary file is removed as it drops, and the values held
    /// are dropped.
    fn fail(&mut self, err: io::Error) {
        log::error!("could not write a {:?} stream: {}", self.kind, err);
        self.failure = Some((err.kind(), err.to_string()));
        self.file = None;
        self.chunk = Vec::new();
    }

    /// The failure that ended the stream, where one did.
    fn failed(&self) -> Option<io::Error> {
        self.failure
            .as_ref()
            .map(|(kind, message)| io::Error::new(*kind, message.clone()))
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
        if self.hash_path(hash).exists() {
            return Ok(());
        }

        self.persist_new(hash, self.temporary(bytes)?)
    }

    /// Moves `file`, a temporary file of this directory, under `hash` where no file holds that
    /// name yet; where one does, the temporary file is removed.
    fn persist_new(&self, hash: &Hash, file: NamedTempFile) -> io::Result<()> {
        let path = self.hash_path(hash);
        if path.exists() {
            return Ok(());
        }

        match file.persist_noclobber(path) {
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
            let mut chunks = ChunkReader::new(self.open_raw(&hash)?, StreamKind::Samples);
            let mut samples = vec![];
            while let Some(body) = chunks.next_chunk()? {
                samples.extend(record::samples_from_body(&body));
            }
            let samples = Arc::from(samples);
            buffers.push(CachedBuffer { samples, hash });
        }

        Ok(Some(buffers))
    }
}

impl Storage for Directory {
    type Output = Output;
    type Reader = io::BufReader<fs::File>;
    type Group = GroupReader;
    type Sink = codec::Reader<io::BufReader<fs::File>>;

    fn create(&self, kind: StreamKind) -> Self::Output {
        Output {
            kind,
            chunk: vec![],
            file: None,
            hasher: Hasher::new(),
            directory: self.clone(),
            hash: None,
            failure: None,
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
            for part in samples.chunks(record::CHUNK / record::SAMPLE) {
                out.write(&record::samples_body(part));
            }
            let hash = out.finish().expect("could not write a buffer's stream");
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
    use crate::{dc::LeakDc, record::SinkIndex, storage::Output as _};
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

    /// The bodies of the chunk records of the stream named `hash`, in order.
    fn chunks(directory: &Directory, hash: &Hash, kind: StreamKind) -> Vec<Vec<u8>> {
        let mut reader = ChunkReader::new(directory.open_raw(hash).unwrap(), kind);
        let mut chunks = vec![];
        while let Some(chunk) = reader.next_chunk().unwrap() {
            chunks.push(chunk);
        }
        chunks
    }

    fn assert_coordinate(read: Cartesian<f64>, written: Polar<f64>) {
        let written: Cartesian<f64> = written.into();
        assert_eq!(
            (read.x, read.y, read.z),
            (
                written.x as f32 as f64,
                written.y as f32 as f64,
                written.z as f32 as f64
            )
        );
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
            assert_coordinate(sample.coordinate, coord);
        }

        // the index is a typed record, and each stream a chunk record whose body holds its values
        // in little-endian order
        let index: SinkIndex = directory.read_record(&hash).unwrap();
        let samples = chunks(&directory, &index.samples, StreamKind::Samples);
        assert_eq!(samples, vec![record::samples_body(&expected)]);

        let runs = chunks(&directory, &index.coordinates, StreamKind::Coordinates);
        // near twice, far twice, then near once
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].len(), 3 * record::COORDINATE_RUN);
        assert_eq!(record::read_coordinate_run(&runs[0][..record::COORDINATE_RUN]).0, 1);

        // a stream's name is the content hash of its file's bytes, the frames of its chunk records
        let bytes = fs::read(directory.hash_path(&index.samples)).unwrap();
        assert_eq!(record::content_hash(&bytes), *index.samples);
        assert_eq!(
            bytes,
            record::chunk_frame(&Stream::new(StreamKind::Samples, samples[0].clone())).unwrap()
        );
    }

    #[test]
    fn a_sink_longer_than_a_chunk_reads_back_across_its_chunks() {
        let (_dir, directory) = directory();
        let hash = [8; 32];
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

        // two whole chunks of samples and three more; the coordinate changes at each sample, so
        // each sample is a coordinate run of its own and the runs span more chunks
        let len = 2 * (record::CHUNK / record::SAMPLE) + 3;
        let input: Vec<(f64, Polar<f64>)> = (0..len)
            .map(|i| {
                let value = i as f64 / len as f64 - 0.5;
                (value, if i % 2 == 0 { near } else { far })
            })
            .collect();

        {
            let mut writer = codec::Writer::new(directory.clone(), hash);
            Sink::write(&mut writer, input.iter().copied());
        }

        let mut leak_dc = LeakDc::default();
        let expected: Vec<f64> = input.iter().map(|(value, _)| leak_dc.apply(*value)).collect();

        let index: SinkIndex = directory.read_record(&hash).unwrap();
        let samples = chunks(&directory, &index.samples, StreamKind::Samples);
        assert_eq!(
            samples.iter().map(Vec::len).collect::<Vec<_>>(),
            vec![record::CHUNK, record::CHUNK, 3 * record::SAMPLE]
        );

        let runs = chunks(&directory, &index.coordinates, StreamKind::Coordinates);
        let runs_per_chunk = record::CHUNK / record::COORDINATE_RUN;
        assert_eq!(runs.len(), (len + runs_per_chunk - 1) / runs_per_chunk);
        assert!(runs.iter().all(|chunk| chunk.len() <= record::CHUNK));
        assert_eq!(
            runs.iter().map(Vec::len).sum::<usize>(),
            len * record::COORDINATE_RUN
        );

        let read: Vec<_> = directory
            .open_sink(&hash)
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(read.len(), len);

        for ((sample, value), (_, coord)) in read.iter().zip(&expected).zip(input.iter().copied())
        {
            assert_eq!(sample.value, *value);
            assert_coordinate(sample.coordinate, coord);
        }
    }

    #[test]
    fn a_stream_holds_at_most_one_chunk_while_it_is_written() {
        let (_dir, directory) = directory();
        let mut out = directory.create(StreamKind::Samples);
        let samples: Vec<f64> = (0..3 * (record::CHUNK / record::SAMPLE) + 1)
            .map(|i| i as f64)
            .collect();

        out.write(&record::samples_body(&samples));

        // three chunks went to the stream's file as they filled; the last sample waits for more
        assert_eq!(out.chunk.len(), record::SAMPLE);
        assert!(out.file.is_some());

        let hash = out.finish().unwrap();
        assert!(out.chunk.is_empty());
        assert_eq!(out.finish().unwrap(), hash);

        let read = chunks(&directory, &hash, StreamKind::Samples);
        assert_eq!(read.len(), 4);
        let read: Vec<f64> = read
            .iter()
            .flat_map(|chunk| record::samples_from_body(chunk).collect::<Vec<_>>())
            .collect();
        assert_eq!(read, samples);
    }

    /// The names of the files the store's directory holds.
    fn files(directory: &Directory) -> Vec<std::ffi::OsString> {
        fs::read_dir(directory.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect()
    }

    #[test]
    fn a_stream_whose_file_cant_be_made_isnt_stored() {
        let (_dir, directory) = directory();
        let mut out = directory.create(StreamKind::Samples);

        // the store's directory is gone when the first chunk fills
        fs::remove_dir_all(directory.path()).unwrap();
        out.write(&record::samples_body(&vec![0.5; record::CHUNK / record::SAMPLE + 1]));
        assert!(out.failure.is_some());
        assert!(out.chunk.is_empty());

        // later values are dropped, and `finish` answers the failure each time it is called
        fs::create_dir_all(directory.path()).unwrap();
        out.write(&record::samples_body(&[0.25]));
        assert!(out.chunk.is_empty());
        assert_eq!(out.finish().unwrap_err().kind(), io::ErrorKind::NotFound);
        assert_eq!(out.finish().unwrap_err().kind(), io::ErrorKind::NotFound);
        assert!(files(&directory).is_empty());
    }

    #[test]
    fn a_stream_whose_file_is_lost_before_it_finishes_isnt_stored() {
        let (_dir, directory) = directory();
        let mut out = directory.create(StreamKind::Samples);

        // the first chunk goes to the temporary file, and then the file is lost
        out.write(&record::samples_body(&vec![0.5; record::CHUNK / record::SAMPLE + 1]));
        assert!(out.file.is_some());
        fs::remove_dir_all(directory.path()).unwrap();
        fs::create_dir_all(directory.path()).unwrap();

        assert!(out.finish().is_err());
        assert!(out.file.is_none());
        assert!(out.finish().is_err());
        assert!(files(&directory).is_empty());
    }

    #[test]
    fn a_sink_whose_stream_fails_isnt_stored_or_cached() {
        let (_dir, directory) = directory();
        let hash = [1; 32];
        let near = Polar {
            azimuth: 0.0,
            inclination: 0.0,
            radius: 1.0,
        };

        {
            let mut writer = codec::Writer::new(directory.clone(), hash);

            // the sample stream's first chunk fails, as on a full disk, and the store's directory
            // is back before the sink drops, so only the failure keeps its index out
            fs::remove_dir_all(directory.path()).unwrap();
            let len = record::CHUNK / record::SAMPLE + 1;
            Sink::write(&mut writer, (0..len).map(|_| (0.5, near)));
            fs::create_dir_all(directory.path()).unwrap();
        }

        assert!(!directory.is_cached(&hash));
        // the coordinate stream finished, and stays as a file that no index names
        assert_eq!(files(&directory).len(), 1);
    }

    #[test]
    fn a_sink_whose_rendering_panics_isnt_cached() {
        let (_dir, directory) = directory();
        let hash = [9; 32];
        let near = Polar {
            azimuth: 0.0,
            inclination: 0.0,
            radius: 1.0,
        };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut writer = codec::Writer::new(directory.clone(), hash);

            // the sample stream's first chunk is in its temporary file when the panic starts
            let len = record::CHUNK / record::SAMPLE + 1;
            Sink::write(&mut writer, (0..len).map(|_| (0.5, near)));
            assert!(!files(&directory).is_empty());

            panic!("a node panics while the sink renders");
        }));
        assert!(result.is_err());

        // the writer dropped while the panic unwound, so it didn't store a stream or the index
        assert!(!directory.is_cached(&hash));
        assert!(files(&directory).is_empty());
    }

    #[test]
    fn a_sink_whose_render_doesnt_finish_isnt_cached() {
        let (_dir, directory) = directory();
        let hash = [10; 32];
        let near = Polar {
            azimuth: 0.0,
            inclination: 0.0,
            radius: 1.0,
        };

        {
            let mut writer = codec::Writer::new(directory.clone(), hash);

            // the sample stream's first chunk is in its temporary file when the render fails
            let len = record::CHUNK / record::SAMPLE + 1;
            Sink::write(&mut writer, (0..len).map(|_| (0.5, near)));
            assert!(!files(&directory).is_empty());

            Sink::discard(&mut writer);
        }

        // the renderer discarded the writer, so it didn't store a stream or the index
        assert!(!directory.is_cached(&hash));
        assert!(files(&directory).is_empty());
    }

    #[test]
    fn a_sink_the_renderer_discards_isnt_cached() {
        let (_dir, mut directory) = directory();
        let hash = [11; 32];

        // the processor the store spawns for the sink passes the discard on to its writer: an
        // empty sink would otherwise be stored as cached as it drops
        let mut sink = directory.sink(&hash);
        sink.discard();
        drop(sink);

        assert!(!directory.is_cached(&hash));
        assert!(files(&directory).is_empty());
    }

    #[test]
    fn an_empty_sink_reads_back_no_samples() {
        let (_dir, directory) = directory();
        let hash = [6; 32];

        drop(codec::Writer::new(directory.clone(), hash));

        let index: SinkIndex = directory.read_record(&hash).unwrap();
        assert!(chunks(&directory, &index.samples, StreamKind::Samples).is_empty());
        assert!(fs::read(directory.hash_path(&index.samples)).unwrap().is_empty());
        assert_eq!(directory.open_sink(&hash).unwrap().count(), 0);
    }

    #[test]
    fn a_sink_whose_stream_file_is_cut_is_an_error() {
        let (_dir, directory) = directory();
        let hash = [2; 32];
        let near = Polar {
            azimuth: 0.0,
            inclination: 0.0,
            radius: 1.0,
        };

        {
            let mut writer = codec::Writer::new(directory.clone(), hash);
            Sink::write(&mut writer, [(0.5, near), (0.25, near)].into_iter());
        }

        let index: SinkIndex = directory.read_record(&hash).unwrap();
        let path = directory.hash_path(&index.samples);
        let bytes = fs::read(&path).unwrap();
        fs::write(&path, &bytes[..bytes.len() - 1]).unwrap();

        let mut sink = directory.open_sink(&hash).unwrap();
        assert_eq!(
            sink.next().unwrap().err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
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
        let samples = samples.finish().unwrap();

        let mut coordinates = directory.create(StreamKind::Coordinates);
        coordinates.write(&record::coordinate_run(0, Cartesian::default()));
        let coordinates = coordinates.finish().unwrap();

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

        // the first channel spans two chunks
        let long: Vec<f64> = (0..record::CHUNK / record::SAMPLE + 1)
            .map(|i| i as f64)
            .collect();
        let long_len = long.len();

        let converted = directory
            .buffer(source, 48_000, |_| {
                Ok::<_, io::Error>(vec![long, vec![1.0]])
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
        assert_eq!(cached[0].samples.len(), long_len);
        assert_eq!(
            chunks(&directory, &cached[0].hash, StreamKind::Samples).len(),
            2
        );
        assert_eq!(&*cached[1].samples, &[1.0]);
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
