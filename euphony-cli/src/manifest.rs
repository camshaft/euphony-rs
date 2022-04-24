use crate::{compiler::Compiler, Result};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug)]
pub struct Manifest {
    pub root: PathBuf,
    pub out_dir: PathBuf,
    pub projects: HashMap<String, Compiler>,
}

impl Manifest {
    pub fn new(manifest_path: Option<&Path>, out_dir: Option<&Path>) -> Result<Self> {
        let mut projects = HashMap::new();

        let root = Self::build_manifest(manifest_path, out_dir, &mut projects)?;
        let out_dir = out_dir.unwrap_or(&root).to_owned();

        let comp = Self {
            root,
            out_dir,
            projects,
        };
        Ok(comp)
    }

    pub fn rebuild_manifest(&mut self) -> Result<()> {
        let manifest_path = self.root.join("Cargo.toml");
        Self::build_manifest(
            Some(&manifest_path),
            Some(&self.out_dir),
            &mut self.projects,
        )?;
        Ok(())
    }

    fn build_manifest(
        manifest_path: Option<&Path>,
        out_dir: Option<&Path>,
        projects: &mut HashMap<String, Compiler>,
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
                    let (contents, timeline) = if let Some(out_dir) = out_dir {
                        (
                            out_dir.join("contents"),
                            out_dir.join(format!("{}.euph", package.name)),
                        )
                    } else {
                        (
                            root.join("target/euphony/contents"),
                            root.join(format!("target/euphony/{}.euph", package.name)),
                        )
                    };
                    let project = Compiler::new(contents, timeline);
                    projects.insert(package.name.clone(), project);
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

                project.render(&mut stdout)?;

                Ok(())
            })
            .collect();

        res?;

        Ok(())
    }
}
