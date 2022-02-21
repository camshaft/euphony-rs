phased_generator!(sawtooth, Sawtooth, |phase: f32| (0.5 - phase) * 2.0);

#[cfg(test)]
mod tests {
    generator_test!(sawtooth_440, |buffer| {
        let mut sawtooth = sawtooth(440.0f32);
        sawtooth.fill::<TestBatch>(buffer);
    });

    generator_test!(sawtooth_fm, |buffer| {
        let carrier = sine::<f32>(20.0).mul_signal(200.0f32).add_signal(440.0f32);
        let mut sawtooth = sawtooth(carrier);
        sawtooth.fill::<TestBatch>(buffer);
    });
}
