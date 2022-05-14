use euphony::prelude::*;

#[euphony::main]
async fn main() {
    let azimuth = &osc::sine().with_frequency(1.0);
    let radius = &osc::triangle().with_frequency(1.0) * 4.0;

    let freq = &osc::sine().with_frequency(0.05);

    let sine = osc::sine()
        .with_frequency(freq.mul_add(100.0, 445.0))
        .mul(0.5)
        .sink();

    sine.set_azimuth(-0.5).set_radius(1.0);

    for i in 0..600 {
        Beat(1, 8).delay().await;
        freq.set_frequency(0.01 + (i as f64) / 2.0);
    }
    for i in (1..600).rev() {
        Beat(1, 16).delay().await;
        freq.set_frequency(0.01 + (i as f64) / 2.0);
    }

    Beat(64, 1).delay().await;

    sine.fin();
}
