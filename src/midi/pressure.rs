use num_rational::Ratio;

midi_value!(Pressure, u8);

impl Pressure {
    pub fn as_ratio(self) -> Ratio<u8> {
        Ratio::new(self.0, Self::MAX.0)
    }
}
