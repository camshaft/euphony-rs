use crate::message::{Node, NodeValue};
use euphony_macros::Command;
use serde::{Deserialize, Serialize};

#[derive(Command, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Add(NodeValue, NodeValue);

#[derive(Command, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sub(NodeValue, NodeValue);

#[derive(Command, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Mul(NodeValue, NodeValue);

#[derive(Command, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Div(NodeValue, NodeValue);

#[derive(Command, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sine {
    #[cmd(default = 444.0)]
    pub frequency: NodeValue,
    pub phase: NodeValue,
}
