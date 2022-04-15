use crate::output::emit;
use euphony_command::message::{self, Message};
use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicU64;

static DEF_ID: AtomicU64 = AtomicU64::new(0);
static SYNTH_ID: AtomicU64 = AtomicU64::new(0);

pub use message::SynthDef;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition<F = fn() -> SynthDef<'static>> {
    callback: F,
    id: OnceCell<u64>,
}

impl<F> Definition<F> {
    pub const fn new(callback: F) -> Self {
        Self {
            callback,
            id: OnceCell::new(),
        }
    }
}

impl Definition {
    pub fn spawn(&self) -> Synth {
        let synthdef = self.synthdef();

        let id = SYNTH_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let group = crate::runtime::group::current().as_u64();

        emit(Message::Spawn {
            id,
            synthdef,
            group,
        });

        Synth(id)
    }

    fn synthdef(&self) -> u64 {
        let callback = self.callback;
        *self.id.get_or_init(|| {
            let definition = callback();

            let id = DEF_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

            emit(Message::SynthDef { id, definition });

            id
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Synth(u64);

impl Synth {
    pub fn set(&self, parameter: u64, value: f64) {
        emit(Message::Set {
            id: self.0,
            parameter,
            value,
        })
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        emit(Message::Drop { id: self.0 })
    }
}
