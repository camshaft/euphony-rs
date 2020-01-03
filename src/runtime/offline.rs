use crate::{
    runtime::{
        future::reactor,
        time::driver::{handle, Driver},
    },
    time::timestamp::Timestamp,
};
use core::time::Duration;

#[derive(Debug, Default)]
pub struct OfflineRuntime {
    driver: Driver,
}

impl OfflineRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_for(&mut self, limit: Duration) {
        let guard = handle::set_default(self.driver.handle());

        let mut now = Timestamp::default();
        while reactor::tick() {
            while self.driver.process(now) {
                reactor::tick();
            }

            if let Some(next) = self.driver.prepare_park() {
                now = next;
                if now - Timestamp::default() > limit {
                    drop(guard);
                    return;
                }
            }
        }

        drop(guard);
    }

    pub fn render(&mut self) {
        let guard = handle::set_default(self.driver.handle());

        let mut now = Timestamp::default();
        while reactor::tick() {
            while self.driver.process(now) {
                reactor::tick();
            }

            if let Some(next) = self.driver.prepare_park() {
                now = next;
            }
        }

        drop(guard);
    }
}

#[test]
fn offline_test() {
    use crate::{
        pitch::interval::Interval,
        runtime::{graph::cell::cell, time::delay},
        time::timestamp::Timestamp,
    };
    use alloc::rc::Rc;
    use core::{cell::RefCell, time::Duration};
    use futures::stream::StreamExt;

    let mut driver = OfflineRuntime::default();
    let cell = cell(Interval(1, 1));
    let mut observer = cell.clone().observe();

    let observed = Rc::new(RefCell::new(vec![]));

    reactor::spawn(async move {
        delay::delay_for(Duration::from_millis(1)).await;
        cell.set(Interval(2, 1));
        delay::delay_for(Duration::from_millis(1)).await;
        cell.set(Interval(3, 1));
        delay::delay_for(Duration::from_millis(1)).await;
        cell.set(Interval(4, 1));
    });

    let observed_ref = observed.clone();
    reactor::spawn(async move {
        while let Some(current) = observer.next().await {
            observed_ref.borrow_mut().push((Timestamp::now(), current));
        }
    });

    driver.render();

    assert_eq!(
        &observed.borrow()[..],
        &[
            (Default::default(), Interval(1, 1)),
            (
                Timestamp::from_duration(Duration::from_millis(1)),
                Interval(2, 1)
            ),
            (
                Timestamp::from_duration(Duration::from_millis(2)),
                Interval(3, 1)
            )
        ][..]
    );
}
