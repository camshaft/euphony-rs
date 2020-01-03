use crate::midi::Connection;
use core::ops::Range;
use euphony::{
    midi::{channel::Channel, key::Key, message::MIDIMessage, velocity::Velocity},
    pitch::{
        interval::Interval,
        mode::{western::*, Mode},
    },
    runtime::{
        future::reactor,
        graph::{
            cell::{cell, Cell},
            node::Node,
            subscription::Readable,
        },
        online::OnlineRuntime,
        time::delay,
    },
    time::{beat::Beat, measure::Measure, tempo::Tempo, time_signature::TimeSignature},
};
use failure::Error;

pub mod midi;

fn main() -> Result<(), Error> {
    let master_conn = midi::open(None)?;

    let mut runtime = OnlineRuntime::new();

    let time = Time {
        tempo: cell(Tempo(250, 1)),
        time_signature: cell(TimeSignature(4, 4)),
    };

    let mode = gen_mode(time.clone());

    let tonic = gen_tonic(time.clone());

    gen_melody(
        time.clone(),
        tonic.map(|tonic| tonic + Interval(3, 1)),
        mode.clone(),
        master_conn.clone(),
        Channel::new(0).unwrap(),
        Beat(1, 8)..Beat(1, 2),
        -3..8,
    );

    ctrlc::set_handler(move || {
        master_conn.close();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    runtime.render();

    println!("Connection closed");

    Ok(())
}

fn gen_mode(time: Time) -> Node<Cell<Mode>> {
    let mode = cell(Mode::new(0, DOUBLE_HARMONIC));
    let mode_writer = mode.clone();

    reactor::spawn(async move {
        loop {
            for i in 0..7 {
                mode_writer.set(Mode::new(i, PRIMA));
                time.delay_for_measure(Measure(2, 1)).await;
            }
            for i in 0..7 {
                mode_writer.set(Mode::new(i, SECUNDA));
                time.delay_for_measure(Measure(2, 1)).await;
            }
            for i in 0..7 {
                mode_writer.set(Mode::new(i, TERTIA));
                time.delay_for_measure(Measure(2, 1)).await;
            }
            for i in 0..7 {
                mode_writer.set(Mode::new(i, DOUBLE_HARMONIC));
                time.delay_for_measure(Measure(2, 1)).await;
            }
        }
    });

    mode
}

fn gen_tonic(time: Time) -> Node<Cell<Interval>> {
    let tonic = cell(I);
    let tonic_writer = tonic.clone();

    reactor::spawn(async move {
        loop {
            for i in 0..=12 {
                // tonic_writer.set(Interval(i, 12));
                time.delay_for_measure(Measure(2, 1)).await;
            }
        }
    });

    tonic
}

fn gen_melody(
    time: Time,
    tonic: impl Readable<Output = Interval> + 'static,
    mode: impl Readable<Output = Mode> + 'static,
    connection: Connection,
    channel: Channel,
    beat_range: Range<Beat>,
    interval_range: Range<i64>,
) {
    let mut prev = None;

    let mut play = move |note: Interval, velocity| {
        if let Some(prev) = prev {
            connection.send(MIDIMessage::NoteOff {
                channel,
                key: Key::new(prev).unwrap(),
                velocity,
            });
        }

        let note = tonic.read() + mode.read() * note;
        let note = (note * 12u8 + 21u8).whole() as u8;

        connection.send(MIDIMessage::NoteOn {
            channel,
            key: Key::new(note).unwrap(),
            velocity,
        });

        prev = Some(note);
    };

    let velocity = Velocity::new(127).unwrap();
    let duration = Beat(1, 8);

    reactor::spawn(async move {
        loop {
            for i in 0..=7 {
                play(Interval(i, 7), velocity);
                time.delay_for_beat(duration).await;
            }
            for i in (0..=6).rev() {
                play(Interval(i, 7), velocity);
                time.delay_for_beat(duration).await;
            }
            time.delay_for_beat(duration).await;
        }
    });
}

#[derive(Clone, Debug)]
struct Time {
    pub time_signature: Node<Cell<TimeSignature>>,
    pub tempo: Node<Cell<Tempo>>,
}

impl Time {
    pub async fn delay_for_beat(&self, beat: Beat) {
        let beat: Beat = (beat / self.time_signature.read().beat()).into();
        let duration = self.tempo.read() * beat;

        delay::delay_for(duration).await;
    }

    pub async fn delay_for_measure(&self, measure: Measure) {
        let beats = measure * self.time_signature.read();
        let duration = self.tempo.read() * beats;
        delay::delay_for(duration).await;
    }
}
