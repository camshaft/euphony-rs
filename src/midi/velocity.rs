use crate::midi::integer::u7;
use num_rational::Ratio;

midi_value!(Velocity, u7);

impl Velocity {
    pub fn as_ratio(self) -> Ratio<u8> {
        Ratio::new(*self.0, *Self::MAX.0)
    }
}
