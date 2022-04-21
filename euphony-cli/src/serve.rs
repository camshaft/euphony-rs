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

    #[structopt(long)]
    manifest_path: Option<PathBuf>,
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

    let project = warp::path("_updates")
        .and(warp::get())
        .and(subs_filter)
        .map(|subs| warp::sse::reply(warp::sse::keep_alive().stream(Subscriber::new(subs))));

    let compiler = Manifest::new(serve.manifest_path.as_deref())?;

    let files = warp::path("euphony").and(warp::fs::dir(compiler.root.join("target/euphony/")));

    let routes = files
        .or(project)
        .with(warp::cors().allow_any_origin().allow_method("GET"));

    std::thread::spawn(move || watcher::create(subscriptions, compiler));

    warp::serve(routes).run(([0, 0, 0, 0], serve.port)).await;

    Ok(())
}
