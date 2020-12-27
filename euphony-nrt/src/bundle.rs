use alloc::sync::Arc;
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
    content: Vec<u8>,
}

impl Bundle {
    pub fn new(time: Duration) -> Self {
        Self {
            time,
            content: vec![],
        }
    }
}

pub trait Sendable {
    fn send(self, content: &mut Vec<u8>);
}

pub fn send<T: Sendable>(value: T) {
    value.send(&mut CURRENT_BUNDLE.lock().unwrap().content);
}

pub fn flush<W: Write>(now: Duration, out: &mut W) -> Result<usize> {
    let bundle = &mut *CURRENT_BUNDLE.lock().unwrap();
    if bundle.content.is_empty() {
        return Ok(0);
    }

    const TAG: &str = "#bundle";

    let len = (TAG.len() + size_of::<u32>() + size_of::<u32>() + bundle.content.len()) as u32;

    let mut o = 0;

    o += out.write(&len.to_be_bytes())?;
    o += write_duration(bundle.time, out)?;
    o += out.write(&bundle.content)?;

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
