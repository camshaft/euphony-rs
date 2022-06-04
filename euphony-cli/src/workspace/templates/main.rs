use euphony::prelude::*;

#[euphony::main]
async fn main() {
    set_tempo(Tempo(90, 1));

    score::a(Beat(8, 1)).await;
    score::b(Beat(16, 1)).await;
}

mod score {
    use super::{voices::*, *};

    pub async fn a(duration: Beat) {
        section(duration).with(bd().group("bd")).await
    }

    pub async fn b(duration: Beat) {
        section(duration)
            .with(bd().group("bd"))
            .with(sd().group("sd"))
            .await
    }
}

mod voices {
    use super::*;
    use samples::dirt;

    pub async fn bd() {
        loop {
            dirt::bd.play().spawn_primary();
            delay!(1 / 2);
        }
    }

    pub async fn sd() {
        delay!(1);
        loop {
            dirt::sd.play().spawn_primary();
            delay!(2);
        }
    }
}
