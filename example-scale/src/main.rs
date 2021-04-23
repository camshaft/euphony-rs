use euphony::pitch::mode::western::*;
euphony::prelude!();

fn sustain<T: 'static + Send>(value: T) {
    async move {
        Beat(4, 1).delay().await;
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

    let tonic = Interval(5, 1);

    for (offset, mode) in structure().take(32) {
        let tonic = tonic + offset;

        for note in [I, III, V, Interval(1, 1), V, III].iter().copied() {
            let note = midi(note + tonic, mode);

            let note = assets::chiplead().note(note).amp(0.3);

            sustain(t.send(note));
            Beat(1, 4).delay().await;
        }
    }
}

async fn bass() {
    let t = track("bass");

    let tonic = Interval(3, 1);

    for (offset, mode) in structure().take(32) {
        let note = midi(tonic + offset, mode);

        for delay in [5, 1].iter().copied() {
            let n = assets::bass().note(note as i32).sustain(0.3).amp(0.2);

            sustain(t.send(n));
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