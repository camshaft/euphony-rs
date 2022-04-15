use euphony_command::message::{self, Message};
use once_cell::sync::Lazy;
use std::{sync::Mutex, time::Duration};

pub trait Output: 'static + Send + Sync {
    fn emit(&mut self, message: Message);
}

struct Stdout;

impl Output for Stdout {
    fn emit(&mut self, message: Message) {
        println!("{}", message);
    }
}

static OUTPUT: Lazy<Mutex<Box<dyn Output>>> = Lazy::new(|| Mutex::new(Box::new(Stdout)));

pub fn set_output(output: Box<dyn Output>) {
    *OUTPUT.lock().unwrap() = output;
}

pub fn emit(message: Message) {
    OUTPUT.lock().unwrap().emit(message);
}

pub fn set_seed(seed: u64) {
    emit(Message::SetSeed { seed });
}

pub fn set_group_name(id: u64, name: &str) {
    emit(Message::SetGroupName { id, name });
}

pub fn advance(amount: Duration) {
    emit(Message::AdvanceTime { amount })
}

pub fn finish() {
    emit(Message::Finish);
}
