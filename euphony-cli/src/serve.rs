use crate::{manifest::Manifest, watcher::Subscriptions, Result};
use base64::prelude::*;
use euphony_store::{record::Digest, timeline::Timeline};
use futures::StreamExt as _;
use serde_json::{json, Map, Value};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use tokio::sync::broadcast::{self, Sender};
use tokio_stream::wrappers::BroadcastStream;
use warp::Filter;

#[derive(Debug, StructOpt)]
pub struct Serve {
    #[structopt(long, short, default_value = "3000")]
    port: u16,

    input: Option<PathBuf>,
}

impl Serve {
    pub fn run(&self) -> Result<()> {
        serve(self)
    }
}

#[tokio::main]
async fn serve(serve: &Serve) -> Result<()> {
    let (tx, _rx) = broadcast::channel(5);
    let opener = tx.clone();

    let updates_filter = warp::any().map(move || {
        BroadcastStream::new(opener.subscribe()).filter_map(|msg| async {
            msg.ok().map(|msg| {
                <Result<_, core::convert::Infallible>>::Ok(warp::sse::Event::default().data(msg))
            })
        })
    });

    let updates = warp::path("_updates")
        .and(warp::get())
        .and(updates_filter)
        .map(|updates| warp::sse::reply(warp::sse::keep_alive().stream(updates)));

    let manifest = Manifest::new(serve.input.as_deref(), None)?;
    let subs = SseSubs::new(tx, &manifest.root);

    // TODO return index view

    let routes = updates
        .or(warp::fs::dir(manifest.root.join("target/euphony")))
        .with(warp::cors().allow_any_origin().allow_method("GET"));

    manifest.watch(subs);

    log::info!("Server listening on port {}", serve.port);
    warp::serve(routes).run(([0, 0, 0, 0], serve.port)).await;

    Ok(())
}

struct SseSubs {
    sender: Sender<String>,
    project_filter: PathBuf,
}

impl SseSubs {
    pub fn new(sender: Sender<String>, root: &Path) -> Self {
        let project_filter = root.join("target/euphony");
        Self {
            sender,
            project_filter,
        }
    }
}

impl<Context> Subscriptions<Context> for SseSubs {
    fn on_update(&mut self, updates: &mut HashSet<PathBuf>, _: &mut Context) {
        // notify subscriptions of project updates
        if let Some(msg) = updates_message(&self.project_filter, updates.drain()) {
            let _ = self.sender.send(msg);
        }
    }
}

/// The message `/_updates` sends for the files at `paths`: a JSON object that holds each timeline
/// among them, keyed by its path under `root`. A file that can't be read, that isn't a timeline or
/// that isn't under `root` is left out, and there is no message where every file is left out.
fn updates_message<I: IntoIterator<Item = PathBuf>>(root: &Path, paths: I) -> Option<String> {
    let mut msg = Map::new();

    for path in paths {
        let Ok(contents) = std::fs::read(&path) else {
            continue;
        };

        // the timeline is a typed record on disk; the update sends it as JSON
        let timeline = match Timeline::from_bytes(&contents) {
            Ok(timeline) => timeline,
            Err(err) => {
                log::debug!("{} isn't a timeline: {}", path.display(), err);
                continue;
            }
        };

        let Ok(key) = path.strip_prefix(root) else {
            continue;
        };

        msg.insert(key.display().to_string(), timeline_json(&timeline));
    }

    if msg.is_empty() {
        None
    } else {
        Some(Value::Object(msg).to_string())
    }
}

/// The timeline as JSON, in the form the timeline file had as JSON: each digest is the name of its
/// file in the store, and a group without MIDI has no `midi` field.
fn timeline_json(timeline: &Timeline) -> Value {
    let groups: Vec<Value> = timeline
        .groups
        .iter()
        .map(|group| {
            let mut out = Map::new();
            out.insert("name".into(), group.name.clone().into());
            out.insert("entries".into(), file_name(&group.entries).into());
            if let Some(midi) = &group.midi {
                out.insert("midi".into(), file_name(midi).into());
            }
            Value::Object(out)
        })
        .collect();

    json!({
        "sample_rate": timeline.sample_rate,
        "groups": groups,
    })
}

/// The name of the file the store keeps a record under: its digest as URL-safe base64 without
/// padding.
fn file_name(digest: &Digest) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(digest.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use euphony_store::{
        storage::{fs::Directory, Storage as _},
        timeline::Group,
    };
    use std::fs;

    fn timeline() -> Timeline {
        Timeline {
            sample_rate: 48_000,
            groups: vec![
                Group {
                    name: "lead".into(),
                    entries: Digest([7; 32]),
                    midi: Some(Digest([8; 32])),
                },
                Group {
                    name: "bass".into(),
                    entries: Digest([0xfb; 32]),
                    midi: None,
                },
            ],
        }
    }

    fn digest(text: &Value) -> Digest {
        let bytes = BASE64_URL_SAFE_NO_PAD
            .decode(text.as_str().expect("a digest is sent as text"))
            .expect("a digest is sent as URL-safe base64");
        Digest(bytes.try_into().expect("a digest has 32 bytes"))
    }

    /// The timeline that a timeline's JSON in a message reads back as.
    fn read_back(sent: &Value) -> Timeline {
        Timeline {
            sample_rate: sent["sample_rate"].as_u64().unwrap() as u32,
            groups: sent["groups"]
                .as_array()
                .unwrap()
                .iter()
                .map(|group| Group {
                    name: group["name"].as_str().unwrap().into(),
                    entries: digest(&group["entries"]),
                    midi: group.get("midi").map(digest),
                })
                .collect(),
        }
    }

    #[test]
    fn a_timeline_is_sent_as_json_keyed_by_its_path() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let timeline = timeline();

        let timeline_path = root.join("project").join("timeline");
        fs::create_dir_all(timeline_path.parent().unwrap()).unwrap();
        fs::write(&timeline_path, timeline.to_bytes().unwrap()).unwrap();
        assert!(timeline_path.exists());

        let notes = root.join("project").join("notes.txt");
        fs::write(&notes, b"not a timeline").unwrap();
        assert!(notes.exists());

        let msg = updates_message(root, [timeline_path, notes]).expect("the timeline is sent");
        let msg: Value = serde_json::from_str(&msg).unwrap();
        let msg = msg.as_object().expect("the message is an object");

        let key = Path::new("project").join("timeline").display().to_string();
        assert_eq!(msg.keys().collect::<Vec<_>>(), [&key]);
        let sent = &msg[&key];

        assert_eq!(sent["sample_rate"], 48_000);
        assert_eq!(sent["groups"][0]["entries"], file_name(&Digest([7; 32])));
        assert_eq!(sent["groups"][0]["midi"], file_name(&Digest([8; 32])));
        assert_eq!(sent["groups"][1]["entries"], file_name(&Digest([0xfb; 32])));
        // a group without MIDI has no `midi` field, not a null one
        assert!(!sent["groups"][1].as_object().unwrap().contains_key("midi"));

        assert_eq!(read_back(sent), timeline);
    }

    #[test]
    fn files_that_arent_timelines_are_left_out() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();

        let notes = root.join("notes.txt");
        fs::write(&notes, b"not a timeline").unwrap();
        let missing = root.join("missing");
        assert!(!missing.exists());

        // a timeline outside the project's output isn't sent either
        let outside = tempfile::tempdir().unwrap();
        let elsewhere = outside.path().join("timeline");
        fs::write(&elsewhere, timeline().to_bytes().unwrap()).unwrap();
        assert!(elsewhere.exists());

        assert_eq!(updates_message(root, [notes, missing, elsewhere]), None);
        assert_eq!(updates_message(root, Vec::<PathBuf>::new()), None);
    }

    #[test]
    fn a_digest_is_sent_as_the_name_of_its_file_in_the_store() {
        let dir = tempfile::tempdir().unwrap();
        let directory = Directory::new(dir.path().to_owned());
        fs::create_dir_all(directory.path()).unwrap();

        // 0xfb is written as `-` and `_` in URL-safe base64, and as `+` and `/` in the standard one
        let hash = [0xfb; 32];
        directory.write_record(&hash, b"a record").unwrap();

        let name = file_name(&Digest(hash));
        assert!(!name.contains(['+', '/', '=']));
        assert!(directory.path().join(&name).exists());
    }
}
