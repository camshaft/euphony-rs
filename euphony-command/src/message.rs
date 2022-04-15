use core::{fmt, time::Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Message<'a> {
    SynthDef {
        id: u64,
        definition: SynthDef<'a>,
    },
    AdvanceTime {
        amount: Duration,
    },
    SetGroupName {
        id: u64,
        name: &'a str,
    },
    /// Spawns a synth
    Spawn {
        id: u64,
        synthdef: u64,
        group: u64,
    },
    /// Sets a parameter on a synth
    Set {
        id: u64,
        parameter: u64,
        value: f64,
    },
    /// Routes the output of one synth to the input of another
    Route {
        output: u64,
        output_idx: u64,
        input: u64,
        input_idx: u64,
    },
    /// Drops a synth
    Drop {
        id: u64,
    },
    /// Sets the seed for any random unit generators
    SetSeed {
        seed: u64,
    },
    /// Finishes the program
    Finish,
}

impl<'a> Message<'a> {
    pub fn write<W: std::io::Write>(&self, w: W) -> std::io::Result<()> {
        bincode::serialize_into(w, self).map_err(|err| match *err {
            bincode::ErrorKind::Io(err) => err,
            other => std::io::Error::new(std::io::ErrorKind::Other, other),
        })?;
        Ok(())
    }
}

impl<'a> fmt::Display for Message<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO don't allocate, just write to the formatter
        let s = serde_json::to_string(self).unwrap();
        s.fmt(f)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SynthDef<'a> {
    #[serde(skip_serializing_if = "str::is_empty", default)]
    pub name: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub buffers: Vec<Buffer<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub inputs: Vec<Input<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub nodes: Vec<Node<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub outputs: Vec<Output<'a>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Buffer<'a> {
    File { path: &'a str },
    Inline { data: &'a [u8] },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub inputs: Vec<NodeValue>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum NodeValue {
    Constant { value: f32 },
    Node { id: u32, output: u32 },
    Input { id: u32 },
    Buffer { id: u32 },
}

impl Default for NodeValue {
    fn default() -> Self {
        Self::Constant { value: 0.0 }
    }
}

impl From<f32> for NodeValue {
    fn from(value: f32) -> Self {
        Self::Constant { value }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Input<'a> {
    pub name: &'a str,
    pub default: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Output<'a> {
    pub name: &'a str,
    pub node: u32,
    pub index: u32,
}
