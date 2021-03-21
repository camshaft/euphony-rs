use alloc::sync::Arc;
use bytes::BytesMut;
use core::{mem::size_of, time::Duration};
use lazy_static::lazy_static;
use std::{
    io::{Result, Write},
    sync::Mutex,
};

lazy_static! {
    static ref CURRENT_BUNDLE: Arc<Mutex<Bundle>> =
        Arc::new(Mutex::new(Bundle::new(Duration::from_secs(0))));
}

#[derive(Clone, Debug)]
pub struct Bundle {
    time: Duration,
    content: BytesMut,
}

impl Bundle {
    pub fn new(time: Duration) -> Self {
        Self {
            time,
            content: BytesMut::new(),
        }
    }
}

pub trait Sendable {
    fn send(self, content: &mut BytesMut);
}

pub fn send<T: Sendable>(value: T) {
    value.send(&mut CURRENT_BUNDLE.lock().unwrap().content);
}

pub fn flush<W: Write>(now: Duration, out: &mut W) -> Result<usize> {
    let bundle = &mut *CURRENT_BUNDLE.lock().unwrap();
    if bundle.content.is_empty() {
        return Ok(0);
    }

    const TAG: &[u8] = b"#bundle\0";

    let len = (TAG.len() + size_of::<u32>() + size_of::<u32>() + bundle.content.len()) as u32;

    let mut o = 0;

    o += out.write(&len.to_be_bytes())?;
    o += out.write(&TAG)?;
    o += write_duration(bundle.time, out)?;
    let content = &mut bundle.content;
    let len = content.len();
    let len = (len as i32).to_be_bytes();
    o += out.write(&len)?;
    o += out.write(content)?;

    bundle.time = now;
    bundle.content.clear();

    Ok(o)
}

fn write_duration<W: Write>(duration: Duration, out: &mut W) -> Result<usize> {
    let secs = duration.as_secs() as u32;
    let nanos = duration.subsec_nanos();
    let frac = core::u32::MAX / 1_000_000_000 * nanos;

    out.write(&secs.to_be_bytes())?;
    out.write(&frac.to_be_bytes())?;

    Ok(size_of::<u32>() + size_of::<u32>())
}

#[cfg(test)]
mod tests {
    use super::CURRENT_BUNDLE;
    use bytes::BytesMut;
    use core::time::Duration;
    use euphony_sc::{
        codec::encode::{EncoderBuffer, TypeEncoder},
        osc,
    };
    use std::{fs::File, io::Read};

    fn send<T>(value: T)
    where
        T: for<'a> TypeEncoder<&'a mut BytesMut>,
    {
        let buffer = &mut CURRENT_BUNDLE.lock().unwrap().content;

        buffer.encode(value).unwrap();
    }

    const EXPECTED: &[u8] = hex!(
        "
        0000 2c00 6223 6e75 6c64 0065 0000 0000
        0000 0000 0000 1800 732f 6e5f 7765 0000
        732c 0069 524e 7354 6e69 0065 0000 7b00
        0000 2400 6223 6e75 6c64 0065 0000 0100
        0000 0000 0000 1000 6e2f 665f 6572 0065
        692c 0000 0000 7b00
        "
    );

    #[test]
    fn sin_wav() {
        let mut f = File::create("actual.osc").unwrap();
        send(osc::synth::New {
            // TODO
            name: "NRTsine",
            id: osc::node::Id(123),
            action: Default::default(),
            busses: &[],
            values: &[],
            target: osc::node::Id(0),
        });
        super::flush(Duration::from_secs(1), &mut f).unwrap();
        send(osc::node::Free {
            id: osc::node::Id(123),
        });
        super::flush(Duration::from_secs(2), &mut f).unwrap();
        dbg!(rosc::decoder::decode(EXPECTED));
        panic!()
    }
}
