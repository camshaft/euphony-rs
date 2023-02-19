use euphony_compiler::{
    midi,
    sample::{DefaultRate, Rate as _},
    Hash, Writer,
};
use serde::{Deserialize, Serialize};
use std::{io, ops};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timeline {
    pub sample_rate: u32,
    pub groups: Vec<Group>,
}

impl Default for Timeline {
    #[inline]
    fn default() -> Self {
        Self {
            sample_rate: DefaultRate::COUNT as _,
            groups: Default::default(),
        }
    }
}

impl Timeline {
    #[inline]
    pub fn to_json<W: io::Write>(&self, w: W) -> io::Result<()> {
        serde_json::to_writer(w, self)?;
        Ok(())
    }

    #[inline]
    pub fn reset(&mut self) {
        self.sample_rate = DefaultRate::COUNT as _;
        self.groups.clear();
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub entries: HashDisplay,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub midi: Option<HashDisplay>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HashDisplay(#[serde(with = "base64")] Hash);

impl ops::Deref for HashDisplay {
    type Target = Hash;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod base64 {
    use ::base64::prelude::*;
    use euphony_compiler::Hash;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut out = [b'A'; 64];
            let len = BASE64_URL_SAFE_NO_PAD
                .encode_slice(bytes, &mut out)
                .unwrap();
            let out = unsafe { core::str::from_utf8_unchecked_mut(&mut out) };
            let out = &out[..len];
            serializer.serialize_str(out)
        } else {
            serializer.serialize_bytes(bytes)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <&str>::deserialize(deserializer)?;
            let mut out = Hash::default();
            let len = BASE64_URL_SAFE_NO_PAD
                .decode_slice(s, &mut out)
                .map_err(serde::de::Error::custom)?;

            if len != out.len() {
                return Err(serde::de::Error::custom("invalid hash length"));
            }

            Ok(out)
        } else {
            Hash::deserialize(deserializer)
        }
    }
}

impl Writer for Timeline {
    #[inline]
    fn is_cached(&self, _: &Hash) -> bool {
        unimplemented!()
    }

    #[inline]
    fn sink(&mut self, _hash: &Hash) -> euphony_node::BoxProcessor {
        unimplemented!()
    }

    #[inline]
    fn group<I: Iterator<Item = euphony_compiler::Entry>>(
        &mut self,
        name: &str,
        hash: &Hash,
        _entries: I,
        midi: &midi::Writer,
    ) {
        self.groups.push(Group {
            name: name.to_string(),
            entries: HashDisplay(*hash),
            midi: if midi.is_empty() {
                None
            } else {
                Some(HashDisplay(*midi.hash()))
            },
        });
    }

    fn buffer<
        F: FnOnce(
            Box<dyn euphony_compiler::BufferReader>,
        ) -> euphony_compiler::Result<Vec<euphony_compiler::ConvertedBuffer>, E>,
        E,
    >(
        &self,
        _path: &str,
        _sample_rate: u64,
        _init: F,
    ) -> euphony_compiler::Result<Vec<euphony_compiler::CachedBuffer>, E> {
        unimplemented!()
    }
}
