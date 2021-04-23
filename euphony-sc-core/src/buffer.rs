use crate::{
    osc,
    osc::control::Value,
    track::{self, Track as _},
    Message,
};
use core::{fmt, ops, time::Duration};
use std::path::Path;

pub trait Contents {
    fn hash(&self) -> &[u8; 32];
    fn as_path(&self, out_dir: &Path) -> &Path;
    fn as_slice(&self) -> &[u8];
}

pub struct BufDef {
    pub channels: u32,
    pub sample_rate: u32,
    pub bit_depth: u16,
    pub frames: u32,
    pub contents: &'static (dyn Contents + Send + Sync),
}

impl BufDef {
    pub fn hash(&self) -> &'static [u8; 32] {
        self.contents.hash()
    }

    pub fn path(&self, out_dir: &Path) -> &Path {
        self.contents.as_path(out_dir)
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs(self.frames as _) / self.sample_rate
    }
}

impl fmt::Debug for BufDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufDef")
            .field("channels", &self.channels)
            .field("sample_rate", &self.sample_rate)
            .field("bit_depth", &self.bit_depth)
            .field("frames", &self.frames)
            .finish()
    }
}

impl Message for &'static BufDef {
    type Output = ();

    fn send(self, track: &track::Handle) -> Self::Output {
        let view: BufView = self.into();
        view.send(track)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BufView {
    start_frame: u32,
    end_frame: u32,
    def: &'static BufDef,
}

impl BufView {
    pub const fn frames(&self) -> ops::Range<u32> {
        self.start_frame..self.end_frame
    }
}

impl ops::Deref for BufView {
    type Target = &'static BufDef;

    fn deref(&self) -> &Self::Target {
        &self.def
    }
}

impl From<&'static BufDef> for BufView {
    fn from(def: &'static BufDef) -> Self {
        Self {
            start_frame: 0,
            end_frame: def.frames,
            def,
        }
    }
}

impl Message for BufView {
    type Output = ();

    fn send(self, track: &track::Handle) -> Self::Output {
        let buffer = track.read(self);
        buffer.send(track)
    }
}

#[derive(Debug)]
pub struct BufDefGroup {
    pub buffers: &'static [&'static BufDef],
}

impl BufDefGroup {
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ops::Deref for BufDefGroup {
    type Target = &'static BufDef;

    fn deref(&self) -> &Self::Target {
        &self.buffers[0]
    }
}

impl ops::Index<usize> for BufDefGroup {
    type Output = &'static BufDef;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffers[index % self.len()]
    }
}

impl Message for &BufDefGroup {
    type Output = ();

    fn send(self, track: &track::Handle) -> Self::Output {
        let view: BufView = self[0].into();
        view.send(track)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    id: osc::buffer::Id,
    view: BufView,
}

impl Buffer {
    pub const fn new(id: osc::buffer::Id, view: BufView) -> Self {
        Self { id, view }
    }

    pub const fn id(&self) -> osc::buffer::Id {
        self.id
    }

    pub const fn view(&self) -> &BufView {
        &self.view
    }

    pub fn to_osc(&self, out_dir: &Path) -> osc::buffer::AllocRead {
        let view = self.view();
        let path = view.path(out_dir);
        let frames = view.frames();
        osc::buffer::AllocRead {
            id: self.id(),
            path: path.to_str().unwrap(),
            offset: frames.start as i32,
            len: (frames.end - frames.start) as i32,
        }
    }
}

impl From<Buffer> for Value {
    fn from(buffer: Buffer) -> Self {
        buffer.id.0.into()
    }
}

impl Message for Buffer {
    type Output = ();

    fn send(self, track: &track::Handle) -> Self::Output {
        let name = match self.view().channels {
            1 => {
                let name = "euphony_play_buf_1";
                track.load(name, include_bytes!("./buffer/euphony_play_buf_1.scsyndef"));
                name
            }
            2 => {
                let name = "euphony_play_buf_2";
                track.load(name, include_bytes!("./buffer/euphony_play_buf_2.scsyndef"));
                name
            }
            channels => unimplemented!("unsupported number of channels: {}", channels),
        };

        let values = [Some((osc::control::Id::Name("buf"), self.id.0.into()))];

        let id = track.play(name, None, None, &values);

        let duration = self.view.duration();
        track.free_after(id, duration);
    }
}
