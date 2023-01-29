use crate::{instruction::Instruction, Hash, Writer};
use euphony_dsp::nodes::load as load_dsp;
use euphony_graph::Graph;
use euphony_node::{BufferMap, Config, Context, ParameterValue, Value};

pub type Error = euphony_graph::Error<u64>;
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

#[derive(Debug, Default)]
pub struct Renderer {
    graph: Graph<Config>,
    context: Context,
    sample_offset: u64,
}

impl Renderer {
    #[inline]
    pub fn set_buffers(&mut self, buffers: Box<dyn BufferMap>) {
        self.context.buffers = buffers;
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
        Ok(())
    }
}
