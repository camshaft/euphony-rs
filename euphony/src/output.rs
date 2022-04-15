pub use euphony_command::message;
use once_cell::sync::Lazy;
use std::{fs, io, path::Path, sync::Mutex, time::Duration};

use message::Message;

pub trait Output: 'static + Send + Sync {
    fn emit(&mut self, message: Message);
}

pub struct Stdout;

impl Output for Stdout {
    fn emit(&mut self, message: Message) {
        println!("{}", message);
    }
}

pub struct File(io::BufWriter<fs::File>);

impl File {
    pub fn create(path: &Path) -> io::Result<Self> {
        let file = fs::File::create(path)?;
        let file = io::BufWriter::new(file);
        Ok(Self(file))
    }
}

impl Output for File {
    fn emit(&mut self, message: Message) {
        message.write(&mut self.0).unwrap();
    }
}

static OUTPUT: Lazy<Mutex<Box<dyn Output>>> = Lazy::new(|| Mutex::new(Box::new(Stdout)));

pub fn set_output(output: Box<dyn Output>) {
    *OUTPUT.lock().unwrap() = output;
}

pub fn set_file(path: &Path) {
    let file = File::create(path);
    let file = file.expect("could not create output file");
    let output = Box::new(file);
    set_output(output)
}

pub(crate) fn emit(message: Message) {
    OUTPUT.lock().unwrap().emit(message);
}

pub(crate) fn set_seed(seed: u64) {
    emit(Message::SetSeed { seed });
}

pub(crate) fn set_group_name(id: u64, name: &str) {
    emit(Message::SetGroupName { id, name });
}

pub(crate) fn advance(amount: Duration) {
    emit(Message::AdvanceTime { amount })
}

pub(crate) fn finish() {
    emit(Message::Finish);
}
