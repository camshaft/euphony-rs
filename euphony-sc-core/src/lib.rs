pub mod buffer;
pub mod osc;
pub mod param;
pub mod project;
pub mod synthdef;
pub mod track;
pub mod ugen;
pub use ::codec;

#[cfg(feature = "codegen")]
pub mod codegen;

pub trait Message {
    type Output;

    fn send(self, track: &track::Handle) -> Self::Output;
}
