use core::f32::consts::TAU;
use fastapprox::{fast, faster};

phased_generator!(sine, Sine, |phase: f32| (TAU * phase).sin());
phased_generator!(sine_fast, SineFast, |phase: f32| fast::sinfull(TAU * phase));
phased_generator!(sine_faster, SineFaster, |phase: f32| faster::sinfull(
    TAU * phase
));

#[cfg(test)]
mod tests {
    generator_test!(sine_440, |buffer| {
        let mut sine = sine(440.0f32);
        sine.fill::<TestBatch>(buffer);
    });

    generator_test!(sine_fm, |buffer| {
        let carrier = sine::<f32>(60.0).mul_signal(200.0f32).add_signal(440.0f32);
        let mut sine = sine(carrier);
        sine.fill::<TestBatch>(buffer);
    });

    generator_test!(sine_fast_440, |buffer| {
        let mut sine = sine_fast(440.0f32);
        sine.fill::<TestBatch>(buffer);
    });

    generator_test!(sine_fast_fm, |buffer| {
        let carrier = sine::<f32>(60.0).mul_signal(200.0f32).add_signal(440.0f32);
        let mut sine = sine_fast(carrier);
        sine.fill::<TestBatch>(buffer);
    });

    generator_test!(sine_faster_440, |buffer| {
        let mut sine = sine_faster(440.0f32);
        sine.fill::<TestBatch>(buffer);
    });

    generator_test!(sine_faster_fm, |buffer| {
        let carrier = sine::<f32>(60.0).mul_signal(200.0f32).add_signal(440.0f32);
        let mut sine = sine_faster(carrier);
        sine.fill::<TestBatch>(buffer);
    });
}
