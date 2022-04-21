use euphony::prelude::*;

use euphony::synth::Processor;

static SINE: Processor = Processor {
    id: 101,
    name: "Sine",
    inputs: 2,
};

static ADD: Processor = Processor {
    id: 200,
    name: "Add",
    inputs: 2,
};

static MUL: Processor = Processor {
    id: 201,
    name: "Mul",
    inputs: 2,
};

#[euphony::main]
async fn main() {
    let freq = SINE.spawn();
    freq.set(0, 0.01);

    let mul = MUL.spawn();
    mul.set(0, 10.0);
    mul.set(1, &freq);

    let add = ADD.spawn();
    add.set(0, 445.0);
    add.set(1, mul);

    let sine = SINE.spawn();
    sine.set(0, add);

    let sink = sine.sink();

    for i in 0..1024 {
        Beat(1, 8).delay().await;
        freq.set(0, 0.01 + (i as f64) / 100.0);
    }

    drop(sink);
}
