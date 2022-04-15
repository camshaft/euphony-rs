use crate::units::time::{Beat, Tempo};
use core::sync::atomic::{AtomicU64, Ordering};

pub use bach::time::scheduler::{self, Handle, Scheduler, Timer};

bach::scope::define!(resolution, Beat);

static TEMPO_NUM: AtomicU64 = AtomicU64::new(120);
static TEMPO_DEN: AtomicU64 = AtomicU64::new(1);

pub fn tempo() -> Tempo {
    let num = TEMPO_NUM.load(Ordering::SeqCst);
    let den = TEMPO_DEN.load(Ordering::SeqCst);
    Tempo(num, den)
}

pub fn set_tempo(tempo: Tempo) -> Tempo {
    // Since a euphony application is single threaded, this doesn't need any additional
    // synchronization
    let num = TEMPO_NUM.swap(tempo.0, Ordering::SeqCst);
    let den = TEMPO_DEN.swap(tempo.1, Ordering::SeqCst);
    Tempo(num, den)
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
    resolution::try_borrow_with(|v| v.unwrap_or(Beat(1, 1024)))
}

pub(super) fn ticks_to_duration(ticks: u64) -> core::time::Duration {
    tempo() * (beats_per_tick() * ticks)
}
