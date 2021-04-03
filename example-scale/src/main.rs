use euphony::{pitch::mode::western::*, prelude::*};

include_synthdef!("../euphony-sc-core/artifacts/v2.scsyndef" as sine);

macro_rules! p {
    ($($t:tt)*) => {
        // TODO
    };
}

#[euphony::main]
async fn main() {
    let a = p![1, [2, 4], [3, 5, 5, 5]];

    let mut notes: Vec<_> = (1..=8).map(|i| Interval(i, 8)).collect();
    notes.extend((1..8).map(|i| Interval(i, 8)).rev());

    let main = track("main");

    let mut tempo = Tempo(120, 1);
    let mut tonic = Interval(4, 1);
    let mut pan = 1;

    for mode in 0..7 {
        let mode = MAJOR >> mode;
        for note in notes.iter().copied() {
            let lower: Interval = (mode * note + tonic) * 12;
            let lower = lower.whole() as i32;
            let lower = sine::new().note(lower).pan(1000 * pan);
            let lower = main.send(lower);

            Beat(1, 2).delay().await;

            let upper: Interval = (mode * (note + III) + tonic) * 12;
            let upper = upper.whole() as i32;
            let upper = sine::new().note(upper).pan(-1000 * pan);
            let upper = main.send(upper);

            Beat(1, 2).delay().await;
            drop(upper);

            drop(lower);

            pan *= -1;

            tempo += 1;
            scheduler().set_tempo(tempo);
        }

        Beat(1, 2).delay().await;

        for note in [I, III, V, Interval(1, 1), V, III, I].iter().copied() {
            let lower: Interval = (mode * note + Interval(4, 1)) * 12;
            let lower = lower.whole();
            let lower = sine::new().note(lower as i32);
            let lower = main.send(lower);
            Beat(1, 2).delay().await;
            drop(lower);
        }

        Beat(1, 2).delay().await;
        tonic += Interval(1, 12);
    }
}
