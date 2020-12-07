use euphony::midi::{
    channel::Channel, controller::ControllerValue, key::Key, message::MIDIMessage,
    velocity::Velocity,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Mallet {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Mallet {
    fn new(channel: Channel) -> Self {
        match (*channel) % 4 {
            0 => Self::Red,
            1 => Self::Yellow,
            2 => Self::Green,
            3 => Self::Blue,
            _ => unreachable!(),
        }
    }
}

macro_rules! evt {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            mallet: Mallet,
            target: Target,
            velocity: Velocity,
        }

        impl $name {
            fn new(channel: Channel, key: Key, velocity: Velocity) -> Option<Self> {
                let mallet = Mallet::new(channel);
                let target = match *channel {
                    0..=3 => Target::Bar(key),
                    4..=7 => {
                        let pad = Pad::new(key)?;
                        Target::Pad(pad)
                    }
                    _ => return None,
                };
                Some(Self {
                    mallet,
                    target,
                    velocity,
                })
            }
        }
    };
}

evt!(On);
evt!(Off);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    mallet: Mallet,
    value: ControllerValue,
}

impl Position {
    fn new(channel: Channel, value: ControllerValue) -> Self {
        let mallet = Mallet::new(channel);
        Self { mallet, value }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Target {
    Bar(Key),
    Pad(Pad),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pad {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
}

impl Pad {
    fn new(key: Key) -> Option<Self> {
        match **key {
            60 => Some(Self::P0),
            61 => Some(Self::P1),
            62 => Some(Self::P2),
            63 => Some(Self::P3),
            64 => Some(Self::P4),
            65 => Some(Self::P5),
            66 => Some(Self::P6),
            67 => Some(Self::P7),
            68 => Some(Self::P8),
            69 => Some(Self::P9),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Strip {
    A,
    B,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Event {
    On(On),
    Off(Off),
    Position(Position),
    StripPosition(Strip, Position),
}

impl Event {
    pub fn from_midi(message: MIDIMessage) -> Option<Self> {
        match message {
            MIDIMessage::NoteOn {
                channel,
                key,
                velocity,
            } => Some(Self::On(On::new(channel, key, velocity)?)),
            MIDIMessage::NoteOff {
                channel,
                key,
                velocity,
            } => Some(Self::Off(Off::new(channel, key, velocity)?)),
            MIDIMessage::ControlChange {
                channel,
                controller,
                value,
            } => {
                let position = Position::new(channel, value);
                match **controller {
                    0 => Some(Self::Position(position)),
                    1 => Some(Self::StripPosition(Strip::A, position)),
                    2 => Some(Self::StripPosition(Strip::B, position)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
