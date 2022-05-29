use crate::{sample::Offset, Hash};
use core::fmt;
use euphony_node::ParameterValue as Value;
use std::collections::btree_set;

#[derive(Clone, Debug)]
pub struct Instructions<'a> {
    pub samples: Offset,
    pub iter: core::iter::Peekable<btree_set::Iter<'a, (Offset, InternalInstruction)>>,
}

impl<'a> Iterator for Instructions<'a> {
    type Item = Instruction;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut next_samples = None;
        if let Some((_, instr)) = self.iter.next_if(|(s, _)| {
            if *s == self.samples {
                true
            } else {
                next_samples = Some(*s);
                false
            }
        }) {
            return Some((*instr).into());
        }

        if let Some(samples) = next_samples {
            let count = samples.since(self.samples).into();
            self.samples = samples;

            return Some(Instruction::AdvanceSamples { count });
        }

        let (_, instr) = self.iter.next()?;
        Some((*instr).into())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.iter.size_hint();
        // remove the upper since we emit AdvanceSamples as well
        (lower, None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    AdvanceSamples {
        count: u64,
    },
    SpawnNode {
        id: u64,
        processor: u64,
    },
    SpawnSink {
        id: u64,
        hash: Hash,
    },
    SetParameter {
        target_node: u64,
        target_parameter: u64,
        value: Value,
    },
    FinishNode {
        node: u64,
    },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::AdvanceSamples { count } => write!(f, "ADV {}", count),
            Instruction::SpawnNode { id, processor } => write!(f, "  SPN {},{}", id, processor),
            Instruction::SpawnSink { id, hash } => {
                write!(f, "  SNK {},0x", id)?;
                for byte in hash {
                    write!(f, "{:x}", byte)?;
                }
                Ok(())
            }
            Instruction::SetParameter {
                target_node,
                target_parameter,
                value,
            } => match value {
                Value::Node(n) => {
                    write!(f, "  PIP {},{},{}", target_node, target_parameter, n)
                }
                Value::Constant(value) => {
                    write!(f, "  SET {},{},{}", target_node, target_parameter, value)
                }
                Value::Buffer((buffer, channel)) => {
                    write!(
                        f,
                        "  BUF {},{},{},{}",
                        target_node, target_parameter, buffer, channel
                    )
                }
            },
            Instruction::FinishNode { node } => write!(f, "  FIN {}", node),
        }
    }
}

impl From<InternalInstruction> for Instruction {
    fn from(inst: InternalInstruction) -> Self {
        use InternalInstruction::*;
        match inst {
            SpawnNode { id, processor } => Self::SpawnNode { id, processor },
            SpawnSink { id, hash } => Self::SpawnSink { id, hash },
            SetParameter {
                target_node,
                target_parameter,
                value,
            } => Self::SetParameter {
                target_node,
                target_parameter,
                value: Value::Constant(f64::from_bits(value)),
            },
            SetBuffer {
                target_node,
                target_parameter,
                buffer,
                buffer_channel,
            } => Self::SetParameter {
                target_node,
                target_parameter,
                value: Value::Buffer((buffer, buffer_channel)),
            },
            ConnectParameter {
                target_node,
                target_parameter,
                source_node,
            } => Self::SetParameter {
                target_node,
                target_parameter,
                value: Value::Node(source_node),
            },
            FinishNode { node } => Self::FinishNode { node },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InternalInstruction {
    SpawnNode {
        id: u64,
        processor: u64,
    },
    SpawnSink {
        id: u64,
        hash: Hash,
    },
    SetParameter {
        target_node: u64,
        target_parameter: u64,
        value: u64,
    },
    SetBuffer {
        target_node: u64,
        target_parameter: u64,
        buffer: u64,
        buffer_channel: u64,
    },
    ConnectParameter {
        target_node: u64,
        target_parameter: u64,
        source_node: u64,
    },
    FinishNode {
        node: u64,
    },
}
