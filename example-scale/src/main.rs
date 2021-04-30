use euphony::pitch::mode::western::*;
euphony::prelude!();

synthdef!(
    fn organ(out: f32<0>, freq: f32<440.0>, amp: f32<0.5>, pan: f32<0.0>) {
        let detune = [0.98, 0.99, 1.00, 1.01, 1.02];
        let freq = freq * detune;
        let signal = SinOsc::new().freq(freq).ar();
        let signal = Splay::new(signal).center(pan).ar() * amp;
        Out::new(out, signal).ar()
    }
);

synthdef!(
    fn bigsynth(out: f32<0>, freq: f32<440.0>, amp: f32<0.5>, pan: f32<0.0>, stutter: f32<0.1>) {
        let detune = [0.9999, 1.0, 1.0001];
        let freq = freq * detune;
        let width = LFPar::new().freq(stutter).iphase(1).kr();
        let signal = Pulse::new().freq(freq).width(width).ar();

        let signal = Splay::new(signal).center(pan).ar() * amp;

        Out::new(out, signal).ar()
    }
);

fn sustain<T: 'static + Send>(value: T, beats: Beat) {
    async move {
        beats.delay().await;
        drop(value);
    }
    .spawn();
}

#[euphony::main]
async fn main() {
    let mut notes: Vec<_> = (1..=8).map(|i| Interval(i, 8)).collect();
    notes.extend((1..8).map(|i| Interval(i, 8)).rev());

    let tempo = Tempo(50, 1);
    scheduler().set_tempo(tempo);

    let drums = drums().spawn();

    bass().spawn();
    melody().await;

    drums.cancel();
}

async fn drums() {
    let t = track("drums");

    let mut measures = 0;

    loop {
        if measures % 4 == 0 {
            t.send(assets::cy);
        }
        measures += 1;

        t.send(assets::bd[1]);

        for i in 0usize..6 {
            t.send(assets::hh[i]);

            if i == 3 {
                t.send(assets::sd[3]);
            }

            if measures % 4 == 3 && i == 5 {
                fill().spawn();
            }

            Beat(1, 4).delay().await;
        }
    }
}

async fn fill() {
    let t = track("drums");

    Beat(1, 1).delay().await;

    let toms = [assets::ht, assets::mt, assets::lt];

    for tom in toms.iter().copied() {
        for i in (0..2).rev() {
            t.send(tom[i + 2]);
            Beat(1, 8).delay().await;
        }
    }
}

async fn melody() {
    let t = track("melody");

    let tonic = Interval(4, 1);

    for (offset, mode) in structure().take(32) {
        let tonic = tonic + offset;

        for note in [I, III, V, Interval(1, 1), V, III].iter().copied() {
            // let note = midi(note + tonic, mode);
            let note = to_freq(note + tonic, mode);

            let note = organ().freq(note).amp(0.1);

            sustain(t.send(note), Beat(1, 2));
            Beat(1, 4).delay().await;
        }
    }
}

async fn bass() {
    let t = track("bass");

    let tonic = Interval(3, 1);

    let stutter_len = 16;
    let stutter = 0..stutter_len;
    let mut stutter = stutter
        .clone()
        .chain(stutter.rev())
        .map(|v| 0.15 - (v as f32 / stutter_len as f32) * 0.1)
        .cycle();

    for (offset, mode) in structure().take(32) {
        let note = to_freq(offset + tonic, mode);

        for delay in [5, 1].iter().copied() {
            let n = bigsynth()
                .freq(note)
                .amp(0.1)
                .stutter(stutter.next().unwrap());

            sustain(t.send(n), Beat(1, 4));
            Beat(delay, 4).delay().await;
        }
    }
}

fn structure() -> impl Iterator<Item = (Interval, euphony::pitch::mode::Mode)> {
    let tonics = [0, 0, 3, 2, 1, 0, -2, -1]
        .iter()
        .cycle()
        .copied()
        .map(|i| Interval(i, 7));
    let modes = [MAJOR, DORIAN, MINOR].iter().cycle().copied();
    tonics.zip(modes)
}

fn midi(interval: Interval, mode: euphony::pitch::mode::Mode) -> i32 {
    let note: Interval = (mode * interval) * 12;
    let note = note.whole();
    note as i32
}

fn to_freq(interval: Interval, mode: euphony::pitch::mode::Mode) -> f32 {
    /*
    let note: Interval = (mode * interval);
    let note: f32 = note.into();
    440.0
    */
    let note = midi(interval, mode) as f32;
    2f32.powf((note - 69f32) / 12f32) * 440.0
}
