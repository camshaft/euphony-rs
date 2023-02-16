use super::*;
use bolero::{check, generator::*};

struct Config;

impl super::Config for Config {
    type Output = Option<u64>;
    type Parameter = u8;
    type Value = ();
    type Context = ();
}

#[derive(Clone, Copy, Debug, TypeGenerator)]
enum Op {
    Insert {
        #[generator(0..4)]
        inputs: u8,
    },
    Set {
        idx: u16,
        parameter: u8,
    },
    Connect {
        idx: u16,
        parameter: u8,
        source: u16,
    },
    Remove {
        idx: u16,
    },
    Process,
}

#[derive(Debug, Default)]
struct Ids {
    current: u64,
    ids: VecDeque<u64>,
}

impl Ids {
    fn spawn(&mut self) -> u64 {
        let id = self.current;
        self.current += 1;
        self.ids.push_back(id);
        id
    }

    fn get(&self, idx: u16) -> (u64, Result<(), Error<u8>>) {
        if self.ids.is_empty() {
            let idx = idx as u64;
            return (idx, Err(Error::MissingNode(idx)));
        }

        let idx = idx as usize % self.ids.len();
        let id = self.ids[idx];
        (id, Ok(()))
    }

    fn remove(&mut self, idx: u16) -> (u64, Result<(), Error<u8>>) {
        if self.ids.is_empty() {
            let idx = idx as u64;
            return (idx, Err(Error::MissingNode(idx)));
        }

        let idx = idx as usize % self.ids.len();
        let id = self.ids.remove(idx).unwrap();
        (id, Ok(()))
    }

    fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        self.ids.iter().copied()
    }
}

struct Processor {
    id: u64,
    inputs: Vec<Input<()>>,
    output: Option<u64>,
}

impl Processor {
    fn new(id: u64, inputs: u8) -> Self {
        Self {
            id,
            inputs: Iterator::map(0..inputs, |_| Input::Value(())).collect(),
            output: None,
        }
    }
}

impl super::Processor<Config> for Processor {
    fn set(&mut self, param: u8, key: Input<()>) -> Result<Input<()>, u8> {
        if self.inputs.is_empty() {
            return Ok(Input::Value(()));
        }

        let idx = param as usize % self.inputs.len();
        let input = &mut self.inputs[idx];
        Ok(core::mem::replace(input, key))
    }

    fn remove(&mut self, key: NodeKey) {
        for input in self.inputs.iter_mut() {
            if let Input::Node(node_key) = input {
                if *node_key == key {
                    *input = Input::Value(());
                }
            }
        }
    }

    fn output(&self) -> &Option<u64> {
        &self.output
    }

    fn output_mut(&mut self) -> &mut Option<u64> {
        &mut self.output
    }

    fn process(&mut self, inputs: Inputs<Config>, _context: &()) {
        for input in self.inputs.iter() {
            if let Input::Node(node) = input {
                let _ = inputs[*node];
            }
        }

        self.output = Some(self.id);
    }

    fn fork(&self) -> Option<Box<dyn super::Processor<Config>>> {
        todo!()
    }
}

fn model(ops: &[Op]) -> Result<(), Error<u8>> {
    let mut subject = Graph::<Config>::default();

    let mut ids = Ids::default();

    for op in ops {
        match *op {
            Op::Insert { inputs } => {
                let id = ids.spawn();
                subject.insert(id, Box::new(Processor::new(id, inputs)));
            }
            Op::Set { idx, parameter } => {
                let (id, expected) = ids.get(idx);
                let actual = subject.set(id, parameter, ());
                assert_eq!(expected, actual);
            }
            Op::Connect {
                idx,
                parameter,
                source,
            } => {
                let (node, expected_node) = ids.get(idx);
                let (source, expected_source) = ids.get(source);
                let expected = if node == source {
                    Err(Error::CycleDetected)
                } else {
                    expected_node.and(expected_source)
                };
                let actual = subject.connect(node, parameter, source);
                assert_eq!(expected, actual);
            }
            Op::Remove { idx } => {
                let (id, expected) = ids.remove(idx);
                let actual = subject.remove(id).map(|_| ());
                assert_eq!(expected, actual);
            }
            Op::Process => {
                // update all of the levels before rendering
                subject.update()?;

                subject.process(&());

                // make sure the render function was called for each one
                for id in ids.iter() {
                    let out = subject.get_mut(id).unwrap();
                    assert_eq!(*out, Some(id));
                    // reset the output for the next render
                    *out = None;
                }
            }
        }
    }

    Ok(())
}

#[test]
fn interdep_test() {
    model(&[
        Op::Insert { inputs: 1 },
        Op::Insert { inputs: 1 },
        Op::Connect {
            idx: 1,
            parameter: 0,
            source: 0,
        },
        Op::Process,
        Op::Connect {
            idx: 0,
            parameter: 0,
            source: 1,
        },
        Op::Process,
    ])
    .expect_err("cycle should be detected");
}

#[test]
fn cycle_test() {
    model(&[
        Op::Insert { inputs: 1 },
        Op::Insert { inputs: 1 },
        Op::Insert { inputs: 1 },
        Op::Connect {
            idx: 1,
            parameter: 0,
            source: 0,
        },
        Op::Connect {
            idx: 2,
            parameter: 0,
            source: 1,
        },
        Op::Process,
        Op::Connect {
            idx: 0,
            parameter: 0,
            source: 1,
        },
        Op::Process,
    ])
    .expect_err("cycle should be detected");
}

#[test]
fn model_test() {
    check!().with_type::<Vec<Op>>().for_each(|ops| {
        let _ = model(ops);
    });
}
