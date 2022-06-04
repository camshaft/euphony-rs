use crate::{manifest::Manifest, watcher::Subscriptions, Result};
use futures::StreamExt as _;
use std::{
    collections::HashSet,
    fmt::Write as _,
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
        let mut msg = String::new();
        for path in updates.drain() {
            // notify subscriptions of project updates
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if msg.is_empty() {
                    msg.push('{');
                } else {
                    msg.push(',');
                }

                let path = path.strip_prefix(&self.project_filter).unwrap().display();

                write!(msg, "{:?}", path).unwrap();
                msg.push(':');
                msg.push_str(&contents);
            }
        }

        if !msg.is_empty() {
            msg.push('}');
            let _ = self.sender.send(msg);
        }
    }
}
