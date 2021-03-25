use crate::types::Timetag;
use bytes::BytesMut;
use codec::encode::{EncoderBuffer, TypeEncoder};
use core::mem::size_of;

pub const TAG: [u8; 8] = *b"#bundle\0";

#[derive(Clone, Debug, Default)]
pub struct Bundle {
    timetag: Timetag,
    content: BytesMut,
}

impl Bundle {
    pub fn new(timetag: Timetag) -> Self {
        Self {
            timetag,
            content: Default::default(),
        }
    }

    pub fn write<T>(&mut self, value: T)
    where
        T: for<'a> TypeEncoder<&'a mut BytesMut>,
    {
        if self.is_empty() {
            // reserve the header bytes
            self.write_header(());
        }

        let buffer = &mut self.content;
        let prev_len = buffer.len();

        // reserve the length prefix
        buffer.encode(0i32).unwrap();
        buffer.encode(value).unwrap();

        let len = (buffer.len() - prev_len - size_of::<i32>()) as i32;
        (&mut buffer[prev_len..]).encode(len).unwrap();
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn finish(self) -> Option<BytesMut> {
        if self.content.is_empty() {
            return None;
        }

        Some(self.content)
    }

    pub fn write_header<T>(&mut self, prefix: T)
    where
        T: for<'a> TypeEncoder<&'a mut BytesMut>,
    {
        let content = &mut self.content;

        debug_assert!(content.is_empty());

        content.encode(prefix).unwrap();
        content.encode(&TAG[..]).unwrap();
        content.encode(self.timetag).unwrap();
    }
}
