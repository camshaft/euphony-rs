//! Render descriptors, compilation and the built-in renderer.
//!
//! A render descriptor identifies a requested calculation; the digest of the
//! result's bytes identifies its output. Local and remote workers render the
//! same descriptors through the same function.

mod compile;
mod descriptor;
mod manifest;
mod mix;
mod render;

pub use compile::{compile, CompileError, OutputFormat};
pub use descriptor::{Input, RenderDescriptor};
pub use manifest::{RenderManifest, Segment};
pub use render::{builtin_registry, mix_processor, render, RenderError};
