use euphony::{
    midi::{
        channel::Channel, codec::MIDIValue, key::Key, message::MIDIMessage, velocity::Velocity,
    },
    pitch::{interval::Interval, mode::western::*},
    runtime::{
        future::reactor,
        graph::{
            cell::cell,
            map::MapCell,
            subscription::{Readable, Subscription},
        },
        online::OnlineRuntime,
        time::delay,
    },
    time::{beat::Beat, measure::Measure, tempo::Tempo, time_signature::TimeSignature},
};
use futures::stream::StreamExt;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    cell::RefCell,
    error::Error,
    io::{stdin, stdout, Write},
    sync::{Arc, Mutex},
};

thread_local! {
    static SEED: u64 = if let Ok(seed) = std::env::var("EUPHONY_SEED") {
        seed.parse().unwrap()
    } else {
        let seed: u64 = rand::thread_rng().gen();
        eprintln!("EUPHONY_SEED={:?}", seed);
        seed
    };
    static RNG: RefCell<StdRng> = RefCell::new(StdRng::seed_from_u64(SEED.with(|s| *s)));
}

fn gen<T>() -> T
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    RNG.with(|r| r.borrow_mut().gen())
}

fn gen_range<T>(lower: T, upper: T) -> T
where
    T: rand::distributions::uniform::SampleUniform,
{
    RNG.with(|r| r.borrow_mut().gen_range(lower, upper))
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Euphony")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    let master_conn = Connection::new(midi_out.connect(out_port, file!())?);

    let mut runtime = OnlineRuntime::new();
    let tempo = Tempo(gen_range(100, 180), 1);
    println!("{:?}", tempo);
    let tempo = cell(tempo);
    let time_signature = TimeSignature(gen_range(3, 8), 4);
    println!("{:?}", time_signature);
    let time_signature = cell(time_signature);

    const ROOT: Interval = Interval(3, 1);

    let mode = cell(MAJOR);
    let mut tonic_mode_reader = mode.observe();
    let melody_mode_reader = mode.clone();
    let bass_mode_reader = mode.clone();
    let tempo_reader = tempo.observe();
    let time_signature_reader = time_signature.observe();
    reactor::spawn(async move {
        let delay = |measure| delay_for_measure(measure, &time_signature_reader, &tempo_reader);

        let print_mode = |mode| match mode {
            MINOR => print!("MINOR"),
            LOCRIAN => print!("LOCRIAN"),
            MAJOR => print!("MAJOR"),
            DORIAN => print!("DORIAN"),
            PHRYGIAN => print!("PHRYGIAN"),
            LYDIAN => print!("LYDIAN"),
            MIXOLYDIAN => print!("MIXOLYDIAN"),
            _ => print!("UNKNOWN"),
        };

        let first_modes = [MAJOR >> gen_range(0, 7), MAJOR >> gen_range(0, 7)];
        let second_modes = [MAJOR >> gen_range(0, 7), MAJOR >> gen_range(0, 7)];

        // print_mode(modes[0]);
        // print!(" -> ");
        // print_mode(modes[1]);
        // println!("");

        loop {
            for m in first_modes.iter().cycle().take(4) {
                mode.set(*m);
                delay(Measure(1, 1)).await;
            }
            for m in second_modes.iter().cycle().take(4) {
                mode.set(*m);
                delay(Measure(1, 1)).await;
            }
        }
    });

    let tonic = cell(MAJOR * I + ROOT);
    let melody_tonic_reader = tonic.clone();
    let bass_tonic_reader = tonic.clone();
    reactor::spawn(async move {
        let first = [Interval(gen_range(0, 7), 7), Interval(gen_range(0, 7), 7)];
        let second = [Interval(gen_range(0, 7), 7), Interval(gen_range(0, 7), 7)];
        let mut tonic_iter = first
            .iter()
            .cycle()
            .take(4)
            .chain(second.iter().cycle().take(4))
            .cycle();

        while let Some(mode) = tonic_mode_reader.next().await {
            let t = tonic_iter.next().unwrap();
            let value = mode * *t + ROOT;
            tonic.set(value);
        }
    });

    let hi_hat_conn = master_conn.clone();
    let tempo_reader = tempo.observe();
    let time_signature_reader = time_signature.observe();
    reactor::spawn(async move {
        let delay = |beat| delay_for_beat(beat, &time_signature_reader, &tempo_reader);

        let mut durations = vec![];
        let mut remaining_time: Beat = time_signature_reader.read().total_beats();

        loop {
            let duration = Beat(1, 16) * gen_range(1, 3);
            if remaining_time > duration {
                durations.push(duration);
                remaining_time -= duration;
            } else {
                durations.push(remaining_time);
                break;
            }
        }

        let channel = Channel::new(2).unwrap();
        let hi_hat = Key::new(0x2a).unwrap();

        loop {
            for duration in durations.iter() {
                hi_hat_conn.send(MIDIMessage::NoteOn {
                    channel,
                    key: hi_hat,
                    velocity: Velocity::new(gen_range(100, 127)).unwrap(),
                });
                delay(*duration).await;
                // hi_hat_conn.send(MIDIMessage::NoteOff {
                //     channel,
                //     key: hi_hat,
                //     velocity,
                // });
            }
        }
    });

    let bass_drum_conn = master_conn.clone();
    let tempo_reader = tempo.observe();
    let time_signature_reader = time_signature.observe();
    reactor::spawn(async move {
        let delay = |beat| delay_for_beat(beat, &time_signature_reader, &tempo_reader);

        let mut durations = vec![];
        let mut remaining_time: Beat = time_signature_reader.read().total_beats();

        loop {
            let duration = Beat(1, 4) * gen_range(1, 4);
            if remaining_time > duration {
                durations.push(duration);
                remaining_time -= duration;
            } else {
                durations.push(remaining_time);
                break;
            }
        }

        let channel = Channel::new(2).unwrap();
        let bass_drum = Key::new(0x20).unwrap();
        let velocity = Velocity::new(127).unwrap();

        loop {
            for duration in durations.iter() {
                bass_drum_conn.send(MIDIMessage::NoteOn {
                    channel,
                    key: bass_drum,
                    velocity,
                });
                delay(*duration).await;
                // bass_drum_conn.send(MIDIMessage::NoteOff {
                //     channel,
                //     key: bass_drum,
                //     velocity,
                // });
            }
        }
    });

    let snare_drum_conn = master_conn.clone();
    let tempo_reader = tempo.observe();
    let time_signature_reader = time_signature.observe();
    reactor::spawn(async move {
        let delay = |beat| delay_for_beat(beat, &time_signature_reader, &tempo_reader);

        let mut durations = vec![];
        let mut remaining_time: Beat = time_signature_reader.read().total_beats();

        loop {
            let duration = Beat(1, 4) * gen_range(1, 4);
            if remaining_time > duration {
                durations.push(duration);
                remaining_time -= duration;
            } else {
                durations.push(remaining_time);
                break;
            }
        }

        let channel = Channel::new(2).unwrap();
        let bass_drum = Key::new(0x26).unwrap();
        let velocity = Velocity::new(127).unwrap();

        loop {
            for duration in durations.iter() {
                delay(*duration).await;
                snare_drum_conn.send(MIDIMessage::NoteOn {
                    channel,
                    key: bass_drum,
                    velocity,
                });
                // snare_drum_conn.send(MIDIMessage::NoteOff {
                //     channel,
                //     key: bass_drum,
                //     velocity,
                // });
            }
        }
    });

    let melody_note = cell(I);
    let melody_note_reader = melody_note.clone();
    let tempo_reader = tempo.observe();
    let time_signature_reader = time_signature.observe();
    reactor::spawn(async move {
        let delay = |beat| delay_for_beat(beat, &time_signature_reader, &tempo_reader);

        let mut intervals = vec![];
        let mut remaining_time: Beat = time_signature_reader.read().total_beats();

        loop {
            let interval = Interval(gen_range(-3, 8), 7);
            let duration = Beat(1, 8) * gen_range(1, 4);
            if remaining_time > duration {
                intervals.push((interval, duration));
                remaining_time -= duration;
            } else {
                intervals.push((interval, remaining_time));
                break;
            }
        }
        println!("MELODY {:?}", intervals);

        loop {
            for (interval, duration) in intervals.iter() {
                melody_note.set(*interval);
                delay(*duration).await;
            }
        }
    });

    let melody = (melody_tonic_reader, melody_mode_reader, melody_note_reader)
        .map(|tonic, mode, melody_note| tonic + mode * melody_note);

    let melody_conn = master_conn.clone();
    reactor::spawn(async move {
        let mut m = melody.observe();

        let channel = Channel::new(0).unwrap();
        let velocity = Velocity::new(127).unwrap();

        let mut prev = None;
        while let Some(note) = m.next().await {
            if let Some(prev) = prev {
                melody_conn.send(MIDIMessage::NoteOff {
                    channel,
                    key: Key::new(prev).unwrap(),
                    velocity,
                });
            }

            let note = (note * 12u8 + 21u8).whole() as u8;
            melody_conn.send(MIDIMessage::NoteOn {
                channel,
                key: Key::new(note).unwrap(),
                velocity,
            });

            prev = Some(note);
        }
    });

    let bass_note = cell(I);
    let bass_note_reader = bass_note.clone();
    let tempo_reader = tempo.observe();
    let time_signature_reader = time_signature.observe();
    reactor::spawn(async move {
        let delay = |beat| delay_for_beat(beat, &time_signature_reader, &tempo_reader);

        let mut intervals = vec![];
        let mut remaining_time: Beat = time_signature_reader.read().total_beats();

        loop {
            let interval = Interval(gen_range(0, 7), 7);
            let duration = Beat(1, 8) * gen_range(1, 4);
            if remaining_time > duration {
                intervals.push((interval, duration));
                remaining_time -= duration;
            } else {
                intervals.push((interval, remaining_time));
                break;
            }
        }
        println!("BASS {:?}", intervals);

        loop {
            for (interval, duration) in intervals.iter() {
                bass_note.set(*interval);
                delay(*duration).await;
            }
        }
    });

    let bass = (bass_tonic_reader, bass_mode_reader, bass_note_reader)
        .map(|tonic, mode, bass_note| tonic - Interval(2, 1) + mode * bass_note);

    let bass_conn = master_conn.clone();
    reactor::spawn(async move {
        let mut m = bass.observe();

        let channel = Channel::new(1).unwrap();
        let velocity = Velocity::new(127).unwrap();

        let mut prev = None;
        while let Some(note) = m.next().await {
            if let Some(prev) = prev {
                bass_conn.send(MIDIMessage::NoteOff {
                    channel,
                    key: Key::new(prev).unwrap(),
                    velocity,
                });
            }

            let note = (note * 12u8 + 21u8).whole() as u8;
            bass_conn.send(MIDIMessage::NoteOn {
                channel,
                key: Key::new(note).unwrap(),
                velocity,
            });

            prev = Some(note);
        }
    });

    ctrlc::set_handler(move || {
        master_conn.reset();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    runtime.render();

    println!("Connection closed");

    Ok(())
}

struct Connection(Arc<Mutex<MidiOutputConnection>>);

impl Clone for Connection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Connection {
    pub fn new(connection: MidiOutputConnection) -> Self {
        Self(Arc::new(Mutex::new(connection)))
    }

    pub fn send(&self, message: MIDIMessage) {
        let mut bytes = vec![];
        message.encode(&mut bytes).unwrap();
        self.send_bytes(&bytes[..])
    }

    pub fn send_bytes(&self, message: &[u8]) {
        let _ = self.0.lock().unwrap().send(message);
    }

    pub fn reset(&self) {
        let velocity = Velocity::new(0).unwrap();
        for channel in 0..16 {
            let channel = Channel::new(channel).unwrap();
            for key in 0..128 {
                self.send(MIDIMessage::NoteOff {
                    channel,
                    velocity,
                    key: Key::new(key).unwrap(),
                });
            }
        }
        self.send(MIDIMessage::Reset);
    }
}

async fn delay_for_beat(
    beat: Beat,
    time_signature: &impl Subscription<Output = TimeSignature>,
    tempo: &impl Subscription<Output = Tempo>,
) {
    let beat: Beat = (beat / time_signature.read().beat()).into();
    let duration = tempo.read() * beat;

    delay::delay_for(duration).await;
}

async fn delay_for_measure(
    measure: Measure,
    time_signature: &impl Subscription<Output = TimeSignature>,
    tempo: &impl Subscription<Output = Tempo>,
) {
    let beats = measure * time_signature.read();
    let duration = tempo.read() * beats;
    delay::delay_for(duration).await;
}
