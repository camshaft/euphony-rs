use std::io;
use symphonia::core::{
    audio::{AudioBuffer, AudioBufferRef},
    conv::{FromSample, IntoSample},
    probe::Hint,
    sample::{i24, u24, Sample},
};

#[derive(Clone, Debug)]
pub struct Samples<S> {
    pub channels: Vec<Vec<S>>,
    pub sample_rate: Option<u32>,
}

impl<S> Samples<S>
where
    S: Sample
        + FromSample<u8>
        + FromSample<u16>
        + FromSample<u24>
        + FromSample<u32>
        + FromSample<i8>
        + FromSample<i16>
        + FromSample<i24>
        + FromSample<i32>
        + FromSample<f32>
        + FromSample<f64>,
{
    pub fn from_reader(
        stream: &mut dyn symphonia::core::formats::FormatReader,
    ) -> Result<Samples<S>, symphonia::core::errors::Error> {
        let track = if let Some(track) = stream.default_track() {
            track
        } else {
            return Ok(Samples {
                channels: vec![],
                sample_rate: None,
            });
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

        Ok(Samples {
            channels,
            sample_rate,
        })
    }
}

#[cfg(any(test, feature = "resample"))]
impl<S> Samples<S>
where
    S: rubato::Sample,
{
    pub fn resample(&mut self, target: u32) -> Result<(), rubato::ResampleError> {
        if self.channels.is_empty() {
            return Ok(());
        }

        let sample_rate = if let Some(sample_rate) = self.sample_rate {
            if sample_rate == target {
                return Ok(());
            }
            sample_rate
        } else {
            return Ok(());
        };

        use rubato::{
            InterpolationParameters, InterpolationType, Resampler, SincFixedIn, WindowFunction,
        };

        let params = InterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: InterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = SincFixedIn::<S>::new(
            target as f64 / sample_rate as f64,
            2.0,
            params,
            self.channels[0].len(),
            self.channels.len(),
        )
        .unwrap();

        self.channels = resampler.process(&self.channels, None)?;

        Ok(())
    }
}

pub fn reader(
    contents: impl io::Read + Send + Sync + 'static,
    ext: &str,
) -> symphonia::core::errors::Result<Box<dyn symphonia::core::formats::FormatReader>> {
    let contents = std::io::BufReader::new(contents);
    let contents = symphonia::core::io::ReadOnlySource::new(contents);

    let contents =
        symphonia::core::io::MediaSourceStream::new(Box::new(contents), Default::default());

    let mut hint = Hint::new();
    if !ext.is_empty() {
        hint.with_extension(ext);
    }

    let res = symphonia::default::get_probe().format(
        &hint,
        contents,
        &Default::default(),
        &Default::default(),
    )?;

    Ok(res.format)
}

#[inline]
fn copy_buf<S: Sample + IntoSample<T>, T>(src: &AudioBuffer<S>, out: &mut Vec<Vec<T>>) {
    use symphonia::core::audio::Signal;

    let spec = src.spec();
    let n_channels = spec.channels.count();

    if out.is_empty() {
        for _ in 0..n_channels {
            out.push(vec![]);
        }
    }

    for (ch, chan) in out.iter_mut().enumerate() {
        for src in src.chan(ch) {
            let sample = (*src).into_sample();
            chan.push(sample);
        }
    }
}
