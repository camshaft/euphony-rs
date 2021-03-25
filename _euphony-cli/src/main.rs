use crate::{composition::Composition, time::Time};
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
        offline::OfflineRuntime,
        online::OnlineRuntime,
    },
    time::{beat::Beat, measure::Measure, tempo::Tempo, time_signature::TimeSignature},
};
use failure::Error;
use futures::stream::StreamExt;
use structopt::StructOpt;

mod composition;
pub mod midi;
mod play;
mod render;
pub mod rng;
pub mod time;

use rng::*;

const HI_HAT: u8 = 0x2a;
const SNARE_DRUM: u8 = 0x26;
const BASS_DRUM: u8 = 0x24;

#[derive(Debug, StructOpt)]
pub enum Arguments {
    // Dump { file: String },
    Render(render::Render),
    Play(play::Play),
}

fn main() -> Result<(), Error> {
    match Arguments::from_args() {
        // Arguments::Dump { file } => {
        //     let data = std::fs::read(file)?;
        //     let smf = midly::Smf::parse(&data)?;
        //     println!("{:#?}", smf);
        // }
        Arguments::Render(args) => {
            let time = Time {
                tempo: cell(Tempo(gen_range(100..180), 1)),
                time_signature: cell(TimeSignature(gen_range(3..8), 4)),
            };

            let composition = render::File::new(&time);

            let mut runtime = OfflineRuntime::new();
            gen_song(&time, &composition);
            runtime.render_for(time.measure_duration(Measure(16, 1)));

            composition.write(&args)?;

            std::process::exit(0);
        }
        Arguments::Play(args) => {
            let composition = midi::open(args.port)?;

            let time = Time {
                tempo: cell(Tempo(gen_range(100..180), 1)),
                time_signature: cell(TimeSignature(gen_range(3..8), 4)),
            };

            let mut runtime = OnlineRuntime::new();
            gen_song(&time, &composition);
            ctrlc::set_handler(move || {
                composition.close();
                std::process::exit(0);
            })
            .expect("Error setting Ctrl-C handler");

            runtime.render();
        }
    }

    Ok(())
}

fn gen_song(time: &Time, composition: &(impl Composition + 'static)) {
    eprintln!("{:?}", time);

    let mode = gen_mode(time.clone());

    let tonic = gen_tonic(time.clone(), &mode, || {
        one_of(&[
            Interval(-4, 7),
            Interval(-3, 7),
            Interval(0, 7),
            Interval(2, 7),
            Interval(3, 7),
            Interval(4, 7),
            // Interval(5, 7),
            Interval(7, 7),
        ])
    });

    gen_percussion(time.clone(), composition.clone(), HI_HAT, || {
        one_of(&[Beat(1, 16), Beat(1, 8), Beat(1, 4)])
    });
    gen_percussion(time.clone(), composition.clone(), SNARE_DRUM, || {
        one_of(&[Beat(1, 4), Beat(1, 2), Beat(1, 1), Beat(2, 1)])
    });
    gen_percussion(time.clone(), composition.clone(), BASS_DRUM, || {
        one_of(&[Beat(1, 4), Beat(1, 2), Beat(1, 1)])
    });

    gen_melody(
        time.clone(),
        tonic.map(|tonic| tonic + Interval(3, 1)),
        mode.clone(),
        composition.clone(),
        Channel::C1,
        || one_of(&[Beat(1, 8), Beat(1, 4), Beat(1, 2)]),
        || {
            one_of(&[
                Interval(-4, 7),
                Interval(-3, 7),
                Interval(-2, 7),
                Interval(0, 7),
                Interval(2, 7),
                Interval(3, 7),
                Interval(4, 7),
                Interval(5, 7),
                Interval(7, 7),
            ])
        },
        {
            let mut velocities: Vec<_> = (0..gen_range(1..16))
                .map(|_| Velocity::new(gen_range(75..127)).unwrap())
                .collect();
            velocities.push(Velocity::new(0).unwrap());
            move || one_of(&velocities)
        },
    );

    gen_melody(
        time.clone(),
        tonic.map(|tonic| tonic + Interval(1, 1)),
        mode.clone(),
        composition.clone(),
        Channel::C2,
        || one_of(&[Beat(1, 8), Beat(1, 4), Beat(1, 2)]),
        || {
            one_of(&[
                Interval(0, 7),
                Interval(2, 7),
                // Interval(3, 7),
                Interval(4, 7),
                // Interval(5, 7),
                Interval(7, 7),
            ])
        },
        {
            let mut velocities: Vec<_> = (0..gen_range(1..16))
                .map(|_| Velocity::new(gen_range(75..127)).unwrap())
                .collect();
            velocities.push(Velocity::new(0).unwrap());
            move || one_of(&velocities)
        },
    );
}

fn gen_mode(time: Time) -> Node<Cell<Mode>> {
    let mode = cell(MAJOR);
    let mode_writer = mode.clone();

    // let beats = gen_beats(time.time_signature.read().total_beats(), beat_range);

    fn gen_single_mode() -> Mode {
        // Mode::new(
        //     gen_range(0..7),
        //     one_of(&[PRIMA, SECUNDA, TERTIA, DOUBLE_HARMONIC]),
        // )
        Mode::new(gen_range(0..7), PRIMA)
    }

    let first_modes = [gen_single_mode(), gen_single_mode()];
    let second_modes = [gen_single_mode(), gen_single_mode()];

    reactor::spawn(async move {
        loop {
            for m in first_modes.iter().cycle().take(4) {
                mode_writer.set(*m);
                time.delay_for_measure(Measure(1, 1)).await;
            }
            for m in second_modes.iter().cycle().take(4) {
                mode_writer.set(*m);
                time.delay_for_measure(Measure(1, 1)).await;
            }
        }
    });

    mode
}

fn gen_tonic(
    _time: Time,
    mode: &Node<Cell<Mode>>,
    gen_interval: impl Fn() -> Interval,
) -> Node<Cell<Interval>> {
    let tonic = cell(I);
    let mut mode_reader = mode.observe();
    let tonic_writer = tonic.clone();
    let first = [gen_interval(), gen_interval()];
    let second = [gen_interval(), gen_interval()];

    reactor::spawn(async move {
        let mut tonic_iter = first
            .iter()
            .cycle()
            .take(4)
            .chain(second.iter().cycle().take(4))
            .cycle();

        while let Some(mode) = mode_reader.next().await {
            let t = tonic_iter.next().unwrap();
            let value = mode * *t;
            tonic_writer.set(value);
        }
    });

    tonic
}

fn gen_percussion(
    time: Time,
    composition: impl Composition + 'static,
    key: u8,
    gen_beat: impl Fn() -> Beat,
) {
    let mut durations = gen_beats(time.time_signature.read().total_beats(), gen_beat);

    let rest = if gen() { durations.pop() } else { None };

    let velocities: Vec<_> = durations
        .iter()
        .map(|_| Velocity::new(gen_range(75..127)).unwrap())
        .collect();

    let channel = Channel::C10;
    let key = Key::new(key).unwrap();

    reactor::spawn(async move {
        loop {
            if let Some(rest) = rest {
                time.delay_for_beat(rest).await;
            }
            for (duration, velocity) in durations.iter().cloned().zip(velocities.iter().cloned()) {
                composition.send(MIDIMessage::NoteOn {
                    channel,
                    key,
                    velocity,
                });
                time.delay_for_beat(duration).await;
                composition.send(MIDIMessage::NoteOff {
                    channel,
                    key,
                    velocity,
                });
            }
        }
    });
}

fn gen_melody(
    time: Time,
    tonic: impl Readable<Output = Interval> + 'static,
    mode: impl Readable<Output = Mode> + 'static,
    composition: impl Composition + 'static,
    channel: Channel,
    gen_beat: impl Fn() -> Beat,
    gen_interval: impl Fn() -> Interval,
    gen_velocity: impl Fn() -> Velocity,
) {
    let beats = gen_beats(time.time_signature.read().total_beats(), gen_beat);
    let intervals: Vec<_> = beats.iter().map(move |_| gen_interval()).collect();
    let velocities: Vec<_> = beats.iter().map(|_| gen_velocity()).collect();

    let mut beats2 = beats.clone();
    swap_count(&mut beats2, gen_range(0..3));

    let mut intervals2 = intervals.clone();
    swap_count(&mut intervals2, gen_range(0..3));

    let mut velocities2 = velocities.clone();
    swap_count(&mut velocities2, gen_range(0..3));

    let mut prev = None;

    let mut play = move |note: Interval, velocity| {
        if let Some(prev) = prev.take() {
            composition.send(MIDIMessage::NoteOff {
                channel,
                key: Key::new(prev).unwrap(),
                velocity,
            });
        }

        let note = tonic.read() + mode.read() * note;
        let note = (note * 12u8 + 21u8).whole() as u8;

        if velocity == Velocity::new(0).unwrap() {
            return;
        }

        composition.send(MIDIMessage::NoteOn {
            channel,
            key: Key::new(note).unwrap(),
            velocity,
        });

        prev = Some(note);
    };

    reactor::spawn(async move {
        loop {
            for ((interval, duration), velocity) in
                intervals.iter().zip(beats.iter()).zip(velocities.iter())
            {
                play(*interval, *velocity);
                time.delay_for_beat(*duration).await;
            }
            for ((interval, duration), velocity) in
                intervals2.iter().zip(beats2.iter()).zip(velocities2.iter())
            {
                play(*interval, *velocity);
                time.delay_for_beat(*duration).await;
            }
        }
    });
}

fn gen_beats(mut remaining_time: Beat, gen_beat: impl Fn() -> Beat) -> Vec<Beat> {
    let mut durations = vec![];

    loop {
        let duration = gen_beat();
        if remaining_time > duration {
            durations.push(duration);
            remaining_time -= duration;
        } else {
            durations.push(remaining_time);
            break;
        }
    }

    durations
}
