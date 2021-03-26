#![no_std]
extern crate alloc;

use alloc::sync::Arc;
pub use euphony_sc_core::*;
pub use euphony_sc_macros::*;

pub mod project {
    use super::*;

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
}

pub mod track {
    use super::*;
    use core::fmt;

    #[derive(Clone)]
    pub struct Handle(pub Arc<dyn 'static + Track + Send + Sync>);

    impl fmt::Debug for Handle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Track").field("name", &self.name()).finish()
        }
    }

    impl Handle {
        pub fn send<T: Message>(&self, msg: T) -> T::Output {
            T::send(msg, &self)
        }
    }

    impl Track for Handle {
        fn name(&self) -> &str {
            self.0.name()
        }

        fn load(&self, synthname: &str, synthdef: &[u8]) {
            self.0.load(synthname, synthdef)
        }

        fn new(
            &self,
            synthname: &str,
            action: Option<osc::group::Action>,
            target: Option<osc::node::Id>,
            values: &[Option<(osc::control::Id, osc::control::Value)>],
        ) -> osc::node::Id {
            self.0.new(synthname, action, target, values)
        }

        fn set(
            &self,
            id: osc::node::Id,
            values: &[Option<(osc::control::Id, osc::control::Value)>],
        ) {
            self.0.set(id, values)
        }

        fn free(&self, id: osc::node::Id) {
            self.0.free(id)
        }
    }

    pub trait Track: 'static + Send {
        fn name(&self) -> &str;

        fn load(&self, synthname: &str, synthdef: &[u8]);

        fn new(
            &self,
            synthname: &str,
            action: Option<osc::group::Action>,
            target: Option<osc::node::Id>,
            values: &[Option<(osc::control::Id, osc::control::Value)>],
        ) -> osc::node::Id;

        fn set(
            &self,
            id: osc::node::Id,
            values: &[Option<(osc::control::Id, osc::control::Value)>],
        );

        fn free(&self, id: osc::node::Id);
    }
}

pub trait Message {
    type Output;

    fn send(self, track: &track::Handle) -> Self::Output;
}
