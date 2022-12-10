use crate::{event::Event, Result};
use anyhow::anyhow;
use midir::{
    os::unix::{VirtualInput, VirtualOutput},
    MidiInput, MidiInputConnection, MidiOutput,
};
use std::{fmt, sync::Arc};

pub(crate) type Receiver = async_broadcast::Receiver<(usize, Event)>;
pub(crate) type Sender = async_broadcast::Sender<(usize, Event)>;

pub(crate) fn channel() -> (Sender, Receiver) {
    async_broadcast::broadcast(CAPACITY)
}

const CAPACITY: usize = 10000;

#[derive(Clone)]
pub struct Output {
    id: usize,
    sender: Sender,
}

impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Output").field(&self.id).finish()
    }
}

impl Output {
    pub(crate) fn new(id: usize, sender: Sender) -> Self {
        Self { id, sender }
    }

    pub fn open(client: &str, name: &str, is_virtual: bool) -> Result<Output> {
        Self::open_with_id(client, name, is_virtual, usize::MAX)
    }

    pub fn open_with_id(client: &str, name: &str, is_virtual: bool, id: usize) -> Result<Output> {
        let client = MidiOutput::new(client)?;

        let mut output = if is_virtual {
            client.create_virtual(name)
        } else {
            let port = client
                .ports()
                .into_iter()
                .find(|port| {
                    if let Ok(n) = client.port_name(port) {
                        n == name
                    } else {
                        false
                    }
                })
                .ok_or_else(|| anyhow!("could not find output {:?}", name))?;
            client.connect(&port, name)
        }
        .map_err(|e| anyhow!("{}", e))?;

        let (sender, mut receiver) = channel();

        std::thread::spawn(move || {
            futures::executor::block_on(async move {
                let mut buffer = vec![];
                while let Ok((_target_id, event)) = receiver.recv().await {
                    if event.write(&mut buffer).is_some() {
                        let _ = output.send(&buffer);
                    }
                    buffer.clear();
                }
            })
        });

        Ok(Self { sender, id })
    }

    pub fn send(&mut self, event: Event) {
        let _ = self.sender.try_broadcast((self.id, event));
    }

    pub fn send_delay(&mut self, event: Event, delay: core::time::Duration) {
        if delay.is_zero() {
            return self.send(event);
        }

        let mut s = self.clone();
        tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            s.send(event);
        });
    }
}

pub struct Input {
    receiver: Receiver,
    handle: Option<Arc<Handle>>,
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Input").finish()
    }
}

impl Input {
    pub(crate) fn spawn(
        client: &str,
        name: &str,
        is_virtual: bool,
        output: Output,
    ) -> Result<Handle> {
        let client = MidiInput::new(client)?;

        let input = if is_virtual {
            client.create_virtual(name, input_callback, output)
        } else {
            dbg!(name);
            let port = client
                .ports()
                .into_iter()
                .find(|port| {
                    if let Ok(n) = dbg!(client.port_name(port)) {
                        n == name
                    } else {
                        false
                    }
                })
                .ok_or_else(|| anyhow!("could not find input {:?}", name))?;
            client.connect(&port, name, input_callback, output)
        }
        .map_err(|e| anyhow!("{}", e))?;

        Ok(Handle { conn: Some(input) })
    }

    pub fn open(client: &str, name: &str, is_virtual: bool) -> Result<Input> {
        let (sender, receiver) = channel();
        let output = Output {
            sender,
            id: usize::MAX,
        };
        let handle = Self::spawn(client, name, is_virtual, output)?;
        let handle = Some(Arc::new(handle));

        Ok(Self { receiver, handle })
    }

    pub async fn recv(&mut self) -> Event {
        self.recv_idx().await.1
    }

    pub async fn recv_idx(&mut self) -> (usize, Event) {
        self.receiver.recv().await.unwrap()
    }

    pub(crate) fn new(receiver: Receiver) -> Self {
        Input {
            receiver,
            handle: None,
        }
    }
}

fn input_callback(_timestamp: u64, data: &[u8], sender: &mut Output) {
    if let Some(event) = Event::parse(data) {
        sender.send(event);
    }
}

pub(crate) struct Handle {
    conn: Option<MidiInputConnection<Output>>,
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Handle").finish()
    }
}

// Safety: conn is only accessed on drop
unsafe impl Sync for Handle {}

impl Clone for Input {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.new_receiver(),
            handle: self.handle.clone(),
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let _ = conn.close();
        }
    }
}
