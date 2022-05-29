use crate::{
    sample::{DefaultRate, DefaultSample, Rate as _},
    CachedBuffer, ConvertedBuffer, Hash, Writer,
};
use euphony_buffer::{
    open_stream,
    symphonia::{
        self,
        core::{
            audio::{AudioBuffer, AudioBufferRef},
            conv::IntoSample,
            sample::Sample,
        },
    },
};
use euphony_node::BufferMap;
use std::{collections::HashMap, fmt, io, ops, sync::Arc};

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
        let paths = cache.buffer(path, DefaultRate::COUNT, |reader| {
            let mut stream = open_stream(reader, ext)?;

            let track = if let Some(track) = stream.default_track() {
                track
            } else {
                // TODO log error
                dbg!("missing tracks");
                return Ok(vec![]);
            };
            let sample_rate = track.codec_params.sample_rate;
            let n_frames = track.codec_params.n_frames;

            let mut decoder =
                symphonia::default::get_codecs().make(&track.codec_params, &Default::default())?;

            let mut channels = vec![];
            loop {
                match stream.next_packet() {
                    Ok(packet) => {
                        let buffer = decoder.decode(&packet)?;
                        match buffer {
                            AudioBufferRef::U8(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::U16(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::U24(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::U32(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::S8(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::S16(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::S24(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::S32(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::F32(buf) => copy_buf(&buf, &mut channels),
                            AudioBufferRef::F64(buf) => copy_buf(&buf, &mut channels),
                        }
                    }
                    Err(symphonia::core::errors::Error::ResetRequired) => break,
                    Err(symphonia::core::errors::Error::IoError(err)) => {
                        if err.kind() != io::ErrorKind::UnexpectedEof {
                            return Err(err.into());
                        }

                        // make sure we got all of the frames we needed
                        if let Some(n_frames) = n_frames {
                            if channels.get(0).map(|c| c.len() as u64).unwrap_or(0) != n_frames {
                                return Err(err.into());
                            }
                        }

                        break;
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            if let Some(sample_rate) = sample_rate {
                if sample_rate as u64 != DefaultRate::COUNT {
                    use rubato::{
                        InterpolationParameters, InterpolationType, Resampler, SincFixedIn,
                        WindowFunction,
                    };

                    let params = InterpolationParameters {
                        sinc_len: 256,
                        f_cutoff: 0.95,
                        interpolation: InterpolationType::Linear,
                        oversampling_factor: 256,
                        window: WindowFunction::BlackmanHarris2,
                    };
                    let mut resampler = SincFixedIn::<DefaultSample>::new(
                        DefaultRate::VALUE as f64 / sample_rate as f64,
                        2.0,
                        params,
                        channels[0].len(),
                        channels.len(),
                    )
                    .unwrap();

                    channels = resampler.process(&channels, None).unwrap();
                }
            }

            Ok(channels)
        })?;

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

#[inline]
fn copy_buf<S: Sample + IntoSample<DefaultSample>>(
    src: &AudioBuffer<S>,
    out: &mut Vec<ConvertedBuffer>,
) {
    use symphonia::core::audio::Signal;

    let spec = src.spec();
    let n_channels = spec.channels.count();

    if out.is_empty() {
        for _ in 0..n_channels {
            out.push(ConvertedBuffer::default());
        }
    }

    for (ch, chan) in out.iter_mut().enumerate() {
        for src in src.chan(ch) {
            let sample = (*src).into_sample();
            chan.push(sample);
        }
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffer").field("len", &self.len()).finish()
    }
}

impl ops::Deref for Buffer {
    type Target = [f64];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.samples.deref()
    }
}
