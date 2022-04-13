use crate::{
    buffer::{BufView, Buffer},
    osc, Message,
};
use core::{fmt, time::Duration};
use std::sync::Arc;

#[derive(Clone)]
pub struct Handle(pub Arc<dyn 'static + Track + Send + Sync>);

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Track").field("name", &self.name()).finish()
    }
}

impl Handle {
    pub fn send<T: Message>(&self, msg: T) -> T::Output {
        T::send(msg, self)
    }
}

impl Track for Handle {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn load(&self, synthname: &str, synthdef: &[u8]) {
        self.0.load(synthname, synthdef)
    }

    fn play(
        &self,
        synthname: &str,
        action: Option<osc::group::Action>,
        target: Option<osc::node::Id>,
        values: &[Option<(osc::control::Id, osc::control::Value)>],
    ) -> osc::node::Id {
        self.0.play(synthname, action, target, values)
    }

    fn read(&self, buffer: BufView) -> Buffer {
        self.0.read(buffer)
    }

    fn set(&self, id: osc::node::Id, values: &[Option<(osc::control::Id, osc::control::Value)>]) {
        self.0.set(id, values)
    }

    fn free(&self, id: osc::node::Id) {
        self.0.free(id)
    }

    fn free_after(&self, id: osc::node::Id, time: Duration) {
        self.0.free_after(id, time)
    }
}

pub trait Track: 'static + Send {
    fn name(&self) -> &str;

    fn load(&self, synthname: &str, synthdef: &[u8]);

    fn play(
        &self,
        synthname: &str,
        action: Option<osc::group::Action>,
        target: Option<osc::node::Id>,
        values: &[Option<(osc::control::Id, osc::control::Value)>],
    ) -> osc::node::Id;

    fn read(&self, buffer: BufView) -> Buffer;

    fn set(&self, id: osc::node::Id, values: &[Option<(osc::control::Id, osc::control::Value)>]);

    fn free(&self, id: osc::node::Id);

    fn free_after(&self, id: osc::node::Id, time: Duration);
}
