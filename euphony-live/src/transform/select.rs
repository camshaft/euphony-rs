use super::*;

#[derive(Clone, Debug)]
pub enum Select {
    Channel,
    Key,
    Velocity,
    Controller,
    ControllerValue,
    Program,
    PitchBend,
    SongPosition,
    SongSelect,
    Table(Vec<Local>),
}

impl Select {
    pub fn apply(&self, event: &Event, locals: &[Event]) -> Option<Event> {
        use Select as S;
        Some(match (self, event) {
            (S::Channel, Event::Midi { channel, .. }) => Event::from(*channel),
            (S::Key, Event::Midi { message, .. }) => match message {
                MidiMessage::NoteOff { key, .. }
                | MidiMessage::NoteOn { key, .. }
                | MidiMessage::Aftertouch { key, .. } => Event::from(*key),
                _ => return None,
            },
            (S::Velocity, Event::Midi { message, .. }) => match message {
                MidiMessage::NoteOff { vel, .. }
                | MidiMessage::NoteOn { vel, .. }
                | MidiMessage::Aftertouch { vel, .. }
                | MidiMessage::ChannelAftertouch { vel } => Event::from(*vel),
                _ => return None,
            },
            (
                S::Controller,
                Event::Midi {
                    message: MidiMessage::Controller { controller, .. },
                    ..
                },
            ) => Event::from(*controller),
            (
                S::ControllerValue,
                Event::Midi {
                    message: MidiMessage::Controller { value, .. },
                    ..
                },
            ) => Event::from(*value),
            (
                S::Program,
                Event::Midi {
                    message: MidiMessage::ProgramChange { program },
                    ..
                },
            ) => Event::from(*program),
            (
                S::PitchBend,
                Event::Midi {
                    message: MidiMessage::PitchBend { bend },
                    ..
                },
            ) => Event::from(bend.as_int() as i64),
            (S::SongPosition, Event::Common(event::Common::SongPosition(value))) => {
                Event::from(*value)
            }
            (S::SongSelect, Event::Common(event::Common::SongSelect(value))) => Event::from(*value),
            (S::Table(entries), event) => {
                if entries.is_empty() {
                    return None;
                }

                let idx = event.as_number()?.whole();
                let idx = (idx % entries.len() as i64) as usize;
                let local = entries[idx];
                locals[local.id].clone()
            }
            _ => return None,
        })
    }
}
