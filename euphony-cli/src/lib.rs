use anyhow::Result;
use futures::stream::{Stream, StreamExt};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use structopt::StructOpt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use warp::Filter;

static NEXT_SUB_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, StructOpt)]
enum Arguments {
    Serve(Serve),
}

#[derive(Debug, StructOpt)]
struct Serve {
    #[structopt(long, short, default_value = "3000")]
    port: u16,

    #[structopt(long)]
    manifest_path: Option<PathBuf>,
}

type Subscriptions = Arc<Mutex<HashMap<usize, mpsc::Sender<String>>>>;

#[tokio::main]
pub async fn main() {
    let args = Arguments::from_args();
    match args {
        Arguments::Serve(args) => serve(args).await.unwrap(),
    }
}

async fn serve(serve: Serve) -> Result<()> {
    let subscriptions = Subscriptions::default();

    let filter_subs = subscriptions.clone();
    let subs_filter = warp::any().map(move || filter_subs.clone());

    let project = warp::path("_updates")
        .and(warp::get())
        .and(subs_filter)
        .map(|subs| warp::sse::reply(warp::sse::keep_alive().stream(Subscriber::new(subs))));

    let compiler = Compiler::new(serve.manifest_path.as_deref())?;

    let files = warp::path("euphony").and(warp::fs::dir(compiler.root.join("target/euphony/")));

    let routes = files
        .or(project)
        .with(warp::cors().allow_any_origin().allow_method("GET"));

    std::thread::spawn(move || watcher::create(subscriptions, compiler));

    warp::serve(routes).run(([0, 0, 0, 0], serve.port)).await;

    Ok(())
}

struct Subscriber {
    subs: Subscriptions,
    recv: ReceiverStream<String>,
    id: usize,
}

impl Subscriber {
    pub fn new(subs: Subscriptions) -> Self {
        let (send, recv) = mpsc::channel(10);

        let id = NEXT_SUB_ID.fetch_add(1, Ordering::SeqCst);
        subs.lock().unwrap().insert(id, send);

        let recv = ReceiverStream::new(recv);

        Self { subs, recv, id }
    }
}

impl Stream for Subscriber {
    type Item = Result<warp::sse::Event, core::convert::Infallible>;

    fn poll_next(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        if let Some(path) = futures::ready!(self.recv.poll_next_unpin(cx)) {
            let event = warp::sse::Event::default().data(path);
            Some(Ok(event)).into()
        } else {
            None.into()
        }
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        self.subs.lock().unwrap().remove(&self.id);
    }
}

mod watcher {
    use super::*;
    use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
    use std::{collections::HashSet, sync::mpsc::channel, time::Duration};

    pub(crate) fn create(subs: Subscriptions, mut compiler: Compiler) {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

        watcher
            .watch(&compiler.root, RecursiveMode::Recursive)
            .unwrap();

        loop {
            let mut updates = HashSet::new();

            fn map_event(event: DebouncedEvent) -> Option<PathBuf> {
                use DebouncedEvent::*;
                match event {
                    Create(path) | Write(path) | Chmod(path) | Rename(_, path) => Some(path),
                    _ => None,
                }
            }

            let mut should_compile = false;
            let mut should_rebuild_manifest = false;

            if let Ok(event) = rx.recv() {
                if let Some(path) = map_event(event) {
                    should_rebuild_manifest |= path.ends_with("Cargo.toml");
                    if !path.components().any(|c| c.as_os_str() == "target") {
                        should_compile = true;
                    }

                    if path.ends_with("project.json") {
                        updates.insert(path);
                    }
                }

                while let Ok(event) = rx.try_recv() {
                    if let Some(path) = map_event(event) {
                        should_rebuild_manifest |= path.ends_with("Cargo.toml");
                        if !path.components().any(|c| c.as_os_str() == "target") {
                            should_compile = true;
                        }

                        if path.ends_with("project.json") {
                            updates.insert(path);
                        }
                    }
                }
            }

            should_compile |= should_rebuild_manifest;

            for update in updates.drain() {
                // notify websockets of project updates
                let path = update
                    .strip_prefix(compiler.root.join("target"))
                    .unwrap()
                    .display()
                    .to_string();
                let subs = subs.lock().unwrap();
                for sub in subs.values() {
                    let _ = sub.blocking_send(path.clone());
                }
            }

            if should_rebuild_manifest {
                let _ = compiler.rebuild_manifest();
            }

            if should_compile {
                let _ = compiler.compile();
            }
        }
    }
}

#[derive(Debug)]
struct Compiler {
    pub root: PathBuf,
    pub projects: HashSet<String>,
    pub multitrack: bool,
}

impl Compiler {
    pub fn new(manifest_path: Option<&Path>) -> Result<Self> {
        let mut projects = HashSet::new();

        let root = Self::build_manifest(manifest_path, &mut projects)?;

        let comp = Self {
            root,
            projects,
            multitrack: true,
        };
        Ok(comp)
    }

    pub fn rebuild_manifest(&mut self) -> Result<()> {
        let manifest_path = self.root.join("Cargo.toml");
        Self::build_manifest(Some(&manifest_path), &mut self.projects)?;
        Ok(())
    }

    fn build_manifest(
        manifest_path: Option<&Path>,
        projects: &mut HashSet<String>,
    ) -> Result<PathBuf> {
        let mut cmd = cargo_metadata::MetadataCommand::new();
        if let Some(manifest_path) = manifest_path {
            cmd.manifest_path(manifest_path);
        }
        let meta = cmd.exec()?;

        projects.clear();

        for id in meta.workspace_members.iter() {
            let package = &meta[id];
            for target in package.targets.iter() {
                if target.kind.iter().any(|k| k == "bin")
                    && package.dependencies.iter().any(|dep| dep.name == "euphony")
                {
                    projects.insert(package.name.clone());
                }
            }
        }

        Ok(meta.workspace_root.into())
    }

    pub fn compile(&self) -> Result<()> {
        let status = std::process::Command::new("cargo")
            .arg("build")
            .current_dir(&self.root)
            .spawn()?
            .wait()?;

        if !status.success() {
            eprintln!("cargo build failed");
            return Err(anyhow::anyhow!("build command failed"));
        }

        for project in &self.projects {
            let mut proc = std::process::Command::new(format!("target/debug/{}", project));
            proc.arg("render");
            if self.multitrack {
                proc.arg("--multitrack");
            }

            let status = proc.current_dir(&self.root).spawn()?.wait()?;

            if !status.success() {
                eprintln!("{:?} failed", project);
            }
        }

        Ok(())
    }
}
