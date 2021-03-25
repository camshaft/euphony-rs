use euphony::midi::message::MIDIMessage;

pub trait Composition: Clone {
    fn send(&self, message: MIDIMessage);
    fn close(&self);
}
