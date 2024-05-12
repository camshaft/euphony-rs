use crate::*;
use std::{fs, io, path::Path, time::Duration};

bach::scope::define!(scope, Box<dyn super::Handler>);

pub fn set_file(path: &Path) {
    let file = fs::File::create(path).unwrap();
    let file = io::BufWriter::new(file);
    set_writer(file)
}

pub fn set_stdout() {
    set_writer(io::stdout())
}

pub fn set_writer<W: io::Write + 'static>(out: W) {
    let out = handler::Writer(out);
    let out = Box::new(out);
    scope::set(Some(out));
}

macro_rules! emit {
    ($method:ident(| $($arg:ident : $arg_t:ty),* $(,)? | $msg:expr)) => {
        pub fn $method($($arg: $arg_t),*) {
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
        }
    };
}

emit!(advance_time(|ticks: u64| AdvanceTime { ticks }));

emit!(set_timing(
    |nanos_per_tick: Duration, ticks_per_beat: u64| SetTiming {
        nanos_per_tick: nanos_per_tick.as_nanos() as _,
        ticks_per_beat,
    }
));

emit!(create_group(|id: u64, name: &str| CreateGroup {
    id,
    name: name.to_string(),
}));

emit!(spawn_node(|id: u64, processor: u64, group: Option<u64>| {
    SpawnNode {
        id,
        group,
        processor,
    }
}));

emit!(fork_node(|source: u64, target: u64| ForkNode {
    source,
    target
}));

emit!(emit_midi(|data: [u8; 3], group: Option<u64>| EmitMidi {
    data,
    group
}));

emit!(set_parameter(
    |target_node: u64, target_parameter: u64, value: f64| {
        SetParameter {
            target_node,
            target_parameter,
            value: value.to_bits(),
        }
    }
));

emit!(pipe_parameter(
    |target_node: u64, target_parameter: u64, source_node: u64| PipeParameter {
        source_node,
        target_node,
        target_parameter,
    }
));

emit!(finish_node(|id: u64| FinishNode { node: id }));

emit!(init_buffer(|source: &Path, meta: &Path| {
    let source = source.to_string_lossy().to_string();
    let meta = meta.to_string_lossy().to_string();
    InitBuffer { source, meta }
}));

emit!(load_buffer(|id: u64, path: &Path, ext: &str| {
    let path = path.to_string_lossy().to_string();
    let ext = ext.to_string();
    LoadBuffer { id, path, ext }
}));

emit!(set_buffer(
    |target_node: u64, target_parameter: u64, buffer: u64, buffer_channel: u64| {
        SetBuffer {
            target_node,
            target_parameter,
            buffer,
            buffer_channel,
        }
    }
));

pub fn flush() {
    scope::try_borrow_mut_with(|output| {
        if let Some(output) = output.as_mut() {
            output.flush().unwrap();
        }
    });
}

pub fn finish() {
    scope::try_borrow_mut_with(|output| {
        if let Some(output) = output.as_mut() {
            output.finish().unwrap();
        }
    });
}
