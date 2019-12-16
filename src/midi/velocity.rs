use num_rational::Ratio;

midi_value!(Velocity, u8);

impl Velocity {
    pub fn as_ratio(self) -> Ratio<u8> {
        Ratio::new(self.0, Self::MAX.0)
    }
}
