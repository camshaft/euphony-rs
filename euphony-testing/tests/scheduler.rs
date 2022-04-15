use euphony::{prelude::*, runtime::time::now, synth::Definition as Def};

static SYNTH_A: Def = Def::new(|| Default::default());
static SYNTH_B: Def = Def::new(|| Default::default());

fn main() {
    euphony_testing::start(test())
}

async fn test() {
    async {
        for i in 0..10 {
            let s = SYNTH_A.spawn();
            s.set(i, now().as_f64());
            Beat(1, 2).delay().await;
        }
    }
    .spawn_primary();

    async {
        for i in 0..10 {
            let s = SYNTH_B.spawn();
            s.set(i, now().as_f64());
            Beat(1, 4).delay().await;
        }
    }
    .spawn_primary();
}
