midi_value!(Controller, u8);
midi_value!(ControllerValue, u8);

// TODO
// All Sound Off. When All Sound Off is received all oscillators will turn off, and their volume envelopes are set to zero as soon as possible. c = 120, v = 0: All Sound Off
// Reset All Controllers. When Reset All Controllers is received, all controller values are reset to their default values. (See specific Recommended Practices for defaults).
// c = 121, v = x: Value must only be zero unless otherwise allowed in a specific Recommended Practice.
// Local Control. When Local Control is Off, all devices on a given channel will respond only to data received over MIDI. Played data, etc. will be ignored. Local Control On restores the functions of the normal controllers.
// c = 122, v = 0: Local Control Off
// c = 122, v = 127: Local Control On
// All Notes Off. When an All Notes Off is received, all oscillators will turn off.
// c = 123, v = 0: All Notes Off (See text for description of actual mode commands.)
// c = 124, v = 0: Omni Mode Off
// c = 125, v = 0: Omni Mode On
// c = 126, v = M: Mono Mode On (Poly Off) where M is the number of channels (Omni Off) or 0 (Omni On)
// c = 127, v = 0: Poly Mode On (Mono Off) (Note: These four messages also cause All Notes Off)
