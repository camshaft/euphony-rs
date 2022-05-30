macro_rules! b {
    ($hash:literal) => {
        euphony_buffer::Buffer::new(concat!(
            "https://camshaft.github.io/euphony-samples/",
            $hash,
            ".wav"
        ))
    };
}

pub struct Group(&'static [euphony_buffer::Buffer<&'static str>]);

#[cfg(test)]
mod generator;

mod samples;

pub use samples::*;
