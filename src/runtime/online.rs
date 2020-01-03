use crate::{
    runtime::{
        future::reactor,
        time::driver::{handle, Driver},
    },
    time::timestamp::Timestamp,
};

#[derive(Debug)]
pub struct OnlineRuntime {
    driver: Driver,
}

impl Default for OnlineRuntime {
    fn default() -> Self {
        let driver = Driver::default();
        Self { driver }
    }
}

impl OnlineRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self) {
        let guard = handle::set_default(self.driver.handle());

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

        drop(guard);
    }
}
