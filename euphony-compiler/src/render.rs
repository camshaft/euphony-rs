use crate::{instruction::Instruction, Hash, Writer};
use euphony_dsp::nodes::load as load_dsp;
use euphony_graph::Graph;
use euphony_node::{BufferMap, Config, Context, ParameterValue, Value};
use std::collections::BTreeSet;

pub type Error = euphony_graph::Error<u64>;
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

#[derive(Debug, Default)]
pub struct Renderer {
    graph: Graph<Config>,
    context: Context,
    sample_offset: u64,
    /// The sinks the render spawned and hasn't finished
    sinks: BTreeSet<u64>,
    /// Whether a render started and hasn't completed
    rendering: bool,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // a panic that unwinds through the render already keeps each sink from being stored
        if std::thread::panicking() {
            return;
        }

        self.discard_unfinished();
    }
}

impl Renderer {
    #[inline]
    pub fn set_buffers(&mut self, buffers: Box<dyn BufferMap>) {
        self.context.buffers = buffers;
    }

    /// Marks the start of a render: until `complete`, a reset or a drop of the renderer discards
    /// each sink the render leaves unfinished, as after a failed instruction or a caught panic
    #[inline]
    pub fn start(&mut self) {
        self.rendering = true;
    }

    /// Marks the render as complete, so its sinks are kept as they drop
    #[inline]
    pub fn complete(&mut self) {
        self.rendering = false;
    }

    /// Discards each sink of a render that started and didn't complete: the samples it holds are
    /// only part of the sink, so it drops without being kept and is rendered again
    pub fn discard_unfinished(&mut self) {
        if !core::mem::take(&mut self.rendering) {
            return;
        }

        for id in core::mem::take(&mut self.sinks) {
            if let Ok(mut sink) = self.graph.remove(id) {
                sink.discard();
            }
        }
    }

    #[inline]
    pub fn push<W: Writer>(&mut self, instr: Instruction, writer: &mut W) -> Result {
        match instr {
            Instruction::AdvanceSamples { count } => self.advance(count),
            Instruction::SpawnNode { id, processor } => self.spawn(id, processor),
            Instruction::ForkNode { source, target } => self.fork(source, target),
            Instruction::SpawnSink { id, hash } => self.sink(id, &hash, writer),
            Instruction::SetParameter {
                target_node,
                target_parameter,
                value,
            } => self.set(target_node, target_parameter, value),
            Instruction::FinishNode { node } => self.finish_node(node),
        }
    }

    pub fn reset(&mut self) {
        *self = Default::default();
        // TODO self.graph.clear();
    }

    #[inline]
    fn advance(&mut self, count: u64) -> Result {
        debug_assert_ne!(count, 0);

        let full = count / euphony_node::LEN as u64;
        let partial = count % euphony_node::LEN as u64;

        self.graph.update()?;

        self.context.partial = None;
        for _ in 0..full {
            self.graph.process(&self.context);
            self.sample_offset += euphony_node::LEN as u64;
        }

        if partial > 0 {
            self.context.partial = Some(partial as _);
            self.graph.process(&self.context);
            self.sample_offset += partial;
        }

        Ok(())
    }

    #[inline]
    fn spawn(&mut self, id: u64, processor: u64) -> Result {
        debug_assert_ne!(processor, 0);

        let node = load_dsp(processor);

        let node = unsafe {
            debug_assert!(node.is_some());
            node.unwrap_unchecked()
        };

        self.graph.insert(id, node);
        Ok(())
    }

    #[inline]
    fn fork(&mut self, source: u64, target: u64) -> Result {
        let source = self.graph.get_node(source)?;
        let node = source.fork().expect("cannot fork source node");
        self.graph.insert(target, node);
        Ok(())
    }

    #[inline]
    fn sink<W: Writer>(&mut self, id: u64, hash: &Hash, writer: &mut W) -> Result {
        let sink = writer.sink(hash);
        self.graph.insert(id, sink);
        self.sinks.insert(id);
        Ok(())
    }

    #[inline]
    fn set(&mut self, id: u64, param: u64, value: ParameterValue) -> Result {
        match value {
            ParameterValue::Constant(value) => {
                self.graph.set(id, param, Value::Constant(value))?;
            }
            ParameterValue::Node(source) => {
                self.graph.connect(id, param, source)?;
            }
            ParameterValue::Buffer(key) => {
                self.graph.set(id, param, Value::Buffer(key))?;
            }
        }
        Ok(())
    }

    #[inline]
    fn finish_node(&mut self, id: u64) -> Result {
        self.graph.remove(id)?;
        self.sinks.remove(&id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{midi, BufferReader, CachedBuffer, ConvertedBuffer, Entry};
    use euphony_node::{BoxProcessor, Sink};
    use euphony_units::coordinates::Polar;
    use std::sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    };

    /// What became of a sink the renderer spawned
    #[derive(Default)]
    struct Probe {
        samples: AtomicUsize,
        discarded: AtomicBool,
        dropped: AtomicBool,
    }

    impl Probe {
        fn samples(&self) -> usize {
            self.samples.load(Ordering::SeqCst)
        }

        fn discarded(&self) -> bool {
            self.discarded.load(Ordering::SeqCst)
        }

        fn dropped(&self) -> bool {
            self.dropped.load(Ordering::SeqCst)
        }
    }

    struct ProbeSink(Arc<Probe>);

    impl Sink for ProbeSink {
        fn write<S: Iterator<Item = (f64, Polar<f64>)>>(&mut self, samples: S) {
            self.0.samples.fetch_add(samples.count(), Ordering::SeqCst);
        }

        fn discard(&mut self) {
            self.0.discarded.store(true, Ordering::SeqCst);
        }
    }

    impl Drop for ProbeSink {
        fn drop(&mut self) {
            self.0.dropped.store(true, Ordering::SeqCst);
        }
    }

    #[derive(Default)]
    struct Output {
        probes: Vec<Arc<Probe>>,
    }

    impl Writer for Output {
        fn is_cached(&self, _hash: &Hash) -> bool {
            false
        }

        fn sink(&mut self, _hash: &Hash) -> BoxProcessor {
            let probe = Arc::new(Probe::default());
            self.probes.push(probe.clone());
            ProbeSink(probe).spawn()
        }

        fn group<I: Iterator<Item = Entry>>(
            &mut self,
            _name: &str,
            _hash: &Hash,
            entries: I,
            _midi: &midi::Writer,
        ) {
            for _ in entries {}
        }

        fn buffer<F: FnOnce(Box<dyn BufferReader>) -> Result<Vec<ConvertedBuffer>, E>, E>(
            &self,
            _path: &str,
            _sample_rate: u64,
            _init: F,
        ) -> Result<Vec<CachedBuffer>, E> {
            Ok(vec![])
        }
    }

    /// Spawns the sink `id`, and returns its probe
    fn spawn_sink(renderer: &mut Renderer, output: &mut Output, id: u64) -> Arc<Probe> {
        let hash = [id as u8; 32];
        renderer
            .push(Instruction::SpawnSink { id, hash }, output)
            .unwrap();
        output.probes.last().unwrap().clone()
    }

    #[test]
    fn a_render_that_fails_partway_discards_its_unfinished_sinks() {
        let mut renderer = Renderer::default();
        let mut output = Output::default();
        renderer.start();

        let unfinished = spawn_sink(&mut renderer, &mut output, 1);
        let finished = spawn_sink(&mut renderer, &mut output, 2);
        renderer
            .push(Instruction::AdvanceSamples { count: 1 }, &mut output)
            .unwrap();
        renderer
            .push(Instruction::FinishNode { node: 2 }, &mut output)
            .unwrap();

        // both sinks rendered a sample, and the finished one dropped without being discarded
        assert_eq!(unfinished.samples(), 1);
        assert_eq!(finished.samples(), 1);
        assert!(finished.dropped());
        assert!(!finished.discarded());
        assert!(!unfinished.dropped());

        // an instruction for a node that doesn't exist fails the render partway
        assert!(renderer
            .push(Instruction::FinishNode { node: 3 }, &mut output)
            .is_err());
        renderer.discard_unfinished();

        assert!(unfinished.discarded());
        assert!(unfinished.dropped());
        assert!(!finished.discarded());
    }

    #[test]
    fn a_reset_after_an_interrupted_render_discards_its_unfinished_sinks() {
        let mut renderer = Renderer::default();
        let mut output = Output::default();

        // as where a caller catches a panic of the render and compiles again
        renderer.start();
        let probe = spawn_sink(&mut renderer, &mut output, 1);
        renderer.reset();

        assert!(probe.discarded());
        assert!(probe.dropped());
    }

    #[test]
    fn a_dropped_renderer_discards_the_sinks_of_an_interrupted_render() {
        let mut renderer = Renderer::default();
        let mut output = Output::default();

        renderer.start();
        let probe = spawn_sink(&mut renderer, &mut output, 1);
        drop(renderer);

        assert!(probe.discarded());
        assert!(probe.dropped());
    }

    #[test]
    fn a_renderer_dropped_while_a_panic_unwinds_doesnt_discard_its_sinks() {
        let mut renderer = Renderer::default();
        let mut output = Output::default();

        renderer.start();
        let probe = spawn_sink(&mut renderer, &mut output, 1);
        assert!(!probe.dropped());

        // the renderer drops as the panic unwinds out of the closure that holds it
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _renderer = renderer;
            panic!("the render panics");
        }));
        assert!(result.is_err());

        // the sink dropped with the renderer, and is left to its own panicking check
        assert!(probe.dropped());
        assert!(!probe.discarded());
    }

    #[test]
    fn a_reset_after_a_complete_render_keeps_its_sinks() {
        let mut renderer = Renderer::default();
        let mut output = Output::default();

        renderer.start();
        let probe = spawn_sink(&mut renderer, &mut output, 1);
        renderer.complete();
        renderer.reset();

        assert!(probe.dropped());
        assert!(!probe.discarded());
    }
}
