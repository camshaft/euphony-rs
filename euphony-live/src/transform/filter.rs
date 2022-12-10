use super::*;

#[derive(Clone, Debug)]
pub enum Filter {
    Midi,
    NoteOff,
    NoteOn,
    Aftertouch,
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
}

impl Filter {
    pub fn apply(&self, event: &Event) -> bool {
        use Filter as F;
        match self {
            F::Midi => matches!(event, Event::Midi { .. }),
            F::NoteOff => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::NoteOff { .. },
                    ..
                }
            ),
            F::NoteOn => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::NoteOn { .. },
                    ..
                }
            ),
            F::Aftertouch => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::Aftertouch { .. },
                    ..
                }
            ),
            F::Controller => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::Controller { .. },
                    ..
                }
            ),
            F::ProgramChange => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::ProgramChange { .. },
                    ..
                }
            ),
            F::ChannelAftertouch => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::ChannelAftertouch { .. },
                    ..
                }
            ),
            F::PitchBend => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::PitchBend { .. },
                    ..
                }
            ),
            F::Channel => {
                matches!(event, Event::Midi { .. })
            }
            F::Key => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::NoteOff { .. }
                        | MidiMessage::NoteOn { .. }
                        | MidiMessage::Aftertouch { .. },
                    ..
                }
            ),
            F::Velocity => matches!(
                event,
                Event::Midi {
                    message: MidiMessage::NoteOff { .. }
                        | MidiMessage::NoteOn { .. }
                        | MidiMessage::Aftertouch { .. },
                    ..
                }
            ),
            F::Common => matches!(event, Event::Common { .. }),
            F::MidiTime => matches!(
                event,
                Event::Common(event::Common::MidiTimeCodeQuarterFrame(_, _))
            ),
            F::SongPosition => {
                matches!(event, Event::Common(event::Common::SongPosition(_)))
            }
            F::SongSelect => {
                matches!(event, Event::Common(event::Common::SongSelect(_)))
            }
            F::TuneRequest => matches!(event, Event::Common(event::Common::TuneRequest)),
            F::Realtime => matches!(event, Event::Realtime { .. }),
            F::TimingClock => matches!(event, Event::Realtime(event::Realtime::TimingClock)),
            F::Start => matches!(event, Event::Realtime(event::Realtime::Start)),
            F::Continue => matches!(event, Event::Realtime(event::Realtime::Continue)),
            F::Stop => matches!(event, Event::Realtime(event::Realtime::Stop)),
            F::ActiveSensing => matches!(event, Event::Realtime(event::Realtime::ActiveSensing)),
            F::Reset => matches!(event, Event::Realtime(event::Realtime::Reset)),
        }
    }
}
