use crate::{
    runtime::{
        future::reactor,
        time::driver::{handle, handle::DefaultGuard, Driver},
    },
    time::timestamp::Timestamp,
};

#[derive(Debug)]
pub struct OnlineRuntime {
    driver: Driver,
    guard: DefaultGuard,
}

impl Default for OnlineRuntime {
    fn default() -> Self {
        let driver = Driver::default();
        let guard = handle::set_default(driver.handle());
        Self { driver, guard }
    }
}

impl OnlineRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self) {
        let mut now = Timestamp::default();
        while reactor::tick() {
            while self.driver.process(now) {
                reactor::tick();
            }

            if let Some(next) = self.driver.prepare_park() {
                std::thread::sleep(next - now);
                now = next;
            }
        }
    }
}
