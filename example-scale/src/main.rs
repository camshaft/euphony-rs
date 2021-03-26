use euphony::{pitch::mode::western::*, prelude::*};

include_synthdef!("../euphony-sc-core/artifacts/v2.scsyndef" as sine);

#[euphony::main]
async fn main() {
    let mut notes: Vec<_> = (1..=8).map(|i| Interval(i, 8)).collect();
    notes.extend((1..8).map(|i| Interval(i, 8)).rev());

    let main = track("main");

    let mut tempo = Tempo(120, 1);
    let mut pan = 1;

    for scale in [MAJOR, DORIAN, LOCRIAN, MIXOLYDIAN].iter().copied() {
        for note in notes.iter().copied() {
            let lower: Interval = (scale * note + Interval(4, 1)) * 12;
            let lower = lower.whole();
            let lower = sine::new().note(lower as i32).pan(1000 * pan);
            let lower = main.send(lower);

            Beat(1, 4).delay().await;

            let upper: Interval = (scale * (note + III) + Interval(5, 1)) * 12;
            let upper = upper.whole();
            let upper = sine::new().note(upper as i32).pan(-1000 * pan);
            let upper = main.send(upper);

            Beat(1, 2).delay().await;
            drop(lower);

            //Beat(1, 4).delay().await;
            drop(upper);

            pan *= -1;
            tempo += 1;
            scheduler().set_tempo(tempo);
        }

        Beat(1, 2).delay().await;

        for note in [I, III, V, Interval(1, 1), V, III, I].iter().copied() {
            let lower: Interval = (scale * note + Interval(4, 1)) * 12;
            let lower = lower.whole();
            let lower = sine::new().note(lower as i32);
            let lower = main.send(lower);
            Beat(1, 2).delay().await;
        }

        Beat(1, 2).delay().await;
    }
}
