use crate::{
    midi::{
        channel::Channel,
        codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue},
        controller::{Controller, ControllerValue},
        key::Key,
        pitch_bend::PitchBend,
        pressure::Pressure,
        program::Program,
        song::Song,
        sysex::SysExPayload,
        timecode::MIDITimeCodeQuarterFrame,
        velocity::Velocity,
    },
    time::beat::Beat,
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum MIDIMessage {
    // Channel Voice Messages
    NoteOff {
        channel: Channel,
        key: Key,
        velocity: Velocity,
    },
    NoteOn {
        channel: Channel,
        key: Key,
        velocity: Velocity,
    },
    PolyphonicKeyPressure {
        channel: Channel,
        key: Key,
        pressure: Pressure,
    },
    ControlChange {
        channel: Channel,
        controller: Controller,
        value: ControllerValue,
    },
    ProgramChange {
        channel: Channel,
        program: Program,
    },
    ChannelPressure {
        channel: Channel,
        pressure: Pressure,
    },
    PitchBendChange {
        channel: Channel,
        pitch_bend: PitchBend,
    },
    // System Common Messages
    SystemExclusive {
        payload: SysExPayload,
    },
    TimeCodeQuarterFrame {
        value: MIDITimeCodeQuarterFrame,
    },
    SongPositionPointer {
        beat: Beat,
    },
    SongSelect {
        song: Song,
    },
    TuneRequest,
    EndOfExclusive,
    // System Real-Time Messages
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
    Undefined,
}

impl From<[u8; 1]> for MIDIMessage {
    fn from(buffer: [u8; 1]) -> Self {
        (&buffer[..]).decode::<Self>().unwrap_or(Self::Undefined)
    }
}

impl From<[u8; 2]> for MIDIMessage {
    fn from(buffer: [u8; 2]) -> Self {
        (&buffer[..]).decode::<Self>().unwrap_or(Self::Undefined)
    }
}

impl From<[u8; 3]> for MIDIMessage {
    fn from(buffer: [u8; 3]) -> Self {
        (&buffer[..]).decode::<Self>().unwrap_or(Self::Undefined)
    }
}

impl MIDIMessage {
    pub fn decode_stream<B: DecoderBuffer>(buffer: B) -> MIDIMessageIterator<B> {
        MIDIMessageIterator(buffer)
    }

    pub fn is_valid(&self) -> bool {
        self != &MIDIMessage::Undefined
    }
}

// https://www.midi.org/specifications/item/table-1-summary-of-midi-message

impl MIDIValue for MIDIMessage {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        loop {
            match buffer.next_byte()? {
                0b0000_0000..=0b0111_1111 => {
                    // next byte
                    continue;
                }
                a @ 0b1000_0000..=0b1000_1111 => {
                    let channel = Channel::from_status_byte(a);
                    let key = buffer.decode()?;
                    let velocity = buffer.decode()?;
                    let message = MIDIMessage::NoteOff {
                        channel,
                        key,
                        velocity,
                    };
                    return Ok(message);
                }
                a @ 0b1001_0000..=0b1001_1111 => {
                    let channel = Channel::from_status_byte(a);
                    let key = buffer.decode()?;
                    let velocity = buffer.decode()?;
                    let message = MIDIMessage::NoteOn {
                        channel,
                        key,
                        velocity,
                    };
                    return Ok(message);
                }
                a @ 0b1010_0000..=0b1010_1111 => {
                    let channel = Channel::from_status_byte(a);
                    let key = buffer.decode()?;
                    let pressure = buffer.decode()?;
                    let message = MIDIMessage::PolyphonicKeyPressure {
                        channel,
                        key,
                        pressure,
                    };
                    return Ok(message);
                }
                a @ 0b1011_0000..=0b1011_1111 => {
                    let channel = Channel::from_status_byte(a);
                    let controller = buffer.decode()?;
                    let value = buffer.decode()?;
                    let message = MIDIMessage::ControlChange {
                        channel,
                        controller,
                        value,
                    };
                    return Ok(message);
                }
                a @ 0b1100_0000..=0b1100_1111 => {
                    let channel = Channel::from_status_byte(a);
                    let program = buffer.decode()?;
                    let message = MIDIMessage::ProgramChange { channel, program };
                    return Ok(message);
                }
                a @ 0b1101_0000..=0b1101_1111 => {
                    let channel = Channel::from_status_byte(a);
                    let pressure = buffer.decode()?;
                    let message = MIDIMessage::ChannelPressure { channel, pressure };
                    return Ok(message);
                }
                a @ 0b1110_0000..=0b1110_1111 => {
                    let channel = Channel::from_status_byte(a);
                    let pitch_bend = buffer.decode()?;
                    let message = MIDIMessage::PitchBendChange {
                        channel,
                        pitch_bend,
                    };
                    return Ok(message);
                }
                0b1111_0000 => {
                    let payload = buffer.decode()?;
                    let message = MIDIMessage::SystemExclusive { payload };
                    return Ok(message);
                }
                0b1111_0001 => {
                    let value = buffer.decode()?;
                    let message = MIDIMessage::TimeCodeQuarterFrame { value };
                    return Ok(message);
                }
                0b1111_0010 => {
                    let beat = buffer.decode()?;
                    let message = MIDIMessage::SongPositionPointer { beat };
                    return Ok(message);
                }
                0b1111_0011 => {
                    let song = buffer.decode()?;
                    let message = MIDIMessage::SongSelect { song };
                    return Ok(message);
                }
                0b1111_0100 => return Ok(MIDIMessage::Undefined),
                0b1111_0101 => return Ok(MIDIMessage::Undefined),
                0b1111_0110 => return Ok(MIDIMessage::TuneRequest),
                0b1111_0111 => return Ok(MIDIMessage::EndOfExclusive),
                0b1111_1000 => return Ok(MIDIMessage::TimingClock),
                0b1111_1001 => return Ok(MIDIMessage::Undefined),
                0b1111_1010 => return Ok(MIDIMessage::Start),
                0b1111_1011 => return Ok(MIDIMessage::Continue),
                0b1111_1100 => return Ok(MIDIMessage::Stop),
                0b1111_1101 => return Ok(MIDIMessage::Undefined),
                0b1111_1110 => return Ok(MIDIMessage::ActiveSensing),
                0b1111_1111 => return Ok(MIDIMessage::Reset),
            }
        }
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        match self {
            MIDIMessage::NoteOff {
                channel,
                key,
                velocity,
            } => {
                buffer.write_byte(0b1000_0000 | **channel)?;
                buffer.encode(key)?;
                buffer.encode(velocity)?;
            }
            MIDIMessage::NoteOn {
                channel,
                key,
                velocity,
            } => {
                buffer.write_byte(0b1001_0000 | **channel)?;
                buffer.encode(key)?;
                buffer.encode(velocity)?
            }
            MIDIMessage::PolyphonicKeyPressure {
                channel,
                key,
                pressure,
            } => {
                buffer.write_byte(0b1010_0000 | **channel)?;
                buffer.encode(key)?;
                buffer.encode(pressure)?
            }
            MIDIMessage::ControlChange {
                channel,
                controller,
                value,
            } => {
                buffer.write_byte(0b1011_0000 | **channel)?;
                buffer.encode(controller)?;
                buffer.encode(value)?
            }
            MIDIMessage::ProgramChange { channel, program } => {
                buffer.write_byte(0b1100_0000 | **channel)?;
                buffer.encode(program)?
            }
            MIDIMessage::ChannelPressure { channel, pressure } => {
                buffer.write_byte(0b1101_0000 | **channel)?;
                buffer.encode(pressure)?
            }
            MIDIMessage::PitchBendChange {
                channel,
                pitch_bend,
            } => {
                buffer.write_byte(0b1110_0000 | **channel)?;
                buffer.encode(pitch_bend)?
            }
            MIDIMessage::SystemExclusive { payload } => {
                buffer.write_byte(0b1111_0000)?;
                buffer.encode(payload)?
            }
            MIDIMessage::TimeCodeQuarterFrame { value } => {
                buffer.write_byte(0b1111_0001)?;
                buffer.encode(value)?
            }
            MIDIMessage::SongPositionPointer { beat } => {
                buffer.write_byte(0b1111_0010)?;
                buffer.encode(beat)?
            }
            MIDIMessage::SongSelect { song } => {
                buffer.write_byte(0b1111_0011)?;
                buffer.encode(song)?;
            }
            MIDIMessage::TuneRequest => buffer.write_byte(0b1111_0110)?,
            MIDIMessage::EndOfExclusive => buffer.write_byte(0b1111_0111)?,
            MIDIMessage::TimingClock => buffer.write_byte(0b1111_1000)?,
            MIDIMessage::Start => buffer.write_byte(0b1111_1010)?,
            MIDIMessage::Continue => buffer.write_byte(0b1111_1011)?,
            MIDIMessage::Stop => buffer.write_byte(0b1111_1100)?,
            MIDIMessage::ActiveSensing => buffer.write_byte(0b1111_1110)?,
            MIDIMessage::Reset => buffer.write_byte(0b1111_1111)?,
            MIDIMessage::Undefined => {}
        }
        Ok(())
    }

    fn encoding_len(&self) -> usize {
        1 + match self {
            MIDIMessage::NoteOff {
                channel: _,
                key,
                velocity,
            }
            | MIDIMessage::NoteOn {
                channel: _,
                key,
                velocity,
            } => key.encoding_len() + velocity.encoding_len(),
            MIDIMessage::PolyphonicKeyPressure {
                channel: _,
                key,
                pressure,
            } => key.encoding_len() + pressure.encoding_len(),
            MIDIMessage::ControlChange {
                channel: _,
                controller,
                value,
            } => controller.encoding_len() + value.encoding_len(),
            MIDIMessage::ProgramChange {
                channel: _,
                program,
            } => program.encoding_len(),
            MIDIMessage::ChannelPressure {
                channel: _,
                pressure,
            } => pressure.encoding_len(),
            MIDIMessage::PitchBendChange {
                channel: _,
                pitch_bend,
            } => pitch_bend.encoding_len(),
            MIDIMessage::SystemExclusive { payload } => payload.encoding_len(),
            MIDIMessage::TimeCodeQuarterFrame { value } => value.encoding_len(),
            MIDIMessage::SongPositionPointer { beat } => beat.encoding_len(),
            MIDIMessage::SongSelect { song } => song.encoding_len(),
            MIDIMessage::TuneRequest => 0,
            MIDIMessage::EndOfExclusive => 0,
            MIDIMessage::TimingClock => 0,
            MIDIMessage::Start => 0,
            MIDIMessage::Continue => 0,
            MIDIMessage::Stop => 0,
            MIDIMessage::ActiveSensing => 0,
            MIDIMessage::Reset => 0,
            MIDIMessage::Undefined => 0,
        }
    }
}

#[derive(Debug)]
pub struct MIDIMessageIterator<B>(B);

impl<B: DecoderBuffer> Iterator for MIDIMessageIterator<B> {
    type Item = MIDIMessage;

    fn next(&mut self) -> Option<MIDIMessage> {
        loop {
            let message: MIDIMessage = self.0.decode().ok()?;
            if message.is_valid() {
                return Some(message);
            }
        }
    }
}

#[test]
fn round_trip_test() {
    let mut output = vec![];
    for a in 128u8..=255 {
        for b in 0u8..=127 {
            for c in 0u8..=127 {
                let input = [a, b, c];
                if let Some(message) = (&input[..])
                    .decode::<MIDIMessage>()
                    .ok()
                    .filter(MIDIMessage::is_valid)
                {
                    output.encode(&message).unwrap();
                    assert_eq!((&input[..output.len()]), &output[..], "{:?}", message);
                    output.clear();
                }
            }
        }
    }
}

#[test]
fn snapshot_test() {
    use insta::assert_debug_snapshot;
    assert_debug_snapshot!(MIDIMessage::from([144, 69, 114]));
    assert_debug_snapshot!(MIDIMessage::from([128, 60, 127]));
    assert_debug_snapshot!(MIDIMessage::from([176, 1, 0]));
    assert_debug_snapshot!(MIDIMessage::from([192, 1]));
    assert_debug_snapshot!(MIDIMessage::from([224, 127, 127]));
    assert_debug_snapshot!(MIDIMessage::from([224, 0, 64]));
    assert_debug_snapshot!(MIDIMessage::from([224, 0, 0]));
}
