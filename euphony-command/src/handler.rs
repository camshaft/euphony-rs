use super::*;
use std::io;

pub trait Handler {
    fn advance_time(&mut self, msg: AdvanceTime) -> io::Result<()>;
    fn set_timing(&mut self, msg: SetTiming) -> io::Result<()>;
    fn create_group(&mut self, msg: CreateGroup) -> io::Result<()>;
    fn spawn_node(&mut self, msg: SpawnNode) -> io::Result<()>;
    fn fork_node(&mut self, msg: ForkNode) -> io::Result<()>;
    fn emit_midi(&mut self, msg: EmitMidi) -> io::Result<()>;
    fn set_parameter(&mut self, msg: SetParameter) -> io::Result<()>;
    fn pipe_parameter(&mut self, msg: PipeParameter) -> io::Result<()>;
    fn finish_node(&mut self, msg: FinishNode) -> io::Result<()>;
    fn init_buffer(&mut self, msg: InitBuffer) -> io::Result<()>;
    fn load_buffer(&mut self, msg: LoadBuffer) -> io::Result<()>;
    fn set_buffer(&mut self, msg: SetBuffer) -> io::Result<()>;
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn finish(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn push_msg<T: fmt::Display>(output: &mut String, v: T) -> io::Result<()> {
    use std::fmt::Write;
    let _ = writeln!(output, "{v}");
    Ok(())
}

impl Handler for String {
    fn advance_time(&mut self, msg: AdvanceTime) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn set_timing(&mut self, msg: SetTiming) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn create_group(&mut self, msg: CreateGroup) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn spawn_node(&mut self, msg: SpawnNode) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn fork_node(&mut self, msg: ForkNode) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn emit_midi(&mut self, msg: EmitMidi) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn set_parameter(&mut self, msg: SetParameter) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn pipe_parameter(&mut self, msg: PipeParameter) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn finish_node(&mut self, msg: FinishNode) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn init_buffer(&mut self, msg: InitBuffer) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn load_buffer(&mut self, msg: LoadBuffer) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn set_buffer(&mut self, msg: SetBuffer) -> io::Result<()> {
        push_msg(self, msg)
    }
}

pub struct Writer<O: io::Write>(pub O);

impl<O: io::Write> super::Handler for Writer<O> {
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

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    fn finish(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
