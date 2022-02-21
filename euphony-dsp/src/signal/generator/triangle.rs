phased_generator!(triangle, Triangle, |phase: f32| ((0.5 - phase).abs()
    - 0.25)
    * 4.0);

#[cfg(test)]
mod tests {
    generator_test!(triangle_440, |buffer| {
        let mut triangle = triangle(440.0f32);
        triangle.fill::<TestBatch>(buffer);
    });

    generator_test!(triangle_fm, |buffer| {
        let carrier = sine::<f32>(20.0).mul_signal(200.0f32).add_signal(440.0f32);
        let mut triangle = triangle(carrier);
        triangle.fill::<TestBatch>(buffer);
    });
}
