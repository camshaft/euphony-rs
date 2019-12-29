use crate::runtime::graph::registry::Registry;
use core::cell::RefCell;

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default())
}

pub fn with<F: FnOnce(&Registry) -> Output, Output>(f: F) -> Output {
    REGISTRY.with(|registry| f(&registry.borrow()))
}

pub fn with_mut<F: FnOnce(&mut Registry) -> Output, Output>(f: F) -> Output {
    REGISTRY.with(|registry| f(&mut registry.borrow_mut()))
}
