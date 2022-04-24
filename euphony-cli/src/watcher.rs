use crate::manifest::Manifest;
use futures::stream::{Stream, StreamExt};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::channel,
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

static NEXT_SUB_ID: AtomicUsize = AtomicUsize::new(1);

pub type Subscriptions = Arc<Mutex<HashMap<usize, mpsc::Sender<String>>>>;

pub fn create(subs: Subscriptions, mut manifest: Manifest) {
    let _ = manifest.compile();

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

    watcher
        .watch(&manifest.root, RecursiveMode::Recursive)
        .unwrap();

    let mut updates = HashSet::new();

    let target_filter = manifest.root.join("target");
    let target_filter = Path::new(&target_filter);
    let project_filter = manifest.root.join("target/euphony");
    let project_filter = Path::new(&project_filter);

    loop {
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
            let mut handle_event = |event| {
                if let Some(path) = map_event(event) {
                    if path.extension() == Some(OsStr::new("euph")) {
                        updates.insert(path);
                        return;
                    }

                    if path.ends_with("Cargo.toml") {
                        should_rebuild_manifest = true;
                        return;
                    }

                    if path.strip_prefix(target_filter).is_err() {
                        should_compile = true;
                    }
                }
            };

            handle_event(event);

            while let Ok(event) = rx.try_recv() {
                handle_event(event);
            }
        }

        should_compile |= should_rebuild_manifest;

        let mut msg = String::new();
        for path in updates.drain() {
            // notify subscriptions of project updates
            if let Ok(contents) = fs::read_to_string(&path) {
                if msg.is_empty() {
                    msg.push('{');
                } else {
                    msg.push(',');
                }

                let path = path.strip_prefix(&project_filter).unwrap().display();

                msg.push_str(&format!("{:?}", path));
                msg.push(':');
                msg.push_str(&contents);
            }
        }

        if !msg.is_empty() {
            msg.push('}');
            let subs = subs.lock().unwrap();
            for sub in subs.values() {
                let _ = sub.blocking_send(msg.clone());
            }
        }

        if should_rebuild_manifest {
            let _ = manifest.rebuild_manifest();
        }

        if should_compile {
            let _ = manifest.compile();
        }
    }
}

pub struct Subscriber {
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
