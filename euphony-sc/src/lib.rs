pub use euphony_sc_core::*;
pub use euphony_sc_macros::*;

pub mod server {
    use super::{osc, Message};
    use core::fmt;
    use std::{cell::RefCell, sync::Arc};

    #[derive(Clone)]
    pub struct Server(Handle);

    pub type Handle = Arc<dyn Api>;

    thread_local! {
        static INSTANCE: RefCell<Option<Handle>> = RefCell::new(None);
    }

    #[derive(Clone, Copy)]
    pub struct Error(());

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error(missing supercollider server)")
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "missing supercollider server")
        }
    }

    impl Server {
        pub fn current() -> Result<Self, Error> {
            INSTANCE.with(|i| i.borrow().clone().map(Self).ok_or_else(|| Error(())))
        }

        pub fn with<F: FnOnce() -> R, R>(&self, f: F) -> R {
            let prev = INSTANCE.with(|i| {
                let prev = i.borrow().clone();
                *i.borrow_mut() = Some(self.0.clone());
                prev
            });

            let res = f();

            INSTANCE.with(|i| *i.borrow_mut() = prev);

            res
        }

        pub fn send<T: Message>(&self, msg: T) -> T::Output {
            T::send(msg, &self.0)
        }
    }

    pub trait Api {
        /// Allocates a message buffer of length MTU
        fn alloc(&self) -> Vec<u8>;

        /// Assigns a new Id
        fn assign(&self) -> osc::node::Id;

        fn default_target(&self) -> osc::node::Id;

        fn default_add_action(&self) -> osc::group::Action;

        /// Sends a message to the server
        fn send(&self, message: Vec<u8>);

        /// should probably call /s_noid rather than /n_free
        fn free(&self, id: osc::node::Id);
    }
}

pub use server::Server;

pub trait Message: Sized {
    type Output;

    fn send(self, api: &server::Handle) -> Self::Output;

    fn send_current(self) -> Self::Output {
        Server::current().unwrap().send(self)
    }
}
