phased_generator!(pulse, Pulse, |phase: f32| (0.5 - phase).signum());

#[cfg(test)]
mod tests {
    generator_test!(pulse_440, |buffer| {
        let mut pulse = pulse(440.0f32);
        pulse.fill::<TestBatch>(buffer);
    });

    generator_test!(pulse_fm, |buffer| {
        let carrier = sine::<f32>(20.0).mul_signal(200.0f32).add_signal(440.0f32);
        let mut pulse = pulse(carrier);
        pulse.fill::<TestBatch>(buffer);
    });
}
