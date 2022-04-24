use crate::{
    manifest::Manifest,
    watcher,
    watcher::{Subscriber, Subscriptions},
    Result,
};
use std::path::PathBuf;
use structopt::StructOpt;
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
    let subscriptions = Subscriptions::default();

    let filter_subs = subscriptions.clone();
    let subs_filter = warp::any().map(move || filter_subs.clone());

    let updates = warp::path("_updates")
        .and(warp::get())
        .and(subs_filter)
        .map(|subs| warp::sse::reply(warp::sse::keep_alive().stream(Subscriber::new(subs))));

    let input = match serve.input.as_ref() {
        Some(path) if path.is_dir() => Some(path.join("Cargo.toml")),
        Some(path) => Some(path.clone()),
        None => None,
    };
    let compiler = Manifest::new(input.as_deref(), None)?;

    // TODO return index view

    let routes = updates
        .or(warp::fs::dir(compiler.root.join("target/euphony")))
        .with(warp::cors().allow_any_origin().allow_method("GET"));

    std::thread::spawn(move || watcher::create(subscriptions, compiler));

    eprintln!("Server listening on port {}", serve.port);
    warp::serve(routes).run(([0, 0, 0, 0], serve.port)).await;

    Ok(())
}
