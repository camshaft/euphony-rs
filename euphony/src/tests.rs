use crate as euphony;
use euphony::output;
use std::{
    future::Future,
    io::{self, Cursor},
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
struct List(Arc<Mutex<Cursor<Vec<u8>>>>);

impl io::Write for List {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn start<F>(name: &str, f: F)
where
    F: 'static + Future<Output = ()> + Send,
{
    let list = List::default();
    output::scope::set(Some(Box::new(list.clone())));

    euphony::runtime::Runtime::new(0).block_on(f);

    let mut result = core::mem::take(&mut *list.0.lock().unwrap());
    result.set_position(0);

    let mut dump = String::new();
    output::decode(&mut result, &mut dump).unwrap();

    insta::assert_display_snapshot!(name, dump);
}

use euphony::{prelude::*, synth::Processor};

static SINE: Processor = Processor {
    id: 100,
    name: "Sine",
    inputs: 3,
};
static SQUARE: Processor = Processor {
    id: 104,
    name: "Square",
    inputs: 3,
};

#[test]
fn tempo_test() {
    start("tempo_test", async {
        for i in 0..10 {
            set_tempo(Tempo(60 + i * 10, 1));
            let s = SINE.spawn();
            s.set(i % 3, now().as_f64());
            Beat(1, 2).delay().await;
        }
    })
}

#[test]
fn rand_test() {
    start("rand_test", async {
        // make sure we have a scope in the root
        let s = SINE.spawn();
        for _ in 0..5 {
            s.set(rand::gen_range(0..3), now().as_f64());
            Beat(1, 2).delay().await;
        }
        drop(s);

        // make sure spawned tasks have scopes
        async {
            let s = SINE.spawn();
            for _ in 0..5 {
                s.set(rand::gen_range(0..3), now().as_f64());
                Beat(1, 2).delay().await;
            }
        }
        .spawn()
        .await;

        // make sure we can seed tasks
        async {
            let s = SINE.spawn();
            for _ in 0..5 {
                s.set(rand::gen_range(0..3), now().as_f64());
                Beat(1, 2).delay().await;
            }
        }
        .seed(0)
        .spawn()
        .await;
    });
}

#[test]
fn scheduler_test() {
    start("scheduler_test", async {
        async {
            for i in 0..10 {
                let s = SINE.spawn();
                s.set(i % 3, now().as_f64());
                Beat(1, 2).delay().await;
            }
        }
        .spawn_primary();

        async {
            for i in 0..10 {
                let s = SQUARE.spawn();
                s.set(i % 3, now().as_f64());
                Beat(1, 4).delay().await;
            }
        }
        .spawn_primary();
    })
}
