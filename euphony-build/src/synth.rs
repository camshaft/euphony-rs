use euphony_sc_core::codegen::include_synthdef;
use proc_macro2::Ident;
use std::{io, path::Path};

pub fn compile(name: &Ident, _url: &str, input: &Path, output: &Path) -> io::Result<()> {
    let out = include_synthdef(input, name.span());

    let def = format!(
        "pub mod {name} {{ use super::*;\n {out} }} pub use {name}::new as {name};",
        name = name,
        out = out
    );
    std::fs::write(output, def)?;

    let _ = std::process::Command::new("rustfmt").arg(output).status();

    Ok(())
}
