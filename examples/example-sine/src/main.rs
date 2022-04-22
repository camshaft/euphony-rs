use euphony::prelude::*;

#[euphony::main]
async fn main() {
    let freq = &osc::sine().with_frequency(0.01);

    let sine = osc::sine()
        .with_frequency(freq.mul_add(100.0, 445.0))
        .sink();

    for i in 0..1024 {
        Beat(1, 8).delay().await;
        freq.set_frequency(0.01 + (i as f64) / 200.0);
    }

    sine.fin();
}
