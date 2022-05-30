use rubato::{
    InterpolationParameters, InterpolationType, Resampler, Sample, SincFixedIn, WindowFunction,
};

pub fn resample<S: Sample>(
    samples: &[Vec<S>],
    current: f64,
    target: f64,
) -> Result<Vec<Vec<S>>, rubato::ResampleError> {
    let params = InterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: InterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::<S>::new(
        target / current,
        2.0,
        params,
        samples[0].len(),
        samples.len(),
    )
    .unwrap();

    resampler.process(samples, None)
}
