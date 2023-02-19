use std::{io, sync::Arc};

macro_rules! error {
    ($fmt:literal $($tt:tt)*) => {
        ::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!($fmt $($tt)*))
    };
}

pub trait Writer: Sync {
    fn is_cached(&self, hash: &Hash) -> bool;
    fn sink(&mut self, hash: &Hash) -> euphony_node::BoxProcessor;
    fn group<I: Iterator<Item = Entry>>(
        &mut self,
        name: &str,
        hash: &Hash,
        entries: I,
        midi: &midi::Writer,
    );
    fn buffer<F: FnOnce(Box<dyn BufferReader>) -> Result<Vec<ConvertedBuffer>, E>, E>(
        &self,
        path: &str,
        sample_rate: u64,
        init: F,
    ) -> Result<Vec<CachedBuffer>, E>;
}

pub trait BufferReader: io::Read + Send + Sync + 'static {}

impl<T: io::Read + Send + Sync + 'static> BufferReader for T {}

pub type ConvertedBuffer = Vec<sample::DefaultSample>;

#[derive(Clone, Debug)]
pub struct CachedBuffer {
    pub samples: Arc<[f64]>,
    pub hash: Hash,
}

#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub sample_offset: u64,
    pub hash: Hash,
}

#[path = "sample.rs"]
mod internal_sample;
pub mod sample {
    pub(crate) use super::internal_sample::*;
    pub use euphony_dsp::sample::*;
}

// TODO better error?
pub type Error = std::io::Error;
pub type Result<T = (), E = Error> = core::result::Result<T, E>;
pub type Hash = [u8; 32];

mod buffer;
mod compiler;
mod group;
mod instruction;
pub mod midi;
mod node;
mod parallel;
mod render;
mod sink;

#[derive(Debug, Default)]
pub struct Compiler {
    compiler: compiler::Compiler,
    render: render::Renderer,
}

impl Compiler {
    pub fn compile<I: io::Read, O: Writer>(&mut self, input: &mut I, output: &mut O) -> Result {
        // clear everything out first
        self.compiler.reset();
        self.render.reset();

        euphony_command::decode(input, &mut self.compiler)?;
        let buffers = self.compiler.finalize(output)?;
        self.render.set_buffers(buffers);

        for instruction in self.compiler.instructions() {
            self.render
                .push(instruction, output)
                .map_err(|err| error!("invalid instruction {:?}", err))?;
        }

        for (_id, group, entries) in self.compiler.groups() {
            output.group(&group.name, &group.hash, entries, &group.midi);
        }

        Ok(())
    }

    pub fn display<I: io::Read, O: io::Write>(&mut self, input: &mut I, output: &mut O) -> Result {
        // clear everything out first
        self.compiler.reset();
        self.render.reset();

        euphony_command::decode(input, &mut self.compiler)?;

        struct Output;

        impl Writer for Output {
            fn is_cached(&self, _hash: &Hash) -> bool {
                false
            }

            fn sink(&mut self, _hash: &Hash) -> euphony_node::BoxProcessor {
                unimplemented!()
            }

            fn group<I: Iterator<Item = Entry>>(
                &mut self,
                _name: &str,
                _hash: &Hash,
                _entries: I,
                _midi: &midi::Writer,
            ) {
            }

            fn buffer<F: FnOnce(Box<dyn BufferReader>) -> Result<Vec<ConvertedBuffer>, E>, E>(
                &self,
                _path: &str,
                _sample_rate: u64,
                _init: F,
            ) -> Result<Vec<CachedBuffer>, E> {
                unimplemented!()
            }
        }

        self.compiler.finalize(&Output)?;

        writeln!(output, "# Groups")?;
        for (_id, group, entries) in self.compiler.groups() {
            let count = entries.count();
            writeln!(output, "* {:?} ({} entries)", group.name, count)?;
        }
        writeln!(output)?;

        writeln!(output, "# Instructions")?;
        for instruction in self.compiler.instructions() {
            writeln!(output, "{}", instruction)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bolero::check;
    use std::io::Cursor;

    struct Output;

    impl Writer for Output {
        fn is_cached(&self, _hash: &Hash) -> bool {
            false
        }

        fn sink(&mut self, _hash: &Hash) -> euphony_node::BoxProcessor {
            // silence
            euphony_dsp::nodes::load(106).unwrap()
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

    #[test]
    #[ignore] // this is currently broken
    fn fuzz() {
        check!().for_each(|input| {
            let mut compiler = Compiler::default();

            {
                let mut out = String::new();
                if euphony_command::decode(&mut Cursor::new(input), &mut out).is_ok() {
                    eprintln!("=====\n{}\n=====", out);
                }
            }

            let mut input = Cursor::new(input);
            let _ = compiler.compile(&mut input, &mut Output);
        });
    }
}
