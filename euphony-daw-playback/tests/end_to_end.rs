//! A project committed through its branch authority renders through the
//! scheduler, its manifest is pinned, and after everything reopens from disk
//! the same manifest plays bit for bit.

use euphony_daw_identity::{AuthorKey, Digest};
use euphony_daw_jobs::{perform, Completion, FromWorker, Scheduler, WorkerId};
use euphony_daw_playback::{
    prepare, Device, DeviceError, DeviceFormat, PlaybackError, SimulatedDevice,
};
use euphony_daw_project::{
    BranchAuthority, Command, CommandBatch, IdempotencyKey, ProjectId, RegionId, TrackId,
};
use euphony_daw_render::{
    builtin_registry, compile, mix_processor, render, OutputFormat, RenderManifest, Segment,
};
use euphony_daw_sample::{ChannelLayout, WorkingAudio};
use euphony_daw_store::{DirStore, ObjectKind, Store};
use std::path::Path;

const FORMAT: OutputFormat = OutputFormat {
    sample_rate: 48_000,
    layout: ChannelLayout::Stereo,
};

fn key() -> AuthorKey {
    AuthorKey::from_seed([7; 32])
}

fn project() -> ProjectId {
    ProjectId(Digest::of("euphony test", b"bounced project"))
}

fn open(path: &Path) -> BranchAuthority {
    let authority = BranchAuthority::open(path, key(), project(), "main");
    authority.unwrap()
}

fn asset(store: &DirStore, frames: usize, step: f32) -> Digest {
    let samples = (0..frames * 2).map(|i| step * (i % 31) as f32).collect();
    let audio = WorkingAudio::new(48_000, ChannelLayout::Stereo, samples);
    let bytes = audio.unwrap().to_canonical_bytes();
    store.publish(ObjectKind::WorkingAudio, &bytes).unwrap()
}

fn commands(drums: Digest, bass: Digest) -> Vec<Command> {
    vec![
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
            gain: 0.75,
        },
        Command::AddRegion {
            region: RegionId(1),
            track: TrackId(1),
            asset: drums,
            start: 0,
        },
        Command::AddRegion {
            region: RegionId(2),
            track: TrackId(2),
            asset: bass,
            start: 700,
        },
    ]
}

/// Renders every descriptor with one local worker through the scheduler.
fn render_all(scheduler: &mut Scheduler, store: &DirStore) {
    let registry = builtin_registry();
    let worker = WorkerId(0);
    scheduler.join(worker);
    while let Some(assignment) = scheduler.assign(worker) {
        for message in perform(&assignment, &registry, store) {
            match message {
                FromWorker::Chunk { attempt, bytes } => scheduler.chunk(worker, attempt, &bytes),
                FromWorker::Finished {
                    attempt,
                    epoch,
                    descriptor,
                    result,
                } => {
                    let done = scheduler.finish(worker, attempt, epoch, descriptor, result, store);
                    assert!(matches!(done.unwrap(), Completion::Accepted(_)));
                }
                other => panic!("the render failed: {other:?}"),
            }
        }
    }
    assert!(scheduler.is_complete());
}

#[test]
fn reopens_to_the_same_manifest_and_plays_it_bit_for_bit() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("store");
    let branch = dir.path().join("branch");
    let jobs = dir.path().join("jobs");

    let (head, manifest_digest, reference) = {
        let store = DirStore::open(&root).unwrap();
        let drums = asset(&store, 1500, 0.03125);
        let bass = asset(&store, 1000, 0.0625);
        let mut authority = open(&branch);
        let batch = CommandBatch {
            project: project(),
            expected: None,
            key: IdempotencyKey([1; 16]),
            actor: key().author(),
            message: "first bounce".to_string(),
            commands: commands(drums, bass),
        };
        let head = authority.submit(&batch).unwrap().revision;

        let processor = mix_processor().identity();
        let descriptors = compile(authority.state(), &store, FORMAT, processor, 512);
        let descriptors = descriptors.unwrap();
        let mut scheduler = Scheduler::open(&jobs).unwrap();
        for descriptor in &descriptors {
            scheduler.submit(descriptor.clone()).unwrap();
        }
        render_all(&mut scheduler, &store);

        let registry = builtin_registry();
        let mut reference = Vec::new();
        let mut segments = Vec::new();
        for descriptor in &descriptors {
            let audio = render(descriptor, &registry, &store).unwrap();
            reference.extend_from_slice(audio.samples());
            let result = scheduler.result(&descriptor.id()).unwrap();
            assert_eq!(result, audio.digest());
            segments.push(Segment {
                descriptor: descriptor.id(),
                start: descriptor.start,
                frames: descriptor.frames,
                result,
            });
        }
        let manifest = RenderManifest {
            revision: head,
            sample_rate: FORMAT.sample_rate,
            layout: FORMAT.layout,
            segments,
        };
        let bytes = manifest.canonical_bytes();
        let digest = store.publish(ObjectKind::Manifest, &bytes).unwrap();
        authority.pin(head, digest).unwrap();
        (head, digest, reference)
    };

    let store = DirStore::open(&root).unwrap();
    let authority = open(&branch);
    assert_eq!(authority.head(), Some(head));
    assert_eq!(authority.pinned(&head), Some(manifest_digest));
    let scheduler = Scheduler::open(&jobs).unwrap();
    assert!(scheduler.is_complete());

    let bytes = store.get(ObjectKind::Manifest, &manifest_digest).unwrap();
    let manifest = RenderManifest::decode(&bytes).unwrap();
    assert_eq!(manifest.revision, head);
    let prepared = prepare(&manifest, &store).unwrap();

    let format = DeviceFormat {
        sample_rate: 48_000,
        channels: 2,
    };
    let mut device = SimulatedDevice::new(format, 256);
    device.play(&prepared).unwrap();
    assert_eq!(device.played().len(), reference.len());
    for (played, expected) in device.played().iter().zip(&reference) {
        assert_eq!(played.to_bits(), expected.to_bits());
    }

    let other = DeviceFormat {
        sample_rate: 44_100,
        channels: 2,
    };
    let mut device = SimulatedDevice::new(other, 256);
    let refused = device.play(&prepared);
    assert!(matches!(refused, Err(DeviceError::Unsupported { .. })));

    let mut missing = manifest.clone();
    missing.segments[0].result = Digest::of("euphony test", b"never rendered");
    let refused = prepare(&missing, &store);
    assert!(matches!(refused, Err(PlaybackError::Missing { .. })));
}
