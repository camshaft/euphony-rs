use rayon::prelude::*;
use smallvec::SmallVec;
use std::{
    cell::UnsafeCell,
    collections::BTreeMap,
    ops,
    sync::atomic::{AtomicU8, Ordering},
};

type Map<K, V> = BTreeMap<K, V>;
type NodeId = u64;
type NodeMap<const BATCH_SIZE: usize> = Map<NodeId, Node<BATCH_SIZE>>;
type BufferMap = Map<u64, Vec<u8>>;

pub struct Graph<const BATCH_SIZE: usize> {
    samples: u64,
    epoch: u64,
    nodes: NodeMap<BATCH_SIZE>,
    buffers: BufferMap,
    current_view: usize,
}

impl<const BATCH_SIZE: usize> Default for Graph<BATCH_SIZE> {
    fn default() -> Self {
        Self {
            samples: 0,
            epoch: 0,
            nodes: NodeMap::new(),
            buffers: BufferMap::new(),
            current_view: 0,
        }
    }
}

impl<const BATCH_SIZE: usize> Graph<BATCH_SIZE> {
    pub fn spawn<P: Processor<BATCH_SIZE>>(&mut self, id: u64, processor: P) {
        let epoch = self.epoch as u8;
        self.nodes.insert(id, Node::new(processor, epoch));
    }

    pub fn finish(&mut self, id: u64) {
        self.nodes.remove(&id);
    }

    pub fn set(&mut self, target_node: u64, target_parameter: u64, value: f64) {
        let node = self.nodes.get_mut(&target_node).unwrap();
        node.set(target_parameter, value);
    }

    pub fn pipe(&mut self, target_node: u64, target_parameter: u64, source_node: u64) {
        let node = self.nodes.get_mut(&target_node).unwrap();
        node.pipe(target_parameter, source_node);
    }

    pub fn render_batch(&mut self, partial: Option<usize>) {
        let batch = &Batch {
            epoch: self.epoch as u8,
            nodes: &self.nodes,
            buffers: &self.buffers,
            partial,
        };

        self.nodes
            .par_iter()
            .for_each(|(_id, node)| batch.node(node));

        self.epoch += 1;
        self.current_view = partial.unwrap_or(BATCH_SIZE);
        self.samples += self.current_view as u64;
    }

    pub fn samples(&self) -> u64 {
        self.samples
    }
}

impl<const BATCH_SIZE: usize> ops::Index<u64> for Graph<BATCH_SIZE> {
    type Output = [f32];

    fn index(&self, index: u64) -> &Self::Output {
        if let Some(node) = self.nodes.get(&index) {
            &node.output()[..self.current_view]
        } else {
            &[][..]
        }
    }
}

struct Batch<'a, const BATCH_SIZE: usize> {
    epoch: u8,
    nodes: &'a NodeMap<BATCH_SIZE>,
    buffers: &'a BufferMap,
    partial: Option<usize>,
}

impl<'a, const BATCH_SIZE: usize> Batch<'a, BATCH_SIZE> {
    fn node(&self, node: &Node<BATCH_SIZE>) {
        // try to lock the node for this epoch, if this fails something is already working on
        // it
        if !node.acquire(self.epoch) {
            return;
        }

        self.dependencies(node.dependencies());
        node.render(self.nodes, self.buffers, self.partial);
    }

    fn dependencies(&self, deps: &[u64]) {
        deps.par_iter().for_each(|id| {
            if let Some(node) = self.nodes.get(id) {
                self.node(node);
            }
        });
    }
}

struct Node<const BATCH_SIZE: usize> {
    state: UnsafeCell<Box<NodeState<BATCH_SIZE>>>,
}

impl<const BATCH_SIZE: usize> Node<BATCH_SIZE> {
    fn new<P: Processor<BATCH_SIZE>>(processor: P, epoch: u8) -> Self {
        Self {
            state: UnsafeCell::new(NodeState::new(processor, epoch)),
        }
    }

    fn acquire(&self, epoch: u8) -> bool {
        unsafe { &*self.state.get() }
            .epoch
            .compare_exchange(
                epoch,
                epoch.wrapping_add(1),
                Ordering::Acquire,
                Ordering::Acquire,
            )
            .is_ok()
    }

    fn output(&self) -> &[f32; BATCH_SIZE] {
        &unsafe { &*self.state.get() }.output
    }

    fn dependencies(&self) -> &[u64] {
        &unsafe { &*self.state.get() }.dependencies
    }

    fn set(&self, parameter: u64, value: f64) {
        let parameter = parameter as usize;
        let state = unsafe { &mut *self.state.get() };

        // if we have some dependencies, make sure it's cleared before updating the constant
        if state.dependencies.len() > parameter {
            self.pipe(parameter as _, u64::MAX);
        }

        if state.constants.len() <= parameter {
            state.constants.resize_with(parameter + 1, || 0.0);
        }

        state.constants[parameter] = value;
    }

    fn pipe(&self, parameter: u64, source_node: u64) {
        let parameter = parameter as usize;
        let state = unsafe { &mut *self.state.get() };

        // if we have some constants, make sure it's cleared before updating the dependency
        if state.constants.len() > parameter {
            self.set(parameter as _, 0.0);
        }

        if state.dependencies.len() <= parameter {
            state.dependencies.resize_with(parameter + 1, || u64::MAX);
        }

        state.dependencies[parameter] = source_node;
    }

    fn render(&self, nodes: &NodeMap<BATCH_SIZE>, buffers: &BufferMap, partial: Option<usize>) {
        let state = unsafe { &mut *self.state.get() };
        let output = &mut state.output;

        let mut inputs = SmallVec::<[Input<BATCH_SIZE>; 32]>::new();

        for idx in 0..state.inner.inputs() {
            let dep = state
                .dependencies
                .get(idx)
                .filter(|dep| **dep != u64::MAX)
                .and_then(|dep| nodes.get(dep))
                .map(|node| node.output());

            if let Some(output) = dep {
                inputs.push(Input::Dynamic(output));
            } else {
                let v = *state.constants.get(idx).unwrap_or(&0.0);
                inputs.push(Input::Constant(v));
            }
        }

        let buffers = Buffers {
            inputs: &state.buffers,
            buffers,
        };

        if let Some(partial) = partial {
            state
                .inner
                .render_partial(&inputs, buffers, &mut output[..partial])
        } else {
            state.inner.render(&inputs, buffers, output)
        }
    }
}

unsafe impl<const BATCH_SIZE: usize> Sync for Node<BATCH_SIZE> {}

struct NodeState<const BATCH_SIZE: usize> {
    output: [f32; BATCH_SIZE],
    inner: Box<dyn Processor<BATCH_SIZE>>,
    epoch: AtomicU8,
    buffers: Vec<u64>,
    constants: SmallVec<[f64; 4]>,
    dependencies: SmallVec<[u64; 4]>,
}

impl<const BATCH_SIZE: usize> NodeState<BATCH_SIZE> {
    fn new<P: Processor<BATCH_SIZE>>(processor: P, epoch: u8) -> Box<Self> {
        Box::new(Self {
            epoch: AtomicU8::new(epoch),
            output: [0.0; BATCH_SIZE],
            buffers: vec![],
            constants: Default::default(),
            dependencies: Default::default(),
            inner: Box::new(processor),
        })
    }
}

pub trait Processor<const BATCH_SIZE: usize>: 'static + Send {
    fn inputs(&self) -> usize;

    fn render(
        &mut self,
        inputs: &[Input<BATCH_SIZE>],
        buffers: Buffers,
        output: &mut [f32; BATCH_SIZE],
    );

    fn render_partial(
        &mut self,
        inputs: &[Input<BATCH_SIZE>],
        buffers: Buffers,
        output: &mut [f32],
    );
}

pub struct Buffers<'a> {
    inputs: &'a [u64],
    buffers: &'a BufferMap,
}

impl<'a> Buffers<'a> {
    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }
}

impl<'a> ops::Index<usize> for Buffers<'a> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let idx = self.inputs[index];
        &*self.buffers.get(&idx).unwrap()
    }
}

pub enum Input<'a, const BATCH_SIZE: usize> {
    Constant(f64),
    Dynamic(&'a [f32; BATCH_SIZE]),
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[derive(Debug, Default)]
    struct Counter(u64);

    impl<const BATCH_SIZE: usize> Processor<BATCH_SIZE> for Counter {
        fn inputs(&self) -> usize {
            0
        }

        fn render(
            &mut self,
            inputs: &[Input<BATCH_SIZE>],
            buffers: Buffers,
            output: &mut [f32; BATCH_SIZE],
        ) {
            self.render_partial(inputs, buffers, output);
        }

        fn render_partial(
            &mut self,
            _inputs: &[Input<BATCH_SIZE>],
            _buffers: Buffers,
            output: &mut [f32],
        ) {
            for sample in output.iter_mut() {
                let value = self.0;
                *sample = value as _;
                self.0 += 1;
            }
        }
    }

    #[derive(Debug, Default)]
    struct Add;

    impl<const BATCH_SIZE: usize> Processor<BATCH_SIZE> for Add {
        fn inputs(&self) -> usize {
            2
        }

        fn render(
            &mut self,
            inputs: &[Input<BATCH_SIZE>],
            buffers: Buffers,
            output: &mut [f32; BATCH_SIZE],
        ) {
            self.render_partial(inputs, buffers, output)
        }

        fn render_partial(
            &mut self,
            inputs: &[Input<BATCH_SIZE>],
            _buffers: Buffers,
            output: &mut [f32],
        ) {
            // clear the output before adding anything
            for sample in output.iter_mut() {
                *sample = 0.0;
            }

            let mut write = |input: &Input<BATCH_SIZE>| {
                match input {
                    Input::Dynamic(input) => {
                        for (sample, source) in output.iter_mut().zip(input.iter()) {
                            *sample += source;
                        }
                    }
                    Input::Constant(value) => {
                        for sample in output.iter_mut() {
                            *sample += *value as f32;
                        }
                    }
                };
            };

            write(&inputs[0]);
            write(&inputs[1]);
        }
    }

    #[test]
    fn counter_test() {
        let mut graph = Graph::<32>::default();

        graph.spawn(0, Counter::default());

        graph.render_batch(None);

        assert_debug_snapshot!("counter_test", &graph[0]);
    }

    #[test]
    fn add_dynamic_test() {
        let mut graph = Graph::<32>::default();

        graph.spawn(0, Counter::default());
        graph.spawn(1, Counter::default());
        graph.spawn(2, Add);
        graph.pipe(2, 0, 0);
        graph.pipe(2, 1, 1);

        graph.render_batch(None);

        assert_debug_snapshot!("add_dynamic_test", &graph[2]);
    }

    #[test]
    fn add_constant_test() {
        let mut graph = Graph::<32>::default();

        graph.spawn(0, Counter::default());
        graph.spawn(1, Add);
        graph.pipe(1, 0, 0);
        graph.set(1, 1, 3.0);

        graph.render_batch(None);

        assert_debug_snapshot!("add_constant_test", &graph[1]);
    }

    #[test]
    fn diamond_test() {
        let mut graph = Graph::<32>::default();

        graph.spawn(0, Counter::default());

        graph.spawn(1, Add);
        graph.pipe(1, 0, 0);
        graph.set(1, 1, 42.0);

        graph.spawn(2, Add);
        graph.pipe(2, 0, 0);
        graph.set(2, 1, 42.0);

        graph.spawn(3, Add);
        graph.pipe(3, 0, 1);
        graph.pipe(3, 1, 2);

        graph.render_batch(None);

        assert_debug_snapshot!("diamond_test", &graph[3]);
    }

    #[test]
    fn many_test() {
        let mut graph = Graph::<32>::default();

        for i in 0..1000 {
            graph.spawn(i, Counter::default());
        }

        graph.render_batch(None);

        assert_debug_snapshot!("many_test", &graph[999]);
    }
}
