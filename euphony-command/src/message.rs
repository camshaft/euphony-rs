use core::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Message<'a> {
    SynthDef {
        id: u64,
        definition: SynthDef<'a>,
    },
    SetTime {
        time: Duration,
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
        parameter: u32,
        value: f32,
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SynthDef<'a> {
    pub name: &'a str,
    pub buffers: Vec<Buffer<'a>>,
    pub inputs: Vec<Input<'a>>,
    pub nodes: Vec<Node<'a>>,
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
