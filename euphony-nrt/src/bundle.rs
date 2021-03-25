use bytes::BytesMut;
use codec::encode::{EncoderBuffer, TypeEncoder};
use core::mem::size_of;
use euphony_osc::{bundle::Bundle as Inner, types::Timetag};

#[derive(Clone, Debug, Default)]
pub struct Bundle(Inner);

impl Bundle {
    pub fn new(timetag: Timetag) -> Self {
        Self(Inner::new(timetag))
    }

    pub fn write<T>(&mut self, value: T)
    where
        T: for<'a> TypeEncoder<&'a mut BytesMut>,
    {
        if self.is_empty() {
            // reserve the len bytes
            self.0.write_header(0i32);
        }

        self.0.write(value)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn finish(self) -> Option<BytesMut> {
        let mut v = self.0.finish()?;

        // update the len prefix
        let len = (v.len() - size_of::<i32>()) as i32;
        v[..4].encode(len).unwrap();

        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;
    use euphony_sc::osc;
    use euphony_testing::*;

    const EXPECTED: &[u8] = include_bytes!("../artifacts/simple-sin.osc");
    const NRTSINE: &[u8] = include_bytes!("../artifacts/nrtsine.synthdef");

    #[test]
    fn sin_wav() {
        let mut buf = vec![];
        let mut bundle = Bundle::default();

        bundle.write(osc::synthdef::Receive { buffer: NRTSINE });

        bundle.write(osc::synth::New {
            name: "NRTsine",
            id: osc::node::Id(1000),
            action: Default::default(),
            busses: &[],
            values: &[],
            target: osc::node::Id(0),
        });

        buf.extend(bundle.finish().unwrap());

        let mut bundle = Bundle::new(Duration::from_secs(3).into());
        bundle.write(osc::node::Free {
            id: osc::node::Id(1000),
        });

        buf.extend(bundle.finish().unwrap());
        assert_hex_eq!(EXPECTED, buf);
    }
}
