use euphony::{prelude::*, synth::Definition as Def};

static SYNTH_A: Def = Def::new(|| Default::default());

fn main() {
    euphony_testing::start(async {
        for i in 0..10 {
            set_tempo(Tempo(60 + i * 10, 1));
            let s = SYNTH_A.spawn();
            s.set(i, now().as_f64());
            Beat(1, 2).delay().await;
        }
    })
}
