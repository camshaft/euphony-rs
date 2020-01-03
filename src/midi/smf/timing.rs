use crate::{
    midi::{
        codec::{DecoderBuffer, DecoderError, EncoderBuffer, MIDIValue},
        integer::{u15be, u16be},
    },
    time::beat::Beat,
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Timing {
    TicksPerQuarter(u15be),
    Timecode(FPS, u8),
}

impl Timing {
    pub fn from_beat(beat: Beat) -> Option<Self> {
        let ticks = Beat(1, 4) / beat;
        if !ticks.is_whole() {
            return None;
        }
        let ticks = ticks.whole();
        Some(Timing::TicksPerQuarter(u15be::new(ticks)?))
    }

    pub fn ticks_to_beat(&self, ticks: u64) -> Option<Beat> {
        match self {
            Self::TicksPerQuarter(ticks_per_quarter) => {
                Some(Beat(1, 4) / **ticks_per_quarter * ticks)
            }
            _ => None,
        }
    }

    pub fn beat_to_ticks(&self, beat: Beat) -> Option<u64> {
        match self {
            Self::TicksPerQuarter(ticks_per_quarter) => {
                let ticks = beat / Beat(1, 4) * (**ticks_per_quarter as u64);
                ticks.try_into_whole()
            }
            _ => None,
        }
    }
}

#[test]
fn timing_conversion_test() {
    let timing = Timing::from_beat(Beat(1, 64)).unwrap();

    let tests = [
        (Beat(1, 1), 64),
        (Beat(1, 2), 32),
        (Beat(1, 4), 16),
        (Beat(1, 8), 8),
        (Beat(1, 16), 4),
        (Beat(1, 32), 2),
        (Beat(1, 64), 1),
    ];

    for (beat, ticks) in tests.iter().cloned() {
        assert_eq!(ticks, timing.beat_to_ticks(beat).unwrap());
        assert_eq!(beat, timing.ticks_to_beat(ticks).unwrap());
    }
}

impl MIDIValue for Timing {
    fn decode<B: DecoderBuffer>(buffer: &mut B) -> Result<Self, DecoderError> {
        match *buffer.decode::<u16be>()? {
            _v @ 0b1000_0000..=core::u16::MAX => {
                // TODO
                unimplemented!()
            }
            v => Ok(Timing::TicksPerQuarter(u15be::new_lossy(v))),
        }
    }

    fn encode<B: EncoderBuffer>(&self, buffer: &mut B) -> Result<(), B::Error> {
        match self {
            Timing::TicksPerQuarter(ticks) => buffer.encode(ticks),
            Timing::Timecode(_, _) => unimplemented!(),
        }
    }

    fn encoding_len(&self) -> usize {
        2
    }
}

// impl Timing {
//     pub fn read(raw: &mut &[u8]) -> Result<Timing> {
//         let raw = u16::read(raw).context(err_invalid("unexpected eof when reading midi timing"))?;
//         if bit_range(raw, 15..16) != 0 {
//             //Timecode
//             let fps = -(bit_range(raw, 8..16) as i8);
//             let subframe = bit_range(raw, 0..8) as u8;
//             Ok(Timing::Timecode(
//                 FPS::from_int(fps as u8).ok_or(err_invalid("invalid smpte fps"))?,
//                 subframe,
//             ))
//         } else {
//             //Metrical
//             Ok(Timing::Metrical(u15::from(raw)))
//         }
//     }

//     pub fn encode(&self) -> [u8; 2] {
//         match self {
//             Timing::Metrical(ticksperbeat) => ticksperbeat.as_int().to_be_bytes(),
//             Timing::Timecode(framespersec, ticksperframe) => {
//                 [(-(framespersec.as_int() as i8)) as u8, *ticksperframe]
//             }
//         }
//     }
// }

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FPS {
    FPS24,
    FPS25,
    FPS29,
    FPS30,
}

impl FPS {
    // /// Does the conversion from a 2-bit fps code to an `FPS` value.
    // pub fn from_code(code: u2) -> FPS {
    //     match code.as_int() {
    //         0 => FPS::FPS24,
    //         1 => FPS::FPS25,
    //         2 => FPS::FPS29,
    //         3 => FPS::FPS30,
    //         _ => unreachable!(),
    //     }
    // }

    // /// Does the conversion to a 2-bit fps code.
    // pub fn as_code(self) -> u2 {
    //     u2::from(match self {
    //         FPS::FPS24 => 0,
    //         FPS::FPS25 => 1,
    //         FPS::FPS29 => 2,
    //         FPS::FPS30 => 3,
    //     })
    // }

    /// Converts an integer representing the semantic fps to an `FPS` value (ie. `24` -> `FPS24`).
    pub fn from_int(raw: u8) -> Option<FPS> {
        Some(match raw {
            24 => FPS::FPS24,
            25 => FPS::FPS25,
            29 => FPS::FPS29,
            30 => FPS::FPS30,
            _ => return None,
        })
    }

    /// Get the integral approximate fps out.
    pub fn as_int(self) -> u8 {
        match self {
            FPS::FPS24 => 24,
            FPS::FPS25 => 25,
            FPS::FPS29 => 29,
            FPS::FPS30 => 30,
        }
    }

    /// Get the actual f32 fps out.
    pub fn as_f32(self) -> f32 {
        match self.as_int() {
            24 => 24.0,
            25 => 25.0,
            29 => 30.0 / 1.001,
            30 => 30.0,
            _ => unreachable!(),
        }
    }
}
