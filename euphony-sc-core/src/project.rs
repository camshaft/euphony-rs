use crate::track;
use std::sync::Arc;

#[derive(Clone)]
pub struct Handle(pub Arc<dyn 'static + Project + Send + Sync>);

impl Handle {
    pub fn track(&self, name: &str) -> track::Handle {
        Project::track(self, name)
    }
}

impl Project for Handle {
    fn track(&self, name: &str) -> track::Handle {
        self.0.track(name)
    }
}

pub trait Project: 'static + Send {
    /// Returns the track for the given name
    fn track(&self, name: &str) -> track::Handle;
}
