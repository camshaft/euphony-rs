use crate::*;
use std::{fs, io, path::Path, time::Duration};

bach::scope::define!(scope, Box<dyn super::Handler>);

struct WriterOut<O: io::Write>(O);

impl<O: io::Write> super::Handler for WriterOut<O> {
    fn advance_time(&mut self, msg: AdvanceTime) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn set_timing(&mut self, msg: SetTiming) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn create_group(&mut self, msg: CreateGroup) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn spawn_node(&mut self, msg: SpawnNode) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn fork_node(&mut self, msg: ForkNode) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn emit_midi(&mut self, msg: EmitMidi) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn set_parameter(&mut self, msg: SetParameter) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn pipe_parameter(&mut self, msg: PipeParameter) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn finish_node(&mut self, msg: FinishNode) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn init_buffer(&mut self, msg: InitBuffer) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn load_buffer(&mut self, msg: LoadBuffer) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn set_buffer(&mut self, msg: SetBuffer) -> io::Result<()> {
        msg.encode(&mut self.0)
    }

    fn finish(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

pub fn set_file(path: &Path) {
    let file = fs::File::create(path).unwrap();
    let file = io::BufWriter::new(file);
    set_writer(file)
}

pub fn set_stdout() {
    set_writer(io::stdout())
}

pub fn set_writer<W: io::Write + 'static>(out: W) {
    let out = WriterOut(out);
    let out = Box::new(out);
    scope::set(Some(out));
}

macro_rules! emit {
    ($method:ident($msg:expr)) => {
        scope::try_borrow_mut_with(|output| {
            let msg = $msg;
            if let Some(output) = output.as_mut() {
                output
                    .$method(msg)
                    .expect("failed to emit message to output");
            } else {
                println!("{msg}");
            }
        })
    };
}

pub fn advance_time(ticks: u64) {
    emit!(advance_time(AdvanceTime { ticks }))
}

pub fn set_timing(nanos_per_tick: Duration, ticks_per_beat: u64) {
    emit!(set_timing(SetTiming {
        nanos_per_tick: nanos_per_tick.as_nanos() as _,
        ticks_per_beat,
    }))
}

pub fn create_group(id: u64, name: String) {
    emit!(create_group(CreateGroup { id, name }))
}

pub fn spawn_node(id: u64, processor: u64, group: Option<u64>) {
    emit!(spawn_node(SpawnNode {
        id,
        group,
        processor,
    }))
}

pub fn fork_node(source: u64, target: u64) {
    emit!(fork_node(ForkNode { source, target }))
}

pub fn emit_midi(data: [u8; 3], group: Option<u64>) {
    emit!(emit_midi(EmitMidi { data, group }))
}

pub fn set_parameter(target_node: u64, target_parameter: u64, value: f64) {
    emit!(set_parameter(SetParameter {
        target_node,
        target_parameter,
        value: value.to_bits(),
    }))
}

pub fn pipe_parameter(target_node: u64, target_parameter: u64, source_node: u64) {
    emit!(pipe_parameter(PipeParameter {
        source_node,
        target_node,
        target_parameter,
    }))
}

pub fn finish_node(id: u64) {
    emit!(finish_node(FinishNode { node: id }))
}

pub fn init_buffer(source: &Path, meta: &Path) {
    let source = source.to_string_lossy().to_string();
    let meta = meta.to_string_lossy().to_string();
    emit!(init_buffer(InitBuffer { source, meta }))
}

pub fn load_buffer(id: u64, path: &Path, ext: &str) {
    let path = path.to_string_lossy().to_string();
    let ext = ext.to_string();
    emit!(load_buffer(LoadBuffer { id, path, ext }))
}

pub fn set_buffer(target_node: u64, target_parameter: u64, buffer: u64, buffer_channel: u64) {
    emit!(set_buffer(SetBuffer {
        target_node,
        target_parameter,
        buffer,
        buffer_channel,
    }))
}

pub fn flush() {
    scope::try_borrow_mut_with(|output| {
        if let Some(output) = output.as_mut() {
            output.finish().unwrap();
        }
    });
}

pub fn finish() {
    flush();
}
