use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Midi {
    NoteOff {
        channel: Local,
        key: Local,
        velocity: Local,
    },
    NoteOn {
        channel: Local,
        key: Local,
        velocity: Local,
    },
    // TODO implement the rest
    /*
    Aftertouch {  },
    Controller,
    ProgramChange,
    ChannelAftertouch,
    PitchBend,
    Channel,
    Key,
    Velocity,

    Common,
    MidiTime,
    SongPosition,
    SongSelect,
    TuneRequest,

    Realtime,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
    */
}

impl Midi {
    pub fn apply(&self, locals: &[Event]) -> Option<Event> {
        match self {
            Midi::NoteOff {
                channel,
                key,
                velocity,
            } => {
                let channel = locals[channel.id].as_u4()?;
                let key = locals[key.id].as_u7()?;
                let vel = locals[velocity.id].as_u7()?;
                Some(Event::Midi {
                    channel,
                    message: MidiMessage::NoteOff { key, vel },
                })
            }
            Midi::NoteOn {
                channel,
                key,
                velocity,
            } => {
                let channel = locals[channel.id].as_u4()?;
                let key = locals[key.id].as_u7()?;
                let vel = locals[velocity.id].as_u7()?;
                Some(Event::Midi {
                    channel,
                    message: MidiMessage::NoteOn { key, vel },
                })
            }
        }
    }
}
