use euphony::{midi::Message, prelude::Beat};
use std::collections::BTreeMap;

pub struct Timeline {
    banks: BTreeMap<u8, Bank>,
}

impl Timeline {
    pub fn insert(&mut self, bank: u8, time: Beat, message: Message) {
        self.banks.entry(beat).or_default().insert(time, message);
    }

    pub fn clear(&mut self, bank: u8) {
        self.banks.remove(&bank);
    }

    pub fn set_status(&mut self, bank: u8, status: bool) {
        if let Some(bank) = self.banks.get_mut(&bank) {
            bank.set_status(status);
        }
    }

    pub fn set_period(&mut self, bank: u8, period: u32) {
        self.banks.entry(bank).or_default().set_period(period);
    }

    pub fn set_quantization(&mut self, bank: u8, length: Beat) {
        self.banks.entry(bank).or_default().set_quantization(length);
    }

    pub fn next_expiration(&self, now: Beat) -> Option<Beat> {
        self.banks
            .iter()
            .filter_map(|bank| bank.next_expiration(now))
            .min()
    }

    // TODO events for time
}

#[derive(Debug, Default)]
pub struct Bank {
    period: u32,
    quant: Beat,
    status: bool,
    events: BTreeMap<u32, Vec<Event>>,
}

impl Bank {
    pub fn insert(&mut self, time: Beat, message: Message) {
        todo!();
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn set_period(&mut self, period: u32) {
        self.period = period;
        todo!()
    }

    pub fn set_quantization(&mut self, length: Beat) {
        self.quant = length;
        todo!()
    }

    pub fn set_status(&mut self, status: bool) {
        self.status = status;
    }

    pub fn next_expiration(&self, now: Beat) -> Option<Beat> {
        if !self.status || self.events.is_empty() {
            return None;
        }

        // TODO
        todo!()
    }

    // TODO events for time
}

struct Event {
    original_time: Beat,
    message: Message,
}
