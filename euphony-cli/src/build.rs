use crate::{compiler::Compiler, manifest::Manifest, Result};
use std::{fs, io, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, Default, StructOpt)]
pub struct Build {
    input: Option<PathBuf>,
}

impl Build {
    pub fn run(&self) -> Result<()> {
        self.build()?;
        Ok(())
    }

    pub fn build(&self) -> Result<Vec<Compiler>> {
        let out_dir = PathBuf::from("target/euphony");
        let contents = PathBuf::from("target/euphony/contents");

        if let Some(input) = self.input.as_ref() {
            if input.is_dir() {
                let manifest_path = input.join("Cargo.toml");
                let mut manifest = Manifest::new(Some(&manifest_path), None)?;
                manifest.compile()?;
                return Ok(manifest.finish());
            }

            match input.file_name().and_then(|v| v.to_str()) {
                Some("-") => {
                    let timeline = out_dir.join("main.json");
                    let mut comp = Compiler::new(contents, timeline);
                    let mut input = io::stdin();
                    comp.render(&mut input)?;
                    Ok(vec![comp])
                }
                Some("Cargo.toml") => {
                    let mut manifest = Manifest::new(Some(input), None)?;
                    manifest.compile()?;
                    Ok(manifest.finish())
                }
                Some(name) => {
                    let timeline = out_dir.join(name).with_extension("json");
                    let mut comp = Compiler::new(contents, timeline);
                    let input = fs::File::open(input)?;
                    let mut input = io::BufReader::new(input);
                    comp.render(&mut input)?;
                    Ok(vec![comp])
                }
                None => {
                    let timeline = out_dir.join("main.json");
                    let mut comp = Compiler::new(contents, timeline);
                    let input = fs::File::open(input)?;
                    let mut input = io::BufReader::new(input);
                    comp.render(&mut input)?;
                    Ok(vec![comp])
                }
            }
        } else {
            let mut manifest = Manifest::new(None, None)?;
            manifest.compile()?;
            Ok(manifest.finish())
        }
    }
}
