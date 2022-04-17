use crate::units::time::{Beat, Tempo};

pub use bach::time::scheduler::{self, Handle, Scheduler, Timer};

bach::scope::define!(resolution, Beat);

mod tempo {
    use super::*;

    bach::scope::define!(scope, Tempo);
}

const DEFAULT_TEMPO: Tempo = Tempo(120, 1);

pub fn tempo() -> Tempo {
    tempo::scope::try_borrow_with(|t| t.unwrap_or(DEFAULT_TEMPO))
}

pub fn set_tempo(tempo: Tempo) -> Tempo {
    let duration = tempo * beats_per_tick();
    crate::output::set_tick_duration(duration);
    tempo::scope::set(Some(tempo)).unwrap_or(DEFAULT_TEMPO)
}

pub fn delay(beats: Beat) -> scheduler::Timer {
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
    resolution::try_borrow_with(|v| v.unwrap_or(Beat(1, 4096)))
}
