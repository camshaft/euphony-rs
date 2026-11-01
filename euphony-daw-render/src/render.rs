use crate::{mix, RenderDescriptor};
use core::fmt;
use euphony_daw_identity::Digest;
use euphony_daw_sample::{
    FormatError, FusedOperations, NumericalProfile, Precision, ProcessorDefinition, Registration,
    Registry, Rounding, Simd, Subnormals, WorkingAudio,
};
use euphony_daw_store::{ObjectKind, Store, StoreError};

const IMPLEMENTATION_DOMAIN: &str = "euphony processor implementation v1";

/// The definition of the built-in mix processor.
pub fn mix_processor() -> ProcessorDefinition {
    ProcessorDefinition {
        name: "mix".to_string(),
        implementation: Digest::of(IMPLEMENTATION_DOMAIN, include_bytes!("mix.rs")),
        parameter_version: 1,
        schema_version: 1,
        profile: NumericalProfile {
            internal_precision: Precision::Binary32,
            rounding: Rounding::NearestEven,
            fused_operations: FusedOperations::Separate,
            simd: Simd::Scalar,
            subnormals: Subnormals::Preserve,
        },
    }
}

/// A registry of the built-in processors.
pub fn builtin_registry() -> Registry {
    let mut registry = Registry::new();
    let registered = registry.register(Registration::of(mix_processor()));
    registered.expect("built-in identities derive as registered");
    registry
}

/// Why a render failed.
#[derive(Debug)]
pub enum RenderError {
    /// The registry doesn't hold the descriptor's processor.
    UnknownProcessor,
    /// The processor's implementation isn't one this build runs.
    UnsupportedProcessor,
    /// An input couldn't be read.
    Input { asset: Digest, error: StoreError },
    /// An input or the output isn't valid working audio.
    Format(FormatError),
    /// An input's format differs from the descriptor's.
    Incompatible { asset: Digest },
    /// The descriptor asks for more samples than this computer addresses.
    TooLarge,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownProcessor => f.write_str("the processor isn't registered"),
            Self::UnsupportedProcessor => f.write_str("the processor isn't supported"),
            Self::Input { asset, error } => write!(f, "input {asset}: {error}"),
            Self::Format(error) => write!(f, "{error}"),
            Self::Incompatible { asset } => write!(f, "input {asset} has another format"),
            Self::TooLarge => f.write_str("the render is larger than memory"),
        }
    }
}

impl std::error::Error for RenderError {}

/// Renders a descriptor. The result is finite working audio in the
/// descriptor's format.
pub fn render(
    descriptor: &RenderDescriptor,
    registry: &Registry,
    store: &dyn Store,
) -> Result<WorkingAudio, RenderError> {
    let definition = registry.get(&descriptor.processor);
    let definition = definition.ok_or(RenderError::UnknownProcessor)?;
    if definition.implementation != mix_processor().implementation {
        return Err(RenderError::UnsupportedProcessor);
    }
    let channels = usize::from(descriptor.layout.channels());
    let frames = usize::try_from(descriptor.frames).map_err(|_| RenderError::TooLarge)?;
    let samples = frames.checked_mul(channels).ok_or(RenderError::TooLarge)?;
    let mut out = vec![0.0f32; samples];
    for input in &descriptor.inputs {
        let asset = input.asset;
        let bytes = store.get(ObjectKind::WorkingAudio, &asset);
        let bytes = bytes.map_err(|error| RenderError::Input { asset, error })?;
        let audio = WorkingAudio::from_canonical_bytes(&bytes).map_err(RenderError::Format)?;
        let header = audio.header();
        if header.sample_rate != descriptor.sample_rate || header.layout != descriptor.layout {
            return Err(RenderError::Incompatible { asset });
        }
        let start = descriptor.start;
        mix::mix(&mut out, channels, start, audio.samples(), input.position, input.gain);
    }
    let audio = WorkingAudio::new(descriptor.sample_rate, descriptor.layout, out);
    let audio = audio.map_err(RenderError::Format)?;
    audio.check_publishable().map_err(RenderError::Format)?;
    Ok(audio)
}
