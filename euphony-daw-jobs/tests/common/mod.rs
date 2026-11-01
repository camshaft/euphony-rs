use euphony_daw_identity::Digest;
use euphony_daw_project::{Command, ProjectState, RegionId, TrackId};
use euphony_daw_render::{builtin_registry, compile, mix_processor, render, OutputFormat};
use euphony_daw_render::RenderDescriptor;
use euphony_daw_sample::{ChannelLayout, WorkingAudio};
use euphony_daw_store::{ObjectKind, Store};
use std::collections::BTreeMap;

pub const FORMAT: OutputFormat = OutputFormat {
    sample_rate: 48_000,
    layout: ChannelLayout::Stereo,
};

/// The frames each segment renders.
pub const SEGMENT: u64 = 1024;

/// Publishes four assets and compiles a project of sixteen overlapping
/// regions into its segments.
pub fn fixture(store: &dyn Store) -> Vec<RenderDescriptor> {
    let mut assets = Vec::new();
    for track in 0..4u32 {
        let frames = 16 * SEGMENT as usize;
        let step = 1.0 / (track + 2) as f32;
        let samples = (0..frames * 2).map(|i| step * (i % 97) as f32).collect();
        let audio = WorkingAudio::new(48_000, ChannelLayout::Stereo, samples).unwrap();
        let bytes = audio.to_canonical_bytes();
        assets.push(store.publish(ObjectKind::WorkingAudio, &bytes).unwrap());
    }
    let mut state = ProjectState::default();
    for track in 0..4u64 {
        let command = Command::AddTrack {
            track: TrackId(track),
            name: format!("track {track}"),
        };
        state.apply(&command).unwrap();
    }
    for region in 0..16u64 {
        let command = Command::AddRegion {
            region: RegionId(region),
            track: TrackId(region % 4),
            asset: assets[(region % 4) as usize],
            start: region * 12 * SEGMENT,
        };
        state.apply(&command).unwrap();
    }
    let processor = mix_processor().identity();
    compile(&state, store, FORMAT, processor, SEGMENT).unwrap()
}

/// The digest of each descriptor's result, rendered here as the reference.
pub fn references(
    descriptors: &[RenderDescriptor],
    store: &dyn Store,
) -> BTreeMap<Digest, Digest> {
    let registry = builtin_registry();
    descriptors
        .iter()
        .map(|descriptor| {
            let audio = render(descriptor, &registry, store).unwrap();
            (descriptor.id(), audio.digest())
        })
        .collect()
}
