use crate::Result;
use euphony_compiler::Compiler;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Disasm {
    #[structopt(short, long)]
    instructions: bool,

    inputs: Vec<PathBuf>,
}

impl Disasm {
    pub fn run(&self) -> Result<()> {
        if self.inputs.is_empty() {
            return self.display(Path::new("-"));
        }

        if self.inputs.len() == 1 {
            return self.display(&self.inputs[0]);
        }

        for input in &self.inputs {
            println!("====\n{}\n----", input.display());
            if let Err(err) = self.display(input) {
                println!("ERROR: {}", err);
            }
            println!();
        }

        Ok(())
    }

    fn display(&self, path: &Path) -> Result<()> {
        let mut input: Box<dyn io::Read> = if path.to_str() == Some("-") {
            Box::new(io::stdin())
        } else {
            let file = fs::File::open(path)?;
            let file = io::BufReader::new(file);
            Box::new(file)
        };

        if self.instructions {
            let mut compiler = Compiler::default();
            compiler.display(&mut input, &mut io::stdout())?;
        } else {
            let mut output = String::new();
            euphony_command::decode(&mut input, &mut output)?;
            print!("{}", output);
        }

        Ok(())
    }
}
