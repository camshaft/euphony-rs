use crate::{
    buffer::Buffer,
    group::{self, GroupMap},
    instruction::{Instructions, InternalInstruction},
    node::Node,
    parallel::*,
    sample::{default_samples_per_tick, samples_per_tick, Offset},
    sink::{Sink, SinkMap},
    Hash, Result, Writer,
};
use euphony_command::{self as message, Handler};
use euphony_dsp::nodes;
use euphony_node::{BufferMap, ParameterValue as Value};
use euphony_units::ratio::Ratio;
use petgraph::{
    visit::{depth_first_search, DfsEvent},
    Graph,
};
use std::collections::{hash_map, BTreeSet, HashMap};

#[derive(Debug)]
pub struct Compiler {
    groups: GroupMap,
    nodes: HashMap<u64, Node>,
    sinks: SinkMap,
    hashes: HashMap<Hash, u64>, // TODO use hash hasher
    active_nodes: BTreeSet<u64>,
    connections: Graph<u64, ()>,
    instructions: BTreeSet<(Offset, InternalInstruction)>,
    samples: Offset,
    samples_per_tick: Ratio<u128>,
    pending_buffers: HashMap<u64, (String, String)>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            groups: Default::default(),
            nodes: Default::default(),
            sinks: Default::default(),
            hashes: Default::default(),
            active_nodes: Default::default(),
            connections: Default::default(),
            instructions: Default::default(),
            samples: Default::default(),
            samples_per_tick: default_samples_per_tick(),
            pending_buffers: Default::default(),
        }
    }
}

impl Compiler {
    pub fn finalize<W: Writer>(&mut self, cache: &W) -> Result<Box<dyn BufferMap>> {
        let samples = self.samples;

        let hasher = blake3::Hasher::new();

        self.nodes.par_iter_mut().for_each(|(_id, node)| {
            if node.end.is_none() {
                let _ = node.finish(samples);
            }

            node.hash(&hasher);
        });

        let buffers = self
            .pending_buffers
            .par_iter()
            .flat_map(
                |(id, (path, ext))| match Buffer::load(*id, path, ext, cache) {
                    Ok(values) => values,
                    Err(err) => {
                        log::error!("could not load buffer {:?}: {}", path, err);
                        vec![]
                    }
                },
            )
            .collect();
        let buffers = crate::buffer::Map::new(buffers);

        // TODO iterate over each node and call optimize

        self.sinks.par_iter_mut().for_each(|(id, sink)| {
            let node = &self.nodes[id];
            let index = node.index;

            let mut start = node.start;
            let mut end = None;

            let conns = &self.connections;
            let mut hasher = hasher.clone();

            depth_first_search(conns, Some(index), |event| match event {
                DfsEvent::Discover(dep, _time) => {
                    let id = conns[dep];
                    sink.members.insert(id);
                    let dep = &self.nodes[&id];

                    hasher.update(&dep.hash);

                    // find the earliest node
                    start = start.min(dep.start);

                    // make sure to include all of the hashes of the deps
                    for ((sample, param, _type), input) in &dep.inputs {
                        match input {
                            Value::Node(source) => {
                                let source = &self.nodes[source];
                                // compute the earlier of the start times
                                let base = dep.start.min(source.start);
                                end = end.max(source.end);
                                // compute the relative sample to the base
                                let sample = (dep.start + *sample).since(base);
                                hasher.update(&sample.to_bytes());
                                hasher.update(&param.to_le_bytes());
                                hasher.update(&source.hash);
                            }
                            Value::Buffer((id, channel)) => {
                                let buffer = buffers.get(*id, *channel);
                                hasher.update(&sample.to_bytes());
                                hasher.update(&param.to_le_bytes());
                                hasher.update(&buffer.hash[..]);
                            }
                            _ => (),
                        }
                    }
                }
                DfsEvent::BackEdge(_, _) => {
                    sink.is_acyclic = false;
                }
                _ => {}
            });

            sink.start = start;
            sink.end = end.unwrap_or_default();
            sink.hash = *hasher.finalize().as_bytes();
            sink.is_cached = cache.is_cached(&sink.hash);
        });

        for (id, sink) in &self.sinks {
            if !sink.is_acyclic {
                return Err(error!("acyclic sink {}", id));
            }

            // the sink already exists
            if sink.is_cached {
                continue;
            }

            if let hash_map::Entry::Vacant(entry) = self.hashes.entry(sink.hash) {
                entry.insert(*id);

                // TODO create unique instances of each member for the sink and shift the time to
                // the start

                // mark all of the members of the sink tree active
                self.active_nodes.extend(&sink.members);
            }
        }

        self.instructions
            .par_extend(self.active_nodes.par_iter().copied().flat_map(|id| {
                let mut instructions = vec![];
                let node = &self.nodes[&id];
                node.instructions(id, &self.sinks, &mut instructions);
                instructions
            }));

        self.groups.par_iter_mut().for_each(|(_, group)| {
            group.update_hash(&self.sinks);
        });

        Ok(Box::new(buffers))
    }

    #[inline]
    pub fn reset(&mut self) {
        self.groups.clear();
        self.nodes.clear();
        self.sinks.clear();
        self.hashes.clear();
        self.connections.clear();
        self.active_nodes.clear();
        self.instructions.clear();
        self.pending_buffers.clear();
        self.samples = Offset::default();
        self.samples_per_tick = default_samples_per_tick();
    }

    #[inline]
    pub fn instructions(&self) -> Instructions {
        Instructions {
            samples: Default::default(),
            iter: self.instructions.iter().peekable(),
        }
    }

    pub fn groups(&self) -> group::Iter {
        group::Iter {
            iter: self.groups.iter(),
            sinks: &self.sinks,
        }
    }

    #[inline]
    fn node(&mut self, id: u64) -> Result<&mut Node> {
        self.nodes
            .get_mut(&id)
            .ok_or_else(|| error!("missing node {}", id))
    }
}

impl Handler for Compiler {
    #[inline]
    fn advance_time(&mut self, msg: message::AdvanceTime) -> Result {
        if msg.ticks == 0 {
            return Ok(());
        }

        let samples = self
            .samples_per_tick
            .0
            .checked_mul(msg.ticks as u128)
            .ok_or_else(|| error!("sample overflow"))?;
        let samples = samples / self.samples_per_tick.1;
        let samples = samples as u64;

        // limit the number of samples in testing so we don't churn indefinitely
        // TODO this should probably error out if the requested time exceeds an hour or something
        #[cfg(any(test, all(test, fuzz)))]
        let samples = {
            use euphony_dsp::sample::{DefaultRate, Rate};
            samples.min(DefaultRate::COUNT)
        };

        self.samples = self
            .samples
            .checked_add(samples)
            .ok_or_else(|| error!("sample overflow"))?;

        Ok(())
    }

    #[inline]
    fn set_nanos_per_tick(&mut self, msg: message::SetNanosPerTick) -> Result {
        if msg.nanos == 0 {
            return Err(error!("nanos per tick must be non-zero"));
        }

        self.samples_per_tick = samples_per_tick(msg.nanos);
        Ok(())
    }

    #[inline]
    fn create_group(&mut self, msg: message::CreateGroup) -> Result {
        self.groups.entry(msg.id).or_default().name = msg.name;
        Ok(())
    }

    #[inline]
    fn spawn_node(&mut self, msg: message::SpawnNode) -> Result {
        let processor = msg.processor;

        if processor == 0 {
            let group = msg.group.unwrap_or(0);
            self.sinks.insert(
                msg.id,
                Sink {
                    start: self.samples,
                    ..Default::default()
                },
            );
            self.groups
                .entry(group)
                .or_default()
                .sinks
                .insert((self.samples, msg.id));
        } else if nodes::name(processor).is_none() {
            return Err(error!("non-existant processor {}", processor));
        }

        let index = self.connections.add_node(msg.id);

        let prev = self.nodes.insert(
            msg.id,
            Node {
                index,
                processor,
                inputs: Default::default(),
                start: self.samples,
                end: None,
                hash: [0; 32],
            },
        );

        if prev.is_some() {
            return Err(error!("node id {} was reused", msg.id));
        }

        Ok(())
    }

    #[inline]
    fn set_parameter(&mut self, msg: message::SetParameter) -> Result {
        let message::SetParameter {
            target_node,
            target_parameter,
            value,
        } = msg;

        let samples = self.samples;
        let node = self.node(target_node)?;
        node.set(target_parameter, value, samples)?;

        Ok(())
    }

    #[inline]
    fn pipe_parameter(&mut self, msg: message::PipeParameter) -> Result {
        let message::PipeParameter {
            target_node,
            target_parameter,
            source_node,
        } = msg;

        if self.sinks.contains_key(&source_node) {
            return Err(error!("cannot connect sink output to another node"));
        }

        let samples = self.samples;
        let source_idx = self.node(source_node)?.index;

        let node = self.node(target_node)?;
        node.connect(target_parameter, source_node, samples)?;
        let target_idx = node.index;

        self.connections.add_edge(target_idx, source_idx, ());

        Ok(())
    }

    #[inline]
    fn finish_node(&mut self, msg: message::FinishNode) -> Result {
        let samples = self.samples;
        let node = self.node(msg.node)?;
        node.finish(samples)?;
        Ok(())
    }

    #[inline]
    fn load_buffer(&mut self, msg: message::LoadBuffer) -> std::io::Result<()> {
        let message::LoadBuffer { id, path, ext } = msg;

        self.pending_buffers.insert(id, (path, ext));

        Ok(())
    }

    #[inline]
    fn set_buffer(&mut self, msg: message::SetBuffer) -> std::io::Result<()> {
        let message::SetBuffer {
            target_node,
            target_parameter,
            buffer,
            buffer_channel,
        } = msg;

        let samples = self.samples;
        let node = self.node(target_node)?;
        node.set_buffer(target_parameter, buffer, buffer_channel, samples)?;

        Ok(())
    }
}
