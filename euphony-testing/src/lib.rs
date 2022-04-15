use core::panic::Location;
use euphony::output::{self, message::Message};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
struct List(Arc<Mutex<String>>);

impl output::Output for List {
    fn emit(&mut self, message: Message) {
        let mut out = self.0.lock().unwrap();
        out.push_str(&format!("{}\n", message));
    }
}

#[track_caller]
pub fn start<F>(f: F)
where
    F: 'static + Future<Output = ()> + Send,
{
    let list = List::default();
    output::set_output(Box::new(list.clone()));

    euphony::runtime::Runtime::new(0).block_on(f);

    let result = list.0.lock().unwrap();

    let name = Location::caller().file();
    let name = name.split('/').last().unwrap();
    let name = name.trim_end_matches(".rs");
    insta::assert_display_snapshot!(name, *result);
}
