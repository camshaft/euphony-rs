//! The built-in mix processor. The digest of this file's bytes is the
//! processor's implementation digest, so any change to it is a new
//! processor identity.

/// Adds `input`, scaled by `gain`, into `out`, which holds the interleaved
/// frames from timeline frame `start`; the input's first frame plays at
/// `position`. Each product is rounded before its sum, in frame and channel
/// order, with binary32 arithmetic throughout.
pub(crate) fn mix(
    out: &mut [f32],
    channels: usize,
    start: u64,
    input: &[f32],
    position: u64,
    gain: f32,
) {
    let frames = (out.len() / channels) as u64;
    let input_frames = (input.len() / channels) as u64;
    let first = start.max(position);
    let last = (start + frames).min(position.saturating_add(input_frames));
    for frame in first..last {
        let out_at = (frame - start) as usize * channels;
        let in_at = (frame - position) as usize * channels;
        let out_frame = &mut out[out_at..out_at + channels];
        let in_frame = &input[in_at..in_at + channels];
        for (sample, source) in out_frame.iter_mut().zip(in_frame) {
            let scaled = source * gain;
            *sample += scaled;
        }
    }
}
