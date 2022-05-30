use core::ops;
use euphony_buffer::{AsChannel, Buffer};

macro_rules! g {
    ($name:ident, $($hash:literal,)*) => {
        pub static $name: &crate::Group = &crate::Group({
            use crate::Buffer;
            &[
            $({
                static B: Buffer = Buffer::new(concat!(
                    "https://camshaft.github.io/euphony-samples/",
                    $hash,
                    ".wav"
                ));
                &B
            }),*
            ]
        });
    };
}

pub struct Group(&'static [&'static Buffer]);

impl ops::Deref for Group {
    type Target = [&'static Buffer];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl ops::Index<usize> for Group {
    type Output = Buffer;

    fn index(&self, index: usize) -> &Self::Output {
        let index = index % self.0.len();
        self.0[index]
    }
}

impl AsChannel for Group {
    fn buffer<F: FnOnce(&std::path::Path, &str) -> u64>(&self, init: F) -> u64 {
        AsChannel::buffer(&self[0], init)
    }

    fn channel(&self) -> u64 {
        AsChannel::channel(&self[0])
    }

    fn duration(&self) -> std::time::Duration {
        AsChannel::duration(&self[0])
    }
}

impl IntoIterator for &Group {
    type Item = &'static Buffer;
    type IntoIter = core::iter::Copied<core::slice::Iter<'static, Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

#[cfg(test)]
mod generator;

mod samples;
pub use samples::*;

mod waveforms;
pub use waveforms::*;
