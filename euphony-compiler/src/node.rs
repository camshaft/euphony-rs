use crate::{
    instruction::InternalInstruction,
    sample::{Offset, RelOffset},
    sink::SinkMap,
    Hash, Result,
};
use blake3::Hasher;
use euphony_dsp::nodes;
use euphony_node::ParameterValue as Value;
use petgraph::graph::NodeIndex;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Node {
    pub index: NodeIndex,
    pub inputs: BTreeMap<(RelOffset, u64, InputType), Value>,
    pub processor: u64,
    pub start: Offset,
    pub end: Option<RelOffset>,
    pub fork_source: Option<u64>,
    pub hash: Hash,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InputType {
    Signal,
    Buffer,
}

impl Node {
    pub fn set(&mut self, parameter: u64, value: u64, sample: Offset) -> Result {
        let value = euphony_node::ParameterValue::Constant(f64::from_bits(value));
        self.validate(parameter, value)?;

        let sample = sample.since(self.start);
        self.inputs
            .insert((sample, parameter, InputType::Signal), value);

        Ok(())
    }

    pub fn set_buffer(
        &mut self,
        parameter: u64,
        buffer: u64,
        channel: u64,
        sample: Offset,
    ) -> Result {
        let value = euphony_node::ParameterValue::Buffer((buffer, channel));
        self.validate(parameter, value)?;

        let sample = sample.since(self.start);
        self.inputs
            .insert((sample, parameter, InputType::Buffer), value);

        Ok(())
    }

    pub fn connect(&mut self, parameter: u64, source: u64, sample: Offset) -> Result {
        let value = euphony_node::ParameterValue::Node(source);
        self.validate(parameter, value)?;

        let sample = sample.since(self.start);
        self.inputs
            .insert((sample, parameter, InputType::Signal), value);

        Ok(())
    }

    pub fn finish(&mut self, sample: Offset) -> Result {
        if self.end.is_some() {
            return Err(error!("node has already been finished"));
        }

        self.end = Some(sample.since(self.start));

        Ok(())
    }

    pub fn hash(&mut self, hasher: &Hasher) {
        let mut hasher = hasher.clone();
        hasher.update(&self.processor.to_le_bytes());
        hasher.update(&self.end.unwrap().to_bytes());

        for ((sample, param, _type), value) in self.inputs.iter() {
            hasher.update(&sample.to_bytes());
            hasher.update(&param.to_le_bytes());

            if let Value::Constant(v) = value {
                hasher.update(&v.to_le_bytes());
            } else {
                // the node parameter will be computed later by the sink
                hasher.update(&[0; 8]);
            }
        }

        self.hash = *hasher.finalize().as_bytes();
    }

    pub fn instructions(
        &self,
        id: u64,
        sinks: &SinkMap,
        instructions: &mut Vec<(Offset, InternalInstruction)>,
    ) {
        let processor = self.processor;
        let offset = self.start;

        if processor == 0 {
            let hash = sinks[&id].hash;
            instructions.push((offset, InternalInstruction::SpawnSink { id, hash }));
        } else if let Some(source) = self.fork_source {
            instructions.push((offset, InternalInstruction::ForkNode { source, target: id }));
        } else {
            instructions.push((offset, InternalInstruction::SpawnNode { id, processor }));
        }

        for ((sample, parameter, _type), value) in &self.inputs {
            let offset = offset + *sample;
            let target_node = id;
            let target_parameter = *parameter;

            match *value {
                Value::Node(source_node) => {
                    instructions.push((
                        offset,
                        InternalInstruction::ConnectParameter {
                            target_node,
                            target_parameter,
                            source_node,
                        },
                    ));
                }
                Value::Constant(value) => {
                    instructions.push((
                        offset,
                        InternalInstruction::SetParameter {
                            target_node,
                            target_parameter,
                            value: value.to_bits(),
                        },
                    ));
                }
                Value::Buffer((buffer, buffer_channel)) => {
                    instructions.push((
                        offset,
                        InternalInstruction::SetBuffer {
                            target_node,
                            target_parameter,
                            buffer,
                            buffer_channel,
                        },
                    ));
                }
            }
        }

        let offset = offset + self.end.unwrap();
        instructions.push((offset, InternalInstruction::FinishNode { node: id }));
    }

    fn validate(&self, parameter: u64, value: euphony_node::ParameterValue) -> Result {
        if self.end.is_some() {
            return Err(error!("node has already been finished"));
        }

        // special case the sink
        if self.processor == 0 {
            if parameter > 4 {
                return Err(error!(
                    "invalid parameter for {}: {}",
                    self.processor, parameter
                ));
            }
            return Ok(());
        }

        nodes::validate_parameter(self.processor, parameter, value)
            .map_err(|err| error!("invalid parameter for {}: {}", self.processor, err))?;

        Ok(())
    }
}
