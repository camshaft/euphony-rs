use crate::Result;
use euphony_compiler::Compiler;
use euphony_store::{storage::fs::Directory, Store};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug)]
pub struct Manifest {
    pub root: PathBuf,
    pub projects: HashMap<String, Project>,
}

#[derive(Debug)]
pub struct Project {
    pub compiler: Compiler,
    pub store: Store,
}

impl Project {
    fn new(root: &Path) -> Self {
        let mut store: Store<Directory<_>, _> = Default::default();
        store.storage.path = root.join("target/euphony/contents");
        let _ = fs::create_dir_all(&store.storage.path);
        Self {
            compiler: Default::default(),
            store,
        }
    }
}

impl Manifest {
    pub fn new(manifest_path: Option<&Path>) -> Result<Self> {
        let mut projects = HashMap::new();

        let root = Self::build_manifest(manifest_path, &mut projects)?;

        let comp = Self { root, projects };
        Ok(comp)
    }

    pub fn rebuild_manifest(&mut self) -> Result<()> {
        let manifest_path = self.root.join("Cargo.toml");
        Self::build_manifest(Some(&manifest_path), &mut self.projects)?;
        Ok(())
    }

    fn build_manifest(
        manifest_path: Option<&Path>,
        projects: &mut HashMap<String, Project>,
    ) -> Result<PathBuf> {
        let mut cmd = cargo_metadata::MetadataCommand::new();
        if let Some(manifest_path) = manifest_path {
            cmd.manifest_path(manifest_path);
        }
        let meta = cmd.exec()?;

        let root: PathBuf = (&meta.workspace_root).into();

        projects.clear();

        for id in meta.workspace_members.iter() {
            let package = &meta[id];
            for target in package.targets.iter() {
                if target.kind.iter().any(|k| k == "bin")
                    && package.dependencies.iter().any(|dep| dep.name == "euphony")
                {
                    projects.insert(package.name.clone(), Project::new(&root));
                }
            }
        }

        Ok(root)
    }

    pub fn compile(&mut self) -> Result<()> {
        let root = &self.root;

        let status = std::process::Command::new("cargo")
            .arg("build")
            .arg("--target-dir")
            .arg("target/euphony/build")
            .current_dir(root)
            .spawn()?
            .wait()?;

        if !status.success() {
            eprintln!("cargo build failed");
            return Err(anyhow::anyhow!("build command failed"));
        }

        let res: Result<()> = self
            .projects
            .par_iter_mut()
            .map(|(name, project)| {
                let mut proc =
                    process::Command::new(format!("target/euphony/build/debug/{}", name));

                proc.stdout(process::Stdio::piped())
                    .env("EUPHONY_OUTPUT", "-")
                    .current_dir(root);

                let proc = proc.spawn()?;

                let mut stdout = io::BufReader::new(proc.stdout.unwrap());

                project.compiler.compile(&mut stdout, &mut project.store)?;

                let timeline = root.join(format!("target/euphony/projects/{}.json", name));

                fs::create_dir_all(timeline.parent().unwrap())?;

                let timeline = fs::File::create(timeline)?;
                let mut timeline = io::BufWriter::new(timeline);
                project.store.timeline.to_json(&mut timeline)?;
                io::Write::flush(&mut timeline)?;

                Ok(())
            })
            .collect();

        res?;

        Ok(())
    }
}
