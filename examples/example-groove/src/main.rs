use euphony::prelude::*;
use samples::dirt;
use western::*;

/// Sets the duration of the composition
const LENGTH: Beat = Beat(16, 1);

#[euphony::main]
async fn main() {
    // spawn a simple bassdrum task
    async {
        loop {
            dirt::bd.play().spawn();
            delay!(2);
        }
    }
    .group("bd")
    .spawn();

    // spawn a simple snaredrum task
    async {
        delay!(1);
        loop {
            dirt::sd.play().spawn();
            delay!(2);
        }
    }
    .group("sd")
    .spawn();

    // generate a set of random rhythms and samples and spawn a task to loop
    async {
        let beats = rand::rhythm(LENGTH, Beat::vec([2, 4]));
        let hats = beats.each(|_| *dirt::glasstap.pick());
        loop {
            let mut hats = (&beats).delays().with(&hats);
            while let Some(hat) = hats.next().await {
                hat.play().spawn();
            }
        }
    }
    .seed(1234)
    .group("hh")
    .spawn();

    // spawn a task for a simple bass line
    async {
        loop {
            for i in [0, 0, 2, 0] {
                bass(Interval(i, 7) - 2).spawn();
                delay!(1 / 2);
            }
        }
    }
    .group("bass")
    .spawn();

    delay!(LENGTH);
}

async fn bass(interval: Interval) {
    let freq = interval * MINOR * ET12;

    let osc = osc::sine().with_frequency(freq);

    let attack = Beat(1, 64);
    let sustain = Beat(1, 4);
    let decay = Beat(1, 16);

    let env = env::linear().with_duration(attack).with_target(0.5);

    let sink = (osc * &env).sink();

    delay!(sustain);

    env.set_duration(decay);
    env.set_target(0.0);

    delay!(decay);

    sink.fin();
}
