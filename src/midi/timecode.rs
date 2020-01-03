use crate::midi::integer::u7;

midi_value!(MIDITimeCodeQuarterFrame, u7);

impl MIDITimeCodeQuarterFrame {
    // TODO enum?
    pub fn message_type(&self) -> u8 {
        *self.0 & 0b1111_0000 >> 4
    }

    pub fn values(&self) -> u8 {
        *self.0 & 0b0000_1111
    }
}
