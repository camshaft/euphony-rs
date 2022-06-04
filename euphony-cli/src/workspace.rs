use crate::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

mod templates;

#[derive(Debug, StructOpt)]
pub enum Workspace {
    #[structopt(aliases = &["i", "init"])]
    Initialize(Initialize),
    #[structopt(aliases = &["g", "gen"])]
    Generate(Generate),
}

impl Workspace {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Initialize(cmd) => cmd.run(),
            Self::Generate(cmd) => cmd.run(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Initialize {
    path: PathBuf,
}

impl Initialize {
    pub fn run(&self) -> Result<()> {
        self.write("Cargo.toml", templates::WS_CARGO)?;
        self.write("rust-toolchain", "stable\n")?;
        self.write(".gitignore", templates::WS_GITIGNORE)?;
        self.write(".rustfmt.toml", templates::WS_RUSTFMT)?;
        self.write("common/Cargo.toml", templates::COMMON_CARGO)?;
        self.write("common/src/lib.rs", templates::COMMON_LIB)?;
        Ok(())
    }

    fn write(&self, path: &str, tmpl: &str) -> Result<()> {
        let path = self.path.join(path);
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, tmpl.trim_start())?;
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Generate {
    name: String,
}

impl Generate {
    pub fn run(&self) -> Result<()> {
        let root = self.root()?;
        self.write(&root, "src/main.rs", templates::COMP_MAIN)?;
        self.write(&root, "Cargo.toml", templates::COMP_CARGO)?;
        self.add_to_cargo(&root)?;
        Ok(())
    }

    fn write(&self, root: &Path, path: &str, tmpl: &str) -> Result<()> {
        let path = root.join(&self.name).join(path);
        let dir = path.parent().unwrap();
        fs::create_dir_all(&dir)?;
        let dir = dir.canonicalize()?;
        let name = Path::new(&self.name).file_stem().unwrap().to_str().unwrap();
        let mut common_path = String::new();
        for _ in dir.strip_prefix(root)?.components() {
            common_path.push_str("../");
        }
        common_path.push_str("common");
        fs::write(
            path,
            tmpl.trim_start()
                .replace("NAME", name)
                .replace("COMMON_PATH", &common_path),
        )?;
        Ok(())
    }

    fn root(&self) -> Result<PathBuf> {
        let meta = cargo_metadata::MetadataCommand::new().exec()?;
        let path = PathBuf::from(&meta.workspace_root);
        Ok(path)
    }

    fn add_to_cargo(&self, root: &Path) -> Result<()> {
        use toml_edit::Document;
        let path = root.join("Cargo.toml");
        let root = fs::read_to_string(&path)?;
        let mut root = root.parse::<Document>()?;
        let members = root["workspace"]["members"].as_array_mut().unwrap();
        members.push(&self.name);
        members.fmt();
        std::fs::write(path, root.to_string())?;
        Ok(())
    }
}
