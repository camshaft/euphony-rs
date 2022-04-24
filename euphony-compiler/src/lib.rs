use std::io;

macro_rules! error {
    ($fmt:literal $($tt:tt)*) => {
        ::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!($fmt $($tt)*))
    };
}

pub trait Writer: Sync {
    fn is_cached(&self, hash: &Hash) -> bool;
    fn sink(&mut self, hash: &Hash) -> euphony_node::BoxProcessor;
    fn group<I: Iterator<Item = Entry>>(&mut self, name: &str, hash: &Hash, entries: I);
}

#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub sample_offset: u64,
    pub hash: Hash,
}

pub use euphony_dsp::sample::{
    Default as Sample, DefaultRate as DefaultSampleRate, Rate as SampleRate,
};

// TODO better error?
pub type Error = std::io::Error;
pub type Result<T = (), E = Error> = core::result::Result<T, E>;
pub type Hash = [u8; 32];

mod compiler;
mod group;
mod instruction;
mod node;
mod parallel;
mod render;
mod sample;
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
        self.compiler.finalize(output)?;

        for instruction in self.compiler.instructions() {
            self.render
                .push(instruction, output)
                .map_err(|err| error!("invalid instruction {:?}", err))?;
        }

        for (_id, group, entries) in self.compiler.groups() {
            output.group(&group.name, &group.hash, entries);
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

            fn group<I: Iterator<Item = Entry>>(&mut self, _name: &str, _hash: &Hash, _entries: I) {
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

        fn group<I: Iterator<Item = Entry>>(&mut self, _name: &str, _hash: &Hash, entries: I) {
            for _ in entries {}
        }
    }

    #[test]
    fn fuzz() {
        check!().for_each(|input| {
            let mut compiler = Compiler::default();
            let mut input = Cursor::new(input);
            let _ = compiler.compile(&mut input, &mut Output);
        });
    }
}
