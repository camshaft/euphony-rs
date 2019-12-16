use num_rational::Ratio;

midi_value!(PitchBend, u16);

impl PitchBend {
    pub fn as_ratio(self) -> Ratio<i16> {
        let value = Ratio::new(self.0 as i32, 2i32.pow(14)) * 2 - 1;
        let (n, d) = value.into();
        Ratio::new(n as i16, d as i16)
    }
}

#[test]
fn as_ratio_test() {
    assert_eq!(PitchBend::new(0).unwrap().as_ratio(), Ratio::new(-1, 1));
    assert_eq!(PitchBend::new(8192).unwrap().as_ratio(), Ratio::new(0, 1));
    assert_eq!(
        PitchBend::new(16383).unwrap().as_ratio(),
        Ratio::new(8191, 8192)
    );
}
