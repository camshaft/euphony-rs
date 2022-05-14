use crate::manifest::Manifest;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{
    collections::HashSet,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Duration,
};

pub trait Subscriptions<Context> {
    fn on_update(&mut self, updates: &mut HashSet<PathBuf>, context: &mut Context);
}

pub fn watch_manifest<S: Subscriptions<Manifest>>(mut subs: S, mut manifest: Manifest) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

    watcher
        .watch(&manifest.root, RecursiveMode::Recursive)
        .unwrap();

    // compile the project after we've set up watchers
    let _ = manifest.compile();

    let mut updates = HashSet::new();
    let target_filter = manifest.root.join("target");
    let target_filter = Path::new(&target_filter);

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

        if should_rebuild_manifest {
            let _ = manifest.rebuild_manifest();
        }

        if should_compile {
            let _ = manifest.compile();
        }

        if !updates.is_empty() {
            subs.on_update(&mut updates, &mut manifest);
            updates.clear();
        }
    }
}

pub fn watch_directory<S: Subscriptions<()>>(mut subs: S, root: PathBuf) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

    watcher.watch(&root, RecursiveMode::Recursive).unwrap();

    let mut updates = HashSet::new();

    loop {
        fn map_event(event: DebouncedEvent) -> Option<PathBuf> {
            use DebouncedEvent::*;
            match event {
                Create(path) | Write(path) | Chmod(path) | Rename(_, path) => Some(path),
                _ => None,
            }
        }

        if let Ok(event) = rx.recv() {
            let mut handle_event = |event| {
                if let Some(path) = map_event(event) {
                    if path.extension() == Some(OsStr::new("euph")) {
                        updates.insert(path);
                    }
                }
            };

            handle_event(event);

            while let Ok(event) = rx.try_recv() {
                handle_event(event);
            }
        }

        if !updates.is_empty() {
            subs.on_update(&mut updates, &mut ());
            updates.clear();
        }
    }
}
