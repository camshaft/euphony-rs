use core::task::Poll;
use euphony_units::time::Beat;
use futures::{FutureExt, Stream};
use midly::{num, Format, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};

#[derive(Debug)]
pub struct File<'a> {
    smf: Smf<'a>,
}

impl<'a> File<'a> {
    #[track_caller]
    pub fn parse(bytes: &'a [u8]) -> Self {
        let smf = Smf::parse(bytes).expect("could not parse midi file");
        Self { smf }
    }

    #[track_caller]
    pub fn events(&self) -> Events {
        Events {
            ticks_per_beat: match self.smf.header.timing {
                Timing::Metrical(t) => t.as_int() as _,
                Timing::Timecode(_, _) => unimplemented!("midi timecode"),
            },
            iter: self.aggregate_iter(),
            next: None,
        }
    }

    fn aggregate_iter(&self) -> AggregateIter {
        let tracks = self.smf.tracks.iter().map(|t| TrackIter::new(t)).collect();
        let parallel = self.smf.header.format == Format::Parallel;
        AggregateIter {
            parallel,
            tracks,
            time: 0,
        }
    }
}

struct AggregateIter<'a> {
    parallel: bool,
    tracks: Vec<TrackIter<'a>>,
    time: u32,
}

impl<'a> Iterator for AggregateIter<'a> {
    type Item = (usize, TrackEvent<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = None;

        for (tidx, track) in self.tracks.iter_mut().enumerate() {
            if let Some(time) = track.peek() {
                if let Some((_prev_idx, prev_time)) = idx {
                    if time < prev_time {
                        idx = Some((tidx, time));
                    }
                } else {
                    idx = Some((tidx, time));
                }

                if !self.parallel {
                    break;
                }
            }
        }

        let (idx, time) = idx?;
        let prev_time = core::mem::replace(&mut self.time, time);
        let delta = num::u28::from_int_lossy(time - prev_time);

        let kind = unsafe { self.tracks.get_unchecked_mut(idx).next() };

        Some((idx, TrackEvent { delta, kind }))
    }
}

struct TrackIter<'a> {
    time: u32,
    iter: core::iter::Peekable<core::slice::Iter<'a, TrackEvent<'a>>>,
}

impl<'a> TrackIter<'a> {
    fn new(t: &'a [TrackEvent]) -> Self {
        Self {
            time: 0,
            iter: t.iter().peekable(),
        }
    }

    fn peek(&mut self) -> Option<u32> {
        let event = self.iter.peek()?;
        let time = self.time + event.delta.as_int();
        Some(time)
    }

    fn next(&mut self) -> TrackEventKind<'a> {
        let event = self.iter.next().unwrap();
        let time = self.time + event.delta.as_int();
        self.time = time;
        event.kind
    }
}

pub struct Events<'a> {
    ticks_per_beat: u64,
    iter: AggregateIter<'a>,
    next: Option<(crate::time::Timer, Event)>,
}

impl<'a> Iterator for Events<'a> {
    type Item = (Beat, Event);

    fn next(&mut self) -> Option<Self::Item> {
        let mut total_delta = 0u64;
        loop {
            let (_track_idx, event) = self.iter.next()?;
            total_delta += event.delta.as_int() as u64;
            if let TrackEventKind::Midi { channel, message } = &event.kind {
                let beat = Beat(total_delta, self.ticks_per_beat).reduce();
                let event = Event {
                    channel: channel.as_int(),
                    message: (*message).into(),
                };
                return Some((beat, event));
            }
        }
    }
}

impl<'a> Stream for Events<'a> {
    type Item = Event;

    fn poll_next(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if let Some((mut timer, event)) = self.next.take() {
            if timer.poll_unpin(cx).is_ready() {
                return Some(event).into();
            } else {
                self.next = Some((timer, event));
                return Poll::Pending;
            }
        }

        if let Some((delta, event)) = self.next() {
            if delta.0 == 0 {
                Some(event).into()
            } else {
                self.next = Some((crate::time::delay(delta), event));
                Poll::Pending
            }
        } else {
            None.into()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event {
    pub channel: u8,
    pub message: Message,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Message {
    NoteOff { key: u8, velocity: u8 },
    NoteOn { key: u8, velocity: u8 },
    Aftertouch { key: u8, velocity: u8 },
    Controller { controller: u8, value: u8 },
    ProgramChange { program: u8 },
    ChannelAftertouch { velocity: u8 },
    PitchBend { bend: i16 },
}

impl From<MidiMessage> for Message {
    fn from(msg: MidiMessage) -> Self {
        match msg {
            MidiMessage::NoteOff { key, vel } => Message::NoteOff {
                key: key.as_int(),
                velocity: vel.as_int(),
            },
            MidiMessage::NoteOn { key, vel } => Message::NoteOn {
                key: key.as_int(),
                velocity: vel.as_int(),
            },
            MidiMessage::Aftertouch { key, vel } => Message::Aftertouch {
                key: key.as_int(),
                velocity: vel.as_int(),
            },
            MidiMessage::Controller { controller, value } => Message::Controller {
                controller: controller.as_int(),
                value: value.as_int(),
            },
            MidiMessage::ProgramChange { program } => Message::ProgramChange {
                program: program.as_int(),
            },
            MidiMessage::ChannelAftertouch { vel } => Message::ChannelAftertouch {
                velocity: vel.as_int(),
            },
            MidiMessage::PitchBend { bend } => Message::PitchBend {
                bend: bend.as_int(),
            },
        }
    }
}

impl Message {
    pub fn emit(&self) {
        use crate::output::EmitMidi;
        let data = self.as_bytes();
        let group = crate::group::scope::try_borrow_with(|g| g.map(|g| g.as_u64()));
        crate::output::emit(EmitMidi { data, group })
    }

    /// Write the data part of this message, including the channel
    pub(crate) fn as_bytes(self) -> [u8; 3] {
        let mut data = match self {
            Self::NoteOff { key, velocity } => [0x8, key, velocity],
            Self::NoteOn { key, velocity } => [0x9, key, velocity],
            Self::Aftertouch { key, velocity } => [0xA, key, velocity],
            Self::Controller { controller, value } => [0xB, controller, value],
            Self::ProgramChange { program } => [0xC, program, 0],
            Self::ChannelAftertouch { velocity } => [0xD, velocity, 0],
            Self::PitchBend { bend } => {
                let [a, b] = bend.to_be_bytes();
                [0xE, a, b]
            }
        };
        data[0] <<= 4;
        data
    }
}

pub fn on<K: IntoKey>(key: K, velocity: u8) {
    Message::NoteOn {
        key: key.into_key(),
        velocity,
    }
    .emit();
}

pub fn off<K: IntoKey>(key: K, velocity: u8) {
    Message::NoteOff {
        key: key.into_key(),
        velocity,
    }
    .emit();
}

pub fn aftertouch<K: IntoKey>(key: K, velocity: u8) {
    Message::Aftertouch {
        key: key.into_key(),
        velocity,
    }
    .emit();
}

pub fn controller(controller: u8, value: u8) {
    Message::Controller { controller, value }.emit();
}

pub fn program_change(program: u8) {
    Message::ProgramChange { program }.emit();
}

pub fn channel_aftertouch(velocity: u8) {
    Message::ChannelAftertouch { velocity }.emit();
}

pub fn pitch_bend(bend: i16) {
    Message::PitchBend { bend }.emit();
}

pub trait IntoKey {
    fn into_key(self) -> u8;
}

impl IntoKey for u8 {
    fn into_key(self) -> u8 {
        self
    }
}

impl IntoKey for crate::prelude::Interval {
    fn into_key(self) -> u8 {
        (self * 12i64 + 69i64).whole().clamp(0, 127) as u8
    }
}
