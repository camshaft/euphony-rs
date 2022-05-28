use crate::sample::{DefaultRate as Rate, Rate as _};
pub use fundsp::*;
pub use hacker::*;

#[inline]
pub fn an<T: AudioNode>(mut an: An<T>) -> T {
    an.reset(Some(Rate::VALUE));
    an.0
}
