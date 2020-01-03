use crate::{composition::Composition, time::Time};
use core::{cell::RefCell, convert::TryInto};
use euphony::{
    midi::{
        channel::Channel,
        codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue},
        integer::u28varint,
        message::MIDIMessage,
        smf::{chunk::ChunkHeader, format::Format, header::Header, timing::Timing},
    },
    runtime::graph::subscription::Readable,
    time::{beat::Beat, timestamp::Timestamp},
};
use failure::Error;
use std::{
    collections::BTreeMap,
    io::{BufWriter, Write},
    path::Path,
    rc::Rc,
};
use structopt::StructOpt;

const RESOLUTION: Beat = Beat(1, 192);

#[derive(Debug, StructOpt)]
pub struct Render {
    pub output: String,
}

pub struct File(Rc<RefCell<Inner>>);

impl File {
    pub fn new(time: &Time) -> Self {
        Self(Rc::new(RefCell::new(Inner {
            time: time.clone(),
            timing: Timing::from_beat(RESOLUTION).unwrap(),
            tracks: Default::default(),
        })))
    }

    pub fn write(&self, render: &Render) -> Result<(), Error> {
        let inner = self.0.borrow();

        if render.output == "-" {
            let mut out = std::io::stdout();
            inner.encode(&mut out)?;
            out.flush()?;
        } else {
            let mut path = Path::new(&render.output).to_path_buf();

            if path.is_dir() {
                let time_signature = inner.time.time_signature.read();
                path.push(format!(
                    "{}-{}_{:x}.mid",
                    time_signature.0,
                    time_signature.1,
                    crate::rng::seed()
                ));
            };

            eprintln!("Writing to {:?}", path);

            let mut file = std::fs::File::create(path)?;

            let mut out = BufWriter::new(&mut file);
            inner.encode(&mut out)?;
            out.flush()?;
        };

        Ok(())
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug)]
struct Inner {
    time: Time,
    timing: Timing,
    tracks: BTreeMap<Channel, Track>,
}

impl MIDIValue for Inner {
    fn decode<B: DecoderBuffer>(_: &mut B) -> Result<Self, DecoderError> {
        unimplemented!()
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> std::result::Result<(), B::Error> {
        let mut header = Header {
            timing: self.timing,
            track_count: self.tracks.len().try_into().unwrap(),
            // track_count: 0,
            format: Format::Parallel,
        };

        // tempo track
        header.track_count += 1;

        buffer.encode(&ChunkHeader::Header(header.encoding_len()))?;
        buffer.encode(&header)?;

        {
            let tempo = self.time.tempo.read();
            let time_signature = self.time.time_signature.read();

            // Timesignature
            let time_signature_header_format = &[0x18, 0x08][..];
            let time_signature_header = &[
                0x00,
                0xFF,
                0x58,
                (time_signature.encoding_len() + time_signature_header_format.len()) as u8,
            ][..];

            // key signature
            // 0x00, 0xFF, 0x59, 0x02, 0x00, 0x00,

            let tempo_header = &[
                // tempo
                0x00,
                0xFF,
                0x51,
                tempo.encoding_len() as u8,
            ][..];

            buffer.encode(&ChunkHeader::Track(
                tempo_header.len()
                    + tempo.encoding_len()
                    + time_signature_header.len()
                    + time_signature.encoding_len()
                    + time_signature_header_format.len()
                    + END_OF_TRACK.len(),
            ))?;

            buffer.write_bytes(tempo_header)?;
            buffer.encode(&tempo)?;

            buffer.write_bytes(time_signature_header)?;
            buffer.encode(&time_signature)?;
            buffer.write_bytes(time_signature_header_format)?;

            buffer.write_bytes(END_OF_TRACK)?;
        }

        for (_channel, track) in self.tracks.iter() {
            buffer.encode(track)?;
        }

        Ok(())
    }

    fn encoding_len(&self) -> usize {
        unimplemented!()
    }
}

impl Inner {
    fn send(&mut self, message: MIDIMessage) {
        match message {
            MIDIMessage::NoteOn { channel, .. }
            | MIDIMessage::NoteOff { channel, .. }
            | MIDIMessage::PolyphonicKeyPressure { channel, .. }
            | MIDIMessage::ControlChange { channel, .. }
            | MIDIMessage::ProgramChange { channel, .. }
            | MIDIMessage::ChannelPressure { channel, .. }
            | MIDIMessage::PitchBendChange { channel, .. } => {
                let beat = self
                    .time
                    .duration_beats(Timestamp::now() - Timestamp::default(), RESOLUTION);

                let ticks = self.timing.beat_to_ticks(beat).unwrap();

                let track = self.tracks.entry(channel).or_default();

                track.write_at(ticks, message);
            }
            _ => {
                println!("{:?}", message);
            }
        }
    }
}

impl Composition for File {
    fn send(&self, message: MIDIMessage) {
        self.0.borrow_mut().send(message)
    }

    fn close(&self) {
        println!("CLOSE");
    }
}

#[derive(Debug, Default)]
pub struct Track {
    tick: u64,
    data: Vec<u8>,
}
const END_OF_TRACK: &[u8] = &[0x00, 0xFF, 0x2F, 0x00];

impl MIDIValue for Track {
    fn decode<B: DecoderBuffer>(_: &mut B) -> Result<Self, DecoderError> {
        unimplemented!()
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> std::result::Result<(), B::Error> {
        buffer.encode(&ChunkHeader::Track(self.data.len() + END_OF_TRACK.len()))?;

        buffer.write_bytes(&self.data)?;
        buffer.write_bytes(END_OF_TRACK)?;

        Ok(())
    }

    fn encoding_len(&self) -> usize {
        unimplemented!()
    }
}

impl Track {
    fn write_at(&mut self, tick: u64, message: MIDIMessage) {
        let delta = u28varint::new(tick - self.tick).expect("tick delta too large");
        self.tick = tick;
        delta.encode(&mut self.data).unwrap();
        message.encode(&mut self.data).unwrap();
    }
}
