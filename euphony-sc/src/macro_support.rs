use euphony_sc_core::buffer;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

pub use euphony_sc_core::{
    buffer::Buffer,
    osc::control::Value as OscValue,
    param::Param,
    synthdef::builder::{
        external_synthdef, param, param_instance, synthdef, Parameters, Synth, SynthDef, SynthDesc,
        SynthDescRef, Value,
    },
    ugen,
};
pub type SynthCell = OnceCell<SynthDesc>;

pub struct InlineFile {
    name: &'static str,
    hash: &'static [u8; 32],
    contents: &'static [u8],
    cell: OnceCell<PathBuf>,
}

impl InlineFile {
    pub const fn new(name: &'static str, hash: &'static [u8; 32], contents: &'static [u8]) -> Self {
        Self {
            name,
            hash,
            contents,
            cell: OnceCell::new(),
        }
    }
}

impl buffer::Contents for InlineFile {
    fn hash(&self) -> &[u8; 32] {
        self.hash
    }

    fn as_path(&self, out_dir: &Path) -> &Path {
        self.cell.get_or_init(|| {
            let path = out_dir.join(self.name);

            if !path.exists() {
                std::fs::write(&path, &self.contents).unwrap();
            }

            path
        })
    }

    fn as_slice(&self) -> &[u8] {
        self.contents
    }
}

pub struct ExternalFile {
    hash: &'static [u8; 32],
    path: &'static str,
    cell: OnceCell<Vec<u8>>,
}

impl ExternalFile {
    pub const fn new(hash: &'static [u8; 32], path: &'static str) -> Self {
        Self {
            hash,
            path,
            cell: OnceCell::new(),
        }
    }
}

impl buffer::Contents for ExternalFile {
    fn hash(&self) -> &[u8; 32] {
        self.hash
    }

    fn as_path(&self, _out_dir: &Path) -> &Path {
        Path::new(self.path)
    }

    fn as_slice(&self) -> &[u8] {
        self.cell.get_or_init(|| std::fs::read(self.path).unwrap())
    }
}
