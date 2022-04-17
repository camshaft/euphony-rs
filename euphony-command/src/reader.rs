use crate::message::{Message, SynthDef};
use core::time::Duration;
use std::collections::{btree_map, BTreeMap};

pub struct Program {
    now: Duration,
    groups: Vec<Group>,
    synths: Vec<Synth>,
}

impl Program {
    pub fn push(&mut self, message: &Message) {
        match message {
            Message::SynthDef { id, definition } => self.synthdef(*id, definition),
            Message::AdvanceTime { amount } => self.advance_time(*amount),
            Message::CreateGroup { id, name } => self.create_group(*id, name),
            Message::Spawn {
                id,
                synthdef,
                group,
                seed,
            } => self.spawn(*id, *synthdef, *group, *seed),
            Message::Set {
                id,
                parameter,
                value,
            } => self.set(*id, *parameter, *value),
            Message::Route {
                output,
                output_idx,
                input,
                input_idx,
            } => todo!(),
            Message::Drop { id } => self.finish_synth(*id),
        }
    }

    fn synthdef(&mut self, id: u64, def: &SynthDef) {
        // TODO
    }

    fn spawn(&mut self, id: u64, synthdef: u64, group: u64, seed: u64) {
        let id = id as usize;
        if self.synths.len() < id {
            self.synths.resize_with(id, Default::default);
        }

        let synth = &mut self.synths[id];
        debug_assert!(
            synth.ops.is_empty(),
            "trying to spawn multiple synths with same id"
        );
        synth.is_open = true;
        synth.start = self.now;
        synth.synthdef = synthdef;
        synth.group = group;
        synth.seed = seed;
    }

    fn create_group(&mut self, id: u64, name: &str) {
        let id = id as usize;
        if self.groups.len() < id {
            self.groups.resize_with(id, Default::default);
        }

        let group = &mut self.groups[id];
        group.name = name.to_string();
    }

    fn set(&mut self, id: u64, parameter: u64, value: f64) {
        let synth = &mut self.synths[id as usize];

        debug_assert!(synth.is_open, "trying to set parameter on closed synth");

        synth.ops.push(SynthOp::Set { parameter, value });
    }

    fn finish_synth(&mut self, id: u64) {
        let synth = &mut self.synths[id as usize];
        synth.is_open = false;
    }

    fn advance_time(&mut self, amount: Duration) {
        self.now += amount;

        for synth in &mut self.synths {
            if synth.is_open {
                synth.ops.push(SynthOp::AdvanceTime { amount });
            }
        }
    }

    fn finish(self) {
        let mut renderings = BTreeMap::new();

        for mut synth in self.synths {
            let def = &self.synthdefs[synth.synthdef as usize];

            let hash = def.hash(synth.seed);
            synth.hash(&mut hash);
            let hash = hash.finish();

            let next_id = renderings.len();

            let id = match renderings.entry(hash) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert((next_id, synth.ops, def));
                    next_id
                }
                btree_map::Entry::Occupied(entry) => entry.get().0,
            };

            let group = self.groups[synth.group as usize];
            group.entries.push(Entry {
                start: synth.start,
                index: id,
            });
        }
    }
}

#[derive(Debug, Default)]
struct Group {
    name: String,
    entries: Vec<Entry>,
}

#[derive(Debug, Default)]
struct Entry {
    start: Duration,
    index: usize,
}

#[derive(Debug, Default)]
struct Synth {
    is_open: bool,
    start: Duration,
    synthdef: u64,
    seed: u64,
    group: u64,
    ops: Vec<SynthOp>,
}

impl core::hash::Hash for Synth {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        SynthOp::hash_slice(&self.ops, state)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SynthOp {
    Set { parameter: u64, value: f64 },
    AdvanceTime { amount: Duration },
}

impl PartialEq for SynthOp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Set {
                    parameter: a_p,
                    value: a_v,
                },
                Self::Set {
                    parameter: b_p,
                    value: b_v,
                },
            ) => a_p == b_p && a_v == b_v,
            (Self::AdvanceTime { amount: a }, Self::AdvanceTime { amount: b }) => a == b,
            _ => false,
        }
    }
}

impl core::hash::Hash for SynthOp {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Set { parameter, value } => {
                0u8.hash(state);
                parameter.hash(state);
                value.to_bits().hash(state);
            }
            Self::AdvanceTime { amount } => {
                1u8.hash(state);
                amount.hash(state);
            }
        }
    }
}
