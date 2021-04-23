use core::ops::Range;
use dashmap::DashMap;
use euphony_sc::{
    buffer::{BufView, Buffer},
    osc::buffer::Id,
};
use hash_hasher::HashBuildHasher;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Default)]
pub struct Buffers {
    buffer_id: AtomicI32,
    buffers: DashMap<([u8; 32], Range<u32>), Buffer, HashBuildHasher>,
}

impl Buffers {
    pub fn read(&self, buffer: BufView) -> Buffer {
        let key = (*buffer.hash(), buffer.frames());
        *self.buffers.entry(key).or_insert_with(|| {
            let id = self.buffer_id.fetch_add(1, Ordering::SeqCst);
            Buffer::new(Id(id), buffer)
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = Buffer> + '_ {
        self.buffers.iter().map(|v| *v.value())
    }
}
