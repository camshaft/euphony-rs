#![cfg_attr(not(feature = "play"), allow(dead_code))]

use crate::{compiler::Compiler, Result};
use anyhow::anyhow;
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug)]
pub struct Manifest {
    pub root: PathBuf,
    pub out_dir: PathBuf,
    pub projects: BTreeMap<String, Compiler>,
    pub project: Option<String>,
}

impl Manifest {
    pub fn new(manifest_path: Option<&Path>, out_dir: Option<&Path>) -> Result<Self> {
        let mut projects = Default::default();

        let root = Self::build_manifest(manifest_path, out_dir, &mut projects)?;
        let out_dir = out_dir.unwrap_or(&root).to_owned();

        let comp = Self {
            root,
            out_dir,
            projects,
            project: None,
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

    pub fn default_project(&self) -> Option<&str> {
        if let Some(p) = self.project.as_ref() {
            return Some(p);
        }
        self.projects.keys().next().map(|v| v.as_str())
    }

    pub fn set_project<T: Into<String>>(&mut self, project: T) -> Result<()> {
        let project = project.into();
        if !self.projects.contains_key(&project) {
            return Err(anyhow!("unknown project: {:?}", project));
        }

        self.project = Some(project);

        Ok(())
    }

    pub fn project(&self) -> Result<&Compiler> {
        let project = self
            .project
            .as_ref()
            .ok_or_else(|| anyhow!("no project set"))?;
        let project = &self.projects[project];
        Ok(project)
    }

    fn build_manifest(
        manifest_path: Option<&Path>,
        out_dir: Option<&Path>,
        projects: &mut BTreeMap<String, Compiler>,
    ) -> Result<PathBuf> {
        let mut cmd = cargo_metadata::MetadataCommand::new();
        if let Some(manifest_path) = manifest_path {
            if manifest_path.is_dir() {
                cmd.manifest_path(manifest_path.join("Cargo.toml"));
            } else {
                cmd.manifest_path(manifest_path);
            }
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

        let mut build = std::process::Command::new("cargo");

        build
            .arg("build")
            .arg("--target-dir")
            .arg("target/euphony/build")
            .current_dir(root);

        if let Some(project) = self.project.as_ref() {
            build.arg("--package").arg(project);
        }

        crate::logger::cmd(&mut build);
        let mut child = build.spawn()?;
        crate::logger::cmd_stderr(child.stderr.take());
        let status = child.wait()?;

        if !status.success() {
            return Err(anyhow::anyhow!("cargo build command failed"));
        }

        fn render(root: &Path, name: &str, compiler: &mut Compiler) -> Result<()> {
            let mut proc = process::Command::new(format!("target/euphony/build/debug/{}", name));

            proc.stdout(process::Stdio::piped())
                .env("EUPHONY_OUTPUT", "-")
                .current_dir(root);

            crate::logger::cmd(&mut proc);

            let mut proc = proc.spawn()?;
            crate::logger::cmd_stderr(proc.stderr.take());
            let mut stdout = io::BufReader::new(proc.stdout.unwrap());

            compiler.render(&mut stdout)?;

            Ok(())
        }

        if let Some(project) = self.project.as_ref() {
            let compiler = self.projects.get_mut(project).unwrap();
            return render(root, project, compiler);
        }

        let res: Result<()> = self
            .projects
            .par_iter_mut()
            .map(|(name, compiler)| render(root, name, compiler))
            .collect();

        res?;

        Ok(())
    }

    pub fn watch<S: 'static + Send + crate::watcher::Subscriptions<Self>>(
        self,
        subscriptions: S,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || crate::watcher::watch_manifest(subscriptions, self))
    }

    pub fn finish(self) -> Vec<Compiler> {
        self.projects.into_values().collect()
    }
}
