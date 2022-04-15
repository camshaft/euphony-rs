use euphony::{prelude::*, synth::Definition as Def};

static SYNTH_A: Def = Def::new(|| Default::default());

fn main() {
    euphony_testing::start(test());
}

async fn test() {
    // make sure we have a scope in the root
    let s = SYNTH_A.spawn();
    for _ in 0..5 {
        s.set(rand::gen(), now().as_f64());
        Beat(1, 2).delay().await;
    }
    drop(s);

    // make sure spawned tasks have scopes
    async {
        let s = SYNTH_A.spawn();
        for _ in 0..5 {
            s.set(rand::gen(), now().as_f64());
            Beat(1, 2).delay().await;
        }
    }
    .spawn()
    .await;

    // make sure we can seed tasks
    async {
        let s = SYNTH_A.spawn();
        for _ in 0..5 {
            s.set(rand::gen(), now().as_f64());
            Beat(1, 2).delay().await;
        }
    }
    .seed(0)
    .spawn()
    .await;
}
