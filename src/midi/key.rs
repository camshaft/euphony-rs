use crate::midi::integer::u7;

midi_value!(Key, u7);

// impl MIDIValue for Interval {
//     fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
//         // 21 == A0
//         Ok(Interval::new(buffer.decode::<u8>()? as i8 - 21) / 12)
//     }

//     fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<usize, EncoderError> {
//         let note = (*self * 12u8 + 21u8).whole() as u8;
//         buffer.write_byte(note & 127)
//     }
// }

#[test]
fn key_test() {
    assert!(Key::new(127).is_some());
    assert!(Key::new(128).is_none());
}
