use anyhow::Result;
use core::ops;
use euphony::{midi::Message, pitch::mode::Mode, prelude::*, units::time::Beat};
use euphony_midi::notes::{self as note, Note, Notes};

/*
pub mod cell;
pub mod connection;
pub mod event;
pub mod transform;
*/
mod timeline;

pub struct App {
    inputs: Controllers,
    state: State,
    output_prev: Controllers,
    output_current: Controllers,
}

impl App {
    // TODO export configuration
    //
    // TODO on_timeout

    pub fn on_event(&mut self, event: &Event, _outputs: &mut Outputs) {
        self.inputs.on_event(event);
    }

    pub fn render(&mut self, outputs: &mut Outputs) {
        for (note, velocity) in self.inputs[0][0].notes() {
            let interval = Interval::from_midi(*note);
            let interval = self.state.mode.collapse(interval, Default::default());
            let interval = self.state.interval + interval;
            let interval = self.state.mode.expand(interval, Default::default());
            let interval = self.state.tonic + interval;

            if let Some(note) = interval.into_midi().and_then(Note::new) {
                self.output_current.on_event(&Event {
                    controller: 0,
                    event: ControllerEvent {
                        channel: 0,
                        message: Message::NoteOn {
                            key: *note,
                            velocity,
                        },
                    },
                });
            }
        }

        for event in self.output_prev.diff(&self.output_current) {
            outputs.on_event(&event);
        }

        // prepare for the next iteration
        core::mem::swap(&mut self.output_prev, &mut self.output_current);
        self.output_current.reset();
    }

    pub fn timeout(&self) -> Option<Beat> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct State {
    mode: Mode,
    tonic: Interval,
    interval: Interval,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: western::MAJOR,
            tonic: Interval(0, 1),
            interval: Interval(0, 1),
        }
    }
}

pub struct Controllers {
    controllers: Vec<Controller>,
}

impl Controllers {
    pub fn on_event(&mut self, event: &Event) {
        let controller = event.controller;
        if self.controllers.len() <= controller {
            self.controllers.resize(controller + 1, Default::default());
        }

        self.controllers[controller].on_event(&event.event);
    }

    pub fn diff<'a>(&'a self, next: &'a Self) -> impl Iterator<Item = Event> + 'a {
        // TODO handle different sized controllers
        self.iter()
            .zip(next.iter())
            .enumerate()
            .flat_map(|(controller, (prev, next))| {
                prev.diff(next)
                    .map(move |event| Event { controller, event })
            })
    }

    pub fn reset(&mut self) {
        for controller in self.iter_mut() {
            controller.reset();
        }
    }
}

impl ops::Deref for Controllers {
    type Target = [Controller];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.controllers
    }
}

impl ops::DerefMut for Controllers {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.controllers
    }
}

#[derive(Clone, Debug, Default)]
pub struct Controller {
    channels: Vec<Channel>,
}

impl ops::Deref for Controller {
    type Target = [Channel];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.channels
    }
}

impl ops::DerefMut for Controller {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channels
    }
}

impl Controller {
    pub fn on_event(&mut self, event: &ControllerEvent) {
        let channel = event.channel as usize;
        if self.channels.len() <= channel {
            self.channels.resize(channel + 1, Default::default());
        }

        self.channels[channel].on_message(&event.message);
    }

    pub fn diff<'a>(&'a self, next: &'a Self) -> impl Iterator<Item = ControllerEvent> + 'a {
        // TODO handle different sized channels
        self.iter()
            .zip(next.iter())
            .enumerate()
            .flat_map(|(channel, (prev, next))| {
                let channel = channel as u8;
                prev.diff(next)
                    .map(move |message| ControllerEvent { message, channel })
            })
    }

    pub fn reset(&mut self) {
        for channel in self.iter_mut() {
            channel.reset();
        }
    }
}

#[derive(Clone, Debug)]
pub struct Channel {
    notes: Notes,
    on_velocities: [u8; 128],
    off_velocities: [u8; 128],
    controllers: [u8; 128],
    aftertouch: [u8; 128],
    channel_aftertouch: u8,
    program: u8,
    bend: i16,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            notes: Default::default(),
            on_velocities: [0; 128],
            off_velocities: [0; 128],
            controllers: [0; 128],
            aftertouch: [0; 128],
            channel_aftertouch: 0,
            program: 0,
            bend: 0,
        }
    }
}

impl Channel {
    pub fn is_on(&self, note: Note) -> bool {
        self.notes.get(note) > 0
    }

    pub fn note_on(&mut self, note: Note, velocity: u8) {
        if self.notes.on(note).is_some() {
            self.on_velocities[(*note) as usize] = velocity;
        }
    }

    pub fn note_off(&mut self, note: Note, velocity: u8) {
        if self.notes.off(note).is_some() {
            self.off_velocities[(*note) as usize] = velocity;
        }
    }

    pub fn notes(&self) -> impl Iterator<Item = (Note, u8)> + '_ {
        self.notes.active().map(|note| {
            let vel = self.on_velocities[(*note) as usize];
            (note, vel)
        })
    }

    pub fn diff<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = Message> + 'a {
        self.notes.diff(&other.notes).map(|event| match event {
            note::Event::On { note } => {
                let key = *note;
                let velocity = self.on_velocities[key as usize];
                Message::NoteOn { key, velocity }
            }
            note::Event::Off { note } => {
                let key = *note;
                let velocity = self.off_velocities[key as usize];
                Message::NoteOff { key, velocity }
            }
        })
    }

    pub fn on_message(&mut self, message: &Message) {
        match *message {
            Message::NoteOff { key, velocity } => {
                if let Some(key) = Note::new(key) {
                    self.note_off(key, velocity);
                }
            }
            Message::NoteOn { key, velocity } => {
                if let Some(key) = Note::new(key) {
                    self.note_on(key, velocity);
                }
            }
            Message::Aftertouch { key, velocity } => {
                if let Some(v) = self.aftertouch.get_mut(key as usize) {
                    *v = velocity;
                }
            }
            Message::Controller { controller, value } => {
                if let Some(v) = self.controllers.get_mut(controller as usize) {
                    *v = value;
                }
            }
            Message::ProgramChange { program } => self.program = program,
            Message::ChannelAftertouch { velocity } => self.channel_aftertouch = velocity,
            Message::PitchBend { bend } => self.bend = bend,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub struct Event {
    pub controller: usize,
    pub event: ControllerEvent,
}

pub struct ControllerEvent {
    pub channel: u8,
    pub message: Message,
}

pub enum Source {
    Timeout(Beat),
    Input(usize),
}

pub struct Outputs {
    outputs: Vec<Output>,
}

impl Outputs {
    pub fn on_event(&mut self, event: &Event) {
        // TODO
    }
}

pub struct Output {
    // TODO
}
