use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use once_cell::sync::Lazy;
use pin_project::pin_project;
use std::{collections::HashMap, sync::Mutex};

static GROUPS: Lazy<Mutex<HashMap<String, u64>>> = Lazy::new(|| Mutex::new(HashMap::new()));

bach::scope::define!(scope, Group);

pub fn current() -> Group {
    scope::try_borrow_with(|v| v.unwrap_or_else(|| Group::new("main")))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Group {
    id: u64,
}

impl Group {
    pub fn new(name: &str) -> Self {
        let mut groups = GROUPS.lock().unwrap();

        if let Some(id) = groups.get(name).copied() {
            return Self { id };
        }

        let id = groups.len() as u64;

        crate::runtime::output::set_group_name(id, name);

        groups.insert(name.to_owned(), id);

        Self { id }
    }

    pub(crate) fn as_u64(self) -> u64 {
        self.id
    }
}

pub trait Ext: Sized {
    fn group(self, name: &str) -> Grouped<Self>;
}

impl<T> Ext for T
where
    T: Future,
{
    fn group(self, name: &str) -> Grouped<Self> {
        Grouped {
            inner: self,
            group: Group::new(name),
        }
    }
}

#[pin_project]
pub struct Grouped<Inner> {
    #[pin]
    inner: Inner,
    group: Group,
}

impl<Inner> Future for Grouped<Inner>
where
    Inner: Future,
{
    type Output = Inner::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let inner = this.inner;
        let group = this.group;
        scope::with(*group, || Future::poll(inner, cx))
    }
}
