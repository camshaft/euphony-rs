use crate::event::Event;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Cell(Arc<Inner>);

impl Default for Cell {
    fn default() -> Self {
        Cell::new(Event::undefined())
    }
}

#[derive(Debug)]
struct Inner {
    value: RwLock<Event>,
}

impl Cell {
    pub fn new(value: Event) -> Self {
        Self(Arc::new(Inner {
            value: RwLock::new(value),
        }))
    }

    pub fn set(&self, value: Event) -> Event {
        core::mem::replace(&mut self.0.value.write().unwrap(), value)
    }

    pub fn clear(&self) -> Event {
        self.set(Event::undefined())
    }

    pub fn get(&self) -> Event {
        self.0.value.read().unwrap().clone()
    }
}
