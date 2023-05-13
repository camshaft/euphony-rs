use crate::prelude::*;
pub use fundsp::*;
pub use hacker::*;

#[inline]
pub fn an<T: AudioNode>(mut an: An<T>) -> T {
    an.set_sample_rate(Rate::VALUE);
    an.0
}
