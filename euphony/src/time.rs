use crate::units::time::{Beat, Tempo};

pub(crate) use bach::time::scheduler::{self, Scheduler};

pub use bach::time::scheduler::Timer;

bach::scope::define!(resolution, Beat);

mod tempo {
    use super::*;

    bach::scope::define!(scope, Tempo);
}

pub fn tempo() -> Tempo {
    tempo::scope::try_borrow_with(|t| t.unwrap_or(Tempo::DEFAULT))
}

pub fn set_tempo(tempo: Tempo) -> Tempo {
    let beats_per_tick = beats_per_tick();
    let duration = tempo * beats_per_tick;
    crate::output::set_timing(
        duration,
        beats_per_tick
            .as_ratio()
            .inverse()
            .try_into_whole()
            .unwrap(),
    );
    tempo::scope::set(Some(tempo)).unwrap_or(Tempo::DEFAULT)
}

pub fn delay(beats: Beat) -> Timer {
    scheduler::scope::borrow_with(|handle| {
        let ticks = beats / beats_per_tick();
        let ticks = ticks.whole();
        handle.delay(ticks)
    })
}

pub fn now() -> Beat {
    scheduler::scope::borrow_with(|handle| beats_per_tick() * handle.ticks())
}

fn beats_per_tick() -> Beat {
    resolution::try_borrow_with(|v| v.unwrap_or(Beat::DEFAULT_RESOLUTION))
}
