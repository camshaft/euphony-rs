use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::{
    io,
    path::{Path, PathBuf},
};

mod buffer;
mod manifest;
mod synth;

/*
pub fn build_synths() -> io::Result<()> {
    let compile = concat!(env!("CARGO_MANIFEST_DIR"), "/src/compile.scd");
    assert!(Path::new(compile).exists());
    println!("cargo:rerun-if-changed={}", compile);

    let input = Path::new("synths").canonicalize()?;
    println!("cargo:rerun-if-changed={}", input.display());

    let output = env::var("OUT_DIR").expect("missing out dir");
    let output = Path::new(&output).canonicalize()?;

    let status = Command::new("sclang")
        .arg(compile)
        .env("IN_DIR", input)
        .env("OUT_DIR", output)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to compile synths",
        ));
    }

    Ok(())
}
*/

#[derive(Default)]
pub struct Builder {
    assets: Vec<Asset>,
}

struct Asset {
    path: String,
    name: String,
    compile: fn(&Ident, &str, &Path, &Path) -> io::Result<()>,
}

impl Asset {
    pub fn compile(&self, root: &Path, out_dir: &Path) -> io::Result<TokenStream> {
        let src = resolve_path(&self.path, root, out_dir)?;

        let mut module = src.clone();
        module.set_extension("rs");

        let src_t = std::fs::metadata(&src)?.modified()?;
        let module_meta = std::fs::metadata(&module).and_then(|m| m.modified()).ok();

        if module_meta.map_or(true, |module_t| src_t > module_t) {
            let name = Ident::new(&self.name, Span::call_site());
            (self.compile)(&name, &self.path, &src, &module)?;
        }

        let module_str = format!("/assets/{}", module.file_name().unwrap().to_str().unwrap());
        let inc = quote!(include!(concat!(env!("OUT_DIR"), #module_str)););

        Ok(inc)
    }
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn synth<N: core::fmt::Display, P: core::fmt::Display>(
        &mut self,
        name: N,
        path: P,
    ) -> &mut Self {
        self.assets.push(Asset {
            path: path.to_string(),
            name: name.to_string(),
            compile: synth::compile,
        });
        self
    }

    pub fn buffer<N: core::fmt::Display, P: core::fmt::Display>(
        &mut self,
        name: N,
        path: P,
    ) -> &mut Self {
        self.assets.push(Asset {
            path: path.to_string(),
            name: name.to_string(),
            compile: buffer::compile,
        });
        self
    }

    pub fn build(&mut self) -> io::Result<()> {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let root = Path::new(&root);

        let assets_manifest = root.join("euphony.toml");
        if let Ok(contents) = std::fs::read(&assets_manifest) {
            println!("cargo:rerun-if-changed={}", assets_manifest.display());
            let manifest: manifest::Manifest = toml::from_slice(&contents)?;
            for (name, synth) in manifest.synths.iter() {
                // TODO config
                self.synth(name, synth.url());
            }

            for (name, sample) in manifest.samples.iter() {
                // TODO config
                self.buffer(name, sample.url());
            }
        }

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let out_dir = Path::new(&out_dir);
        let modules = self
            .assets
            .iter()
            .map(|asset| asset.compile(&root, &out_dir))
            .collect::<io::Result<Vec<TokenStream>>>()?;

        let out = quote!(pub mod assets {
            #![allow(warnings)]
            use euphony::euphony_sc::{
                self,
                buffer::{BufDef, BufDefGroup},
                _macro_support::{InlineFile, ExternalFile}
            };

            #(#modules)*
        });
        let output = out_dir.join("euphony_assets.rs");
        std::fs::write(output, out.to_string())?;

        println!("cargo:rustc-cfg=euphony_assets");

        Ok(())
    }
}

fn resolve_path(url: &str, _root: &Path, out_dir: &Path) -> std::io::Result<PathBuf> {
    let url = if let Some(path) = url.strip_prefix("gh://") {
        let mut parts = path.split('/');
        let mut out = url::Url::parse("https://raw.githubusercontent.com").unwrap();
        let mut segments = out.path_segments_mut().unwrap();
        segments.push(parts.next().unwrap());
        segments.push(parts.next().unwrap());

        segments.push(if let Some(version) = path.split('?').nth(1) {
            version
        } else {
            "main"
        });

        segments.extend(parts);

        drop(segments);

        out
    } else if let Some(path) = url.strip_prefix("clean://") {
        let mut parts = path.split('/');
        let mut out = url::Url::parse("https://raw.githubusercontent.com").unwrap();
        let mut segments = out.path_segments_mut().unwrap();
        segments.push(parts.next().unwrap());
        segments.push(parts.next().unwrap());

        segments.push(if let Some(version) = path.split('?').nth(1) {
            version
        } else {
            "main"
        });

        segments.push("_soundmeta");
        segments.push(&format!("{}.json", parts.next().unwrap()));

        drop(segments);

        out
    } else if url.starts_with("https://") || url.starts_with("http://") {
        url::Url::parse(url).unwrap()
    } else {
        unimplemented!();
    };

    match url.scheme() {
        "https" | "http" => {
            let path = download(url.as_str(), &out_dir.join("assets"))?;
            Ok(path)
        }
        _ => unimplemented!(),
    }
}

pub(crate) fn download(url: &str, out_dir: &Path) -> std::io::Result<PathBuf> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url);
    let filename = base64::encode_config(hasher.finalize(), base64::URL_SAFE_NO_PAD);

    let path = out_dir.join(filename);

    fn convert_err<E: core::fmt::Display>(err: E) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }

    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;

        let mut response = reqwest::blocking::get(url)
            .map_err(convert_err)?
            .error_for_status()
            .map_err(convert_err)?;
        let mut file = std::fs::File::create(&path)?;
        let mut file = std::io::BufWriter::new(&mut file);
        response.copy_to(&mut file).map_err(convert_err)?;
    }

    Ok(path)
}
