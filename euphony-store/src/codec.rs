use euphony_node::Sink;
use std::io;

#[cfg(feature = "codec-wave")]
pub mod wave;

pub trait Codec<W: io::Write>: Sink {
    const EXTENSION: &'static str;

    fn new(w: W) -> io::Result<Self>;
}
