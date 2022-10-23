#![allow(dead_code)] // ignore pin_project unused functions

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use pin_project::pin_project;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static GROUPS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

bach::scope::define!(scope, Group);

pub fn current() -> Group {
    scope::try_borrow_with(|group| group.unwrap_or_else(|| Group::new("main")))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Group {
    id: u64,
}

impl Group {
    pub fn new(name: &str) -> Self {
        GROUPS.with(|groups| {
            let mut groups = groups.borrow_mut();

            if let Some(id) = groups.get(name).copied() {
                return Self { id };
            }

            let id = groups.len() as u64;

            crate::output::create_group(id, name.to_string());

            groups.insert(name.to_owned(), id);

            Self { id }
        })
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

#[must_use = "futures do nothing unless polled"]
#[pin_project]
pub struct Grouped<Inner> {
    #[pin]
    inner: Inner,
    group: Group,
}

impl<Inner> Grouped<Inner> {
    pub fn new(inner: Inner, group: Group) -> Self {
        Self { inner, group }
    }
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
