use base64::URL_SAFE_NO_PAD;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fmt,
    io::Read,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

#[cfg(feature = "decode")]
use symphonia::core::codecs::CodecParameters;

#[cfg(feature = "decode")]
pub use symphonia;
#[cfg(feature = "decode")]
pub mod decode;
pub mod hash;

static TARGET_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = std::env::var("EUPHONY_TARGET_DIR").unwrap_or_else(|_| "target/euphony".to_owned());
    std::fs::create_dir_all(&dir).unwrap();
    PathBuf::from(dir).canonicalize().unwrap()
});

static BUFFER_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = TARGET_DIR.join("buffers");
    std::fs::create_dir_all(&dir).unwrap();
    dir
});

pub struct Buffer<S = &'static str> {
    source: S,
    values: Values,
}

impl<S: AsRef<str>> fmt::Debug for Buffer<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Buffer")
            .field(&self.source.as_ref())
            .finish()
    }
}

impl<S> Buffer<S> {
    pub const fn new(source: S) -> Self {
        Self {
            source,
            values: Values::new(),
        }
    }
}

impl<S: AsRef<str>> Buffer<S> {
    #[doc(hidden)]
    pub fn initialize<F: FnOnce(&Path, &str) -> u64>(&self, init: F) -> u64 {
        self.values
            .initialize(&BUFFER_DIR, self.source.as_ref(), init)
    }

    pub fn duration(&self) -> Duration {
        self.meta().duration()
    }

    pub fn channels(&self) -> u32 {
        self.meta().channels
    }

    pub fn channel(&self, index: u64) -> Channel<Self> {
        assert!(self.channels() as u64 > index);
        Channel(self, index)
    }

    fn meta(&self) -> &Meta {
        self.values.meta(&BUFFER_DIR, self.source.as_ref())
    }
}

#[cfg(feature = "host")]
impl Buffer<String> {
    pub fn init(msg: euphony_command::InitBuffer) -> std::io::Result<()> {
        let buffer = Self::new(msg.source);

        let meta = buffer
            .values
            .meta_path
            .get_or_init(|| PathBuf::from(msg.meta));

        let buffer_dir = meta.parent().expect("missing parent path");

        buffer.values.load_meta(buffer_dir, &buffer.source)?;

        Ok(())
    }
}

struct Values {
    meta_path: OnceCell<PathBuf>,
    contents_path: OnceCell<PathBuf>,
    meta: OnceCell<Meta>,
    id: OnceCell<u64>,
}

impl Values {
    const fn new() -> Self {
        Self {
            meta_path: OnceCell::new(),
            contents_path: OnceCell::new(),
            meta: OnceCell::new(),
            id: OnceCell::new(),
        }
    }

    fn initialize<F: FnOnce(&Path, &str) -> u64>(
        &self,
        buffer_dir: &Path,
        source: &str,
        init: F,
    ) -> u64 {
        *self.id.get_or_init(|| {
            let meta = self.meta(buffer_dir, source);
            let contents = self.contents_path(buffer_dir, source);
            init(contents, meta.ext.as_deref().unwrap_or(""))
        })
    }

    fn contents_path(&self, buffer_dir: &Path, source: &str) -> &Path {
        self.contents_path.get_or_init(|| {
            if source.starts_with("https://") || source.starts_with("http://") {
                return self.http(buffer_dir, source);
            }

            self.local(buffer_dir, source)
        })
    }

    #[cfg(not(feature = "http"))]
    fn http(&self, buffer_dir: &Path, source: &str) -> PathBuf {
        self.meta(buffer_dir, source).contents.to_owned()
    }

    #[cfg(feature = "http")]
    fn http(&self, buffer_dir: &Path, source: &str) -> PathBuf {
        let meta_path = self.meta_path(buffer_dir, source);
        if meta_path.exists() {
            return self.meta(buffer_dir, source).contents.to_owned();
        }

        log::info!(" Downloading {}", source);

        let ext = Path::new(source)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        hash::create(buffer_dir, ext, |writer| {
            reqwest::blocking::get(source)
                .unwrap()
                .error_for_status()
                .unwrap()
                .copy_to(writer)
                .unwrap();
            Ok(())
        })
        .unwrap()
    }

    fn local(&self, buffer_dir: &Path, source: &str) -> PathBuf {
        let meta_path = self.meta_path(buffer_dir, source);

        if meta_path.exists() {
            let source_modifed = modified(&source);
            let meta_modifed = modified(&self.meta_path(buffer_dir, source));
            if source_modifed <= meta_modifed {
                return self.meta(buffer_dir, source).contents.to_owned();
            } else {
                // remove the old path since the source has been updated
                let _ = std::fs::remove_file(meta_path);
            }
        }

        let mut file = std::fs::File::open(source).unwrap();
        let hash = hash_reader(&mut file);
        let path = hash_path(&BUFFER_DIR, &hash);
        std::fs::copy(source, &path).unwrap();
        path
    }

    fn meta_path(&self, buffer_dir: &Path, source: &str) -> &Path {
        self.meta_path.get_or_init(|| {
            let hash = blake3::hash(source.as_bytes());
            hash_path(buffer_dir, &hash)
        })
    }

    fn meta(&self, buffer_dir: &Path, source: &str) -> &Meta {
        self.meta.get_or_init(|| {
            self.load_meta(buffer_dir, source)
                .unwrap_or_else(|err| panic!("error while loading buffer {:?} - {:?}", source, err))
        })
    }

    fn load_meta(&self, buffer_dir: &Path, source: &str) -> std::io::Result<Meta> {
        let meta_path = self.meta_path(buffer_dir, source);
        if meta_path.exists() {
            return json_from_path(meta_path);
        }

        #[cfg(not(feature = "decode"))]
        {
            euphony_command::api::init_buffer(source.to_string(), meta_path);
            euphony_command::api::flush();

            for i in 0..7 {
                std::thread::sleep(core::time::Duration::from_millis(100 * 2u64.pow(i)));
                if meta_path.exists() {
                    return json_from_path(meta_path);
                }
            }

            panic!("{:?} was not downloaded", source);
        }

        #[cfg(feature = "decode")]
        {
            use std::io::Write;

            let ext = Path::new(source).extension().and_then(|e| e.to_str());

            let contents_path = self.contents_path(buffer_dir, source);

            let contents = std::fs::File::open(contents_path)?;
            let format = decode::reader(contents, ext.unwrap_or(""))
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

            let mut meta = Meta {
                contents: contents_path.to_owned(),
                frames: 0,
                sample_rate: 0,
                channels: 0,
                ext: ext.map(String::from),
            };

            if let Some(track) = format.default_track() {
                meta.update(&track.codec_params);
            }

            let meta_file = std::fs::File::create(meta_path)?;
            let mut meta_file = std::io::BufWriter::new(meta_file);
            serde_json::to_writer(&mut meta_file, &meta)?;
            meta_file.flush()?;

            Ok(meta)
        }
    }
}

pub struct Channel<'a, Buffer>(&'a Buffer, u64);

impl<'a, B> Channel<'a, B> {
    pub fn buffer(&self) -> &B {
        self.0
    }

    pub fn channel(&self) -> u64 {
        self.1
    }
}

pub trait AsChannel {
    fn buffer<F: FnOnce(&Path, &str) -> u64>(&self, init: F) -> u64;
    fn duration(&self) -> Duration;
    fn channel(&self) -> u64;
}

impl<T: AsChannel> AsChannel for &T {
    fn buffer<F: FnOnce(&Path, &str) -> u64>(&self, init: F) -> u64 {
        (*self).buffer(init)
    }

    fn duration(&self) -> Duration {
        (*self).duration()
    }

    fn channel(&self) -> u64 {
        (*self).channel()
    }
}

impl<'a, S: AsRef<str>> AsChannel for Channel<'a, Buffer<S>> {
    fn buffer<F: FnOnce(&Path, &str) -> u64>(&self, init: F) -> u64 {
        self.0.initialize(init)
    }

    fn duration(&self) -> Duration {
        self.0.duration()
    }

    fn channel(&self) -> u64 {
        self.1
    }
}

impl<S: AsRef<str>> AsChannel for Buffer<S> {
    fn buffer<F: FnOnce(&Path, &str) -> u64>(&self, init: F) -> u64 {
        self.initialize(init)
    }

    fn duration(&self) -> Duration {
        self.duration()
    }

    fn channel(&self) -> u64 {
        if self.channels() == 0 {
            panic!("invalid buffer")
        }
        0
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    contents: PathBuf,
    frames: u64,
    sample_rate: u32,
    channels: u32,
    ext: Option<String>,
}

impl Meta {
    #[cfg(feature = "decode")]
    fn update(&mut self, params: &CodecParameters) {
        if let Some(frames) = params.n_frames {
            self.frames = frames;
        }

        if let Some(sample_rate) = params.sample_rate {
            self.sample_rate = sample_rate;
        }

        if let Some(c) = params.channels {
            self.channels = c.count() as _;
        }
    }

    #[inline]
    fn duration(&self) -> Duration {
        if self.sample_rate == 0 || self.frames == 0 {
            return Duration::ZERO;
        }
        Duration::from_secs(self.frames) / self.sample_rate
    }
}

fn json_from_path<T: serde::de::DeserializeOwned>(path: &Path) -> std::io::Result<T> {
    let file = std::fs::File::open(path)?;
    let file = std::io::BufReader::new(file);
    Ok(serde_json::from_reader(file)?)
}

fn modified(path: &impl AsRef<OsStr>) -> SystemTime {
    Path::new(path).metadata().unwrap().modified().unwrap()
}

fn hash_path(root: &Path, hash: &blake3::Hash) -> PathBuf {
    let mut out = [b'A'; 64];
    let len = base64::encode_config_slice(hash.as_bytes(), URL_SAFE_NO_PAD, &mut out);
    let out = unsafe { core::str::from_utf8_unchecked_mut(&mut out) };
    root.join(&out[..len])
}

fn hash_reader(r: &mut impl Read) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0; 4096];
    loop {
        let len = r.read(&mut buf).unwrap();

        if len == 0 {
            return hasher.finalize();
        }

        hasher.update(&buf[..len]);
    }
}
