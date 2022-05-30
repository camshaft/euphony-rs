use crate::{
    sample::{DefaultRate, DefaultSample, Rate as _},
    CachedBuffer, Hash, Writer,
};
use euphony_buffer::{decode, symphonia};
use euphony_node::BufferMap;
use std::{collections::HashMap, fmt, ops, sync::Arc};

#[derive(Debug)]
pub struct Map {
    buffers: HashMap<(u64, u64), Buffer>,
}

impl Map {
    pub fn new(buffers: HashMap<(u64, u64), Buffer>) -> Self {
        Self { buffers }
    }
}

impl BufferMap for Map {
    fn get(&self, id: u64, channel: u64) -> euphony_node::Buffer {
        let buffer = self
            .buffers
            .get(&(id, channel))
            .unwrap_or_else(|| panic!("missing buffer {} channel {}", id, channel));
        euphony_node::Buffer {
            samples: &*buffer,
            hash: &buffer.hash,
        }
    }
}

pub struct Buffer {
    samples: Arc<[f64]>,
    hash: Hash,
}

impl Buffer {
    fn open(cached: CachedBuffer) -> std::io::Result<Self> {
        Ok(Self {
            samples: cached.samples,
            hash: cached.hash,
        })
    }

    pub(crate) fn load<W: Writer>(
        id: u64,
        path: &str,
        ext: &str,
        cache: &W,
    ) -> symphonia::core::errors::Result<Vec<((u64, u64), Self)>> {
        let paths = cache.buffer::<_, symphonia::core::errors::Error>(
            path,
            DefaultRate::COUNT,
            |reader| {
                let mut stream = decode::reader(reader, ext)?;
                let mut samples = decode::Samples::from_reader(&mut *stream)?;

                samples
                    .resample(DefaultRate::COUNT as _)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                Ok(samples.channels)
            },
        )?;

        Ok(paths
            .into_iter()
            .enumerate()
            .map(|(channel, cached)| {
                let buf = Buffer::open(cached).unwrap();
                ((id, channel as u64), buf)
            })
            .collect())
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffer").field("len", &self.len()).finish()
    }
}

impl ops::Deref for Buffer {
    type Target = [DefaultSample];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.samples.deref()
    }
}
