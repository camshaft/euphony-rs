use euphony_daw_identity::{AuthorKey, Digest};
use euphony_daw_project::{Command, ProjectState, RegionId, Revision, RevisionId, TrackId};
use euphony_daw_render::{
    builtin_registry, compile, mix_processor, render, OutputFormat, RenderDescriptor,
    RenderError, RenderManifest, Segment,
};
use euphony_daw_sample::{
    ChannelLayout, NumericalProfile, ProcessorDefinition, Registration, Registry, Rounding, Simd,
    Subnormals, WorkingAudio,
};
use euphony_daw_store::{MemStore, ObjectKind, Store};
use std::collections::HashMap;

const FORMAT: OutputFormat = OutputFormat {
    sample_rate: 48_000,
    layout: ChannelLayout::Stereo,
};

fn asset(store: &MemStore, frames: usize, value: f32) -> Digest {
    let samples = (0..frames * 2).map(|i| value * (i % 7) as f32).collect();
    let audio = WorkingAudio::new(48_000, ChannelLayout::Stereo, samples).unwrap();
    let bytes = audio.to_canonical_bytes();
    store.publish(ObjectKind::WorkingAudio, &bytes).unwrap()
}

fn project(store: &MemStore) -> ProjectState {
    let mut state = ProjectState::default();
    let commands = [
        Command::AddTrack {
            track: TrackId(1),
            name: "drums".to_string(),
        },
        Command::AddTrack {
            track: TrackId(2),
            name: "bass".to_string(),
        },
        Command::SetTrackGain {
            track: TrackId(2),
            gain: 0.5,
        },
        Command::AddRegion {
            region: RegionId(1),
            track: TrackId(1),
            asset: asset(store, 300, 0.25),
            start: 0,
        },
        Command::AddRegion {
            region: RegionId(2),
            track: TrackId(2),
            asset: asset(store, 200, 0.125),
            start: 150,
        },
    ];
    for command in &commands {
        state.apply(command).unwrap();
    }
    state
}

fn mix_id() -> euphony_daw_sample::ProcessorId {
    mix_processor().identity()
}

#[test]
fn renders_the_mix_of_overlapping_regions() {
    let store = MemStore::new();
    let state = project(&store);
    let descriptors = compile(&state, &store, FORMAT, mix_id(), 128).unwrap();
    let starts: Vec<_> = descriptors.iter().map(|d| (d.start, d.frames)).collect();
    assert_eq!(starts, vec![(0, 128), (128, 128), (256, 94)]);

    let registry = builtin_registry();
    let mut rendered = Vec::new();
    for descriptor in &descriptors {
        let decoded = RenderDescriptor::decode(&descriptor.canonical_bytes()).unwrap();
        assert_eq!(&decoded, descriptor);
        let audio = render(descriptor, &registry, &store).unwrap();
        assert_eq!(audio.frames(), descriptor.frames);
        rendered.extend_from_slice(audio.samples());
    }

    let drums: Vec<f32> = (0..600).map(|i| 0.25 * (i % 7) as f32).collect();
    let bass: Vec<f32> = (0..400).map(|i| 0.125 * (i % 7) as f32).collect();
    for frame in 0..350usize {
        for channel in 0..2 {
            let at = frame * 2 + channel;
            let mut expected = 0.0f32;
            if frame < 300 {
                expected += drums[at] * 1.0;
            }
            if (150..350).contains(&frame) {
                expected += bass[(frame - 150) * 2 + channel] * 0.5;
            }
            assert_eq!(rendered[at].to_bits(), expected.to_bits(), "frame {frame}");
        }
    }
}

#[test]
fn an_edit_elsewhere_keeps_untouched_segment_identities() {
    let store = MemStore::new();
    let mut state = project(&store);
    let before = compile(&state, &store, FORMAT, mix_id(), 128).unwrap();
    let late = asset(&store, 10, 1.0);
    let edit = Command::AddRegion {
        region: RegionId(3),
        track: TrackId(1),
        asset: late,
        start: 300,
    };
    state.apply(&edit).unwrap();
    let after = compile(&state, &store, FORMAT, mix_id(), 128).unwrap();
    assert_eq!(before[0].id(), after[0].id());
    assert_eq!(before[1].id(), after[1].id());
    assert_ne!(before[2].id(), after[2].id());
}

fn variants() -> Vec<ProcessorDefinition> {
    let base = mix_processor();
    let profile = base.profile;
    let profiles = [
        NumericalProfile {
            internal_precision: euphony_daw_sample::Precision::Binary64,
            ..profile
        },
        NumericalProfile {
            rounding: Rounding::TowardZero,
            ..profile
        },
        NumericalProfile {
            fused_operations: euphony_daw_sample::FusedOperations::Fused,
            ..profile
        },
        NumericalProfile {
            simd: Simd::Lanes(4),
            ..profile
        },
        NumericalProfile {
            subnormals: Subnormals::FlushToZero,
            ..profile
        },
    ];
    let mut out: Vec<_> = profiles
        .into_iter()
        .map(|profile| ProcessorDefinition {
            profile,
            ..base.clone()
        })
        .collect();
    out.push(ProcessorDefinition {
        implementation: Digest::of("euphony test", b"another mix"),
        ..base
    });
    out
}

#[test]
fn a_changed_profile_or_implementation_prevents_old_cache_reuse() {
    let store = MemStore::new();
    let state = project(&store);
    let registry = builtin_registry();

    let mut cache = HashMap::new();
    for descriptor in compile(&state, &store, FORMAT, mix_id(), 128).unwrap() {
        let audio = render(&descriptor, &registry, &store).unwrap();
        cache.insert(descriptor.id(), audio.digest());
    }

    for variant in variants() {
        let processor = variant.identity();
        assert_ne!(processor, mix_id());
        let descriptors = compile(&state, &store, FORMAT, processor, 128).unwrap();
        for descriptor in &descriptors {
            assert!(
                !cache.contains_key(&descriptor.id()),
                "{variant:?} reused a cached result"
            );
        }

        let mut registry = Registry::new();
        registry
            .register(Registration::of(variant.clone()))
            .unwrap();
        let result = render(&descriptors[0], &registry, &store);
        if variant.implementation != mix_processor().implementation {
            assert!(matches!(result, Err(RenderError::UnsupportedProcessor)));
        }
        let result = render(&descriptors[0], &builtin_registry(), &store);
        assert!(matches!(result, Err(RenderError::UnknownProcessor)));
    }
}

#[test]
fn manifests_round_trip() {
    let key = AuthorKey::from_seed([1; 32]);
    let revision = Revision {
        parents: Vec::new(),
        root: Digest::of("euphony test", b"root"),
        schema_version: Revision::SCHEMA_VERSION,
        author: key.author(),
        message: String::new(),
    };
    let manifest = RenderManifest {
        revision: RevisionId(revision.id().0),
        sample_rate: 48_000,
        layout: ChannelLayout::Stereo,
        segments: vec![Segment {
            descriptor: Digest::of("euphony test", b"descriptor"),
            start: 0,
            frames: 128,
            result: Digest::of("euphony test", b"result"),
        }],
    };
    let decoded = RenderManifest::decode(&manifest.canonical_bytes()).unwrap();
    assert_eq!(decoded, manifest);
}

#[test]
fn non_finite_results_are_refused() {
    let store = MemStore::new();
    let loud = WorkingAudio::new(48_000, ChannelLayout::Stereo, vec![f32::MAX; 2]).unwrap();
    let loud = store
        .publish(ObjectKind::WorkingAudio, &loud.to_canonical_bytes())
        .unwrap();
    let descriptor = RenderDescriptor {
        sample_rate: 48_000,
        layout: ChannelLayout::Stereo,
        start: 0,
        frames: 1,
        processor: mix_id(),
        inputs: vec![
            euphony_daw_render::Input {
                asset: loud,
                position: 0,
                gain: 1.0,
            },
            euphony_daw_render::Input {
                asset: loud,
                position: 0,
                gain: 1.0,
            },
        ],
    };
    let result = render(&descriptor, &builtin_registry(), &store);
    assert!(matches!(result, Err(RenderError::Format(_))));
}
