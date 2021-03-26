use std::{io, path::Path, process::Command};

pub struct Render<'a> {
    pub commands: &'a Path,
    pub input: Option<&'a Path>,
    pub output: &'a Path,
    pub channels: u16,
}

impl<'a> Render<'a> {
    pub fn render(&self) -> Result<(), io::Error> {
        let mut command = Command::new("scsynth");

        command
            .arg("-N")
            .arg(self.commands)
            .arg(self.input.unwrap_or_else(|| Path::new("_")))
            .arg(self.output)
            .arg("48000")
            .arg("WAV")
            .arg("int24")
            .arg("-o")
            .arg(self.channels.to_string());

        let output = command.output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                String::from_utf8_lossy(&output.stderr),
            ));
        }

        Ok(())
    }
}
