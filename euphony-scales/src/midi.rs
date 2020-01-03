use dialoguer::Select;
use euphony::midi::{
    channel::Channel, codec::MIDIValue, key::Key, message::MIDIMessage, velocity::Velocity,
};
use failure::{bail, Error};
use midir::{MidiOutput, MidiOutputConnection};
use std::sync::{Arc, Mutex};

pub fn open(index: Option<usize>) -> Result<Connection, Error> {
    let midi_out = MidiOutput::new("Euphony")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port = match (out_ports.len(), index) {
        (0, _) => bail!("no output port found"),
        (1, _) => &out_ports[0],
        (_, Some(index)) => out_ports.get(index).unwrap(),
        _ => {
            let items: Vec<_> = out_ports
                .iter()
                .map(|port| midi_out.port_name(port).unwrap())
                .collect();
            let index = Select::new()
                .with_prompt("Select output port: ")
                .default(0)
                .items(&items)
                .interact()?;
            out_ports.get(index).unwrap()
        }
    };

    Ok(Connection::new(
        midi_out.connect(out_port, "euphony").unwrap(),
    ))
}

pub struct Connection(Arc<Mutex<MidiOutputConnection>>);

impl Clone for Connection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Connection {
    pub fn new(connection: MidiOutputConnection) -> Self {
        Self(Arc::new(Mutex::new(connection)))
    }

    pub fn send<M: MIDIValue>(&self, message: M) {
        let mut bytes = vec![];
        let len = message.encode(&mut bytes).unwrap();
        self.send_bytes(&bytes[0..len])
    }

    pub fn send_bytes(&self, message: &[u8]) {
        let _ = self.0.lock().unwrap().send(message);
    }

    pub fn reset(&self) {
        let velocity = Velocity::new(0).unwrap();
        for channel in 0..16 {
            let channel = Channel::new(channel).unwrap();
            for key in 0..128 {
                self.send(MIDIMessage::NoteOff {
                    channel,
                    velocity,
                    key: Key::new(key).unwrap(),
                });
            }
        }
        self.send(MIDIMessage::Reset);
    }

    pub fn close(&self) {
        self.reset();
    }
}
