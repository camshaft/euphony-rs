//! https://en.wikipedia.org/wiki/Heptatonic_scale

use crate::pitch::{
    interval::Interval,
    mode::{system::ModeSystem, Mode},
};

pub const I: Interval = Interval(0, 7);
pub const II: Interval = Interval(1, 7);
pub const III: Interval = Interval(2, 7);
pub const IV: Interval = Interval(3, 7);
pub const V: Interval = Interval(4, 7);
pub const VI: Interval = Interval(5, 7);
pub const VII: Interval = Interval(6, 7);

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_prima
pub const PRIMA: ModeSystem = mode_system_rotation!(2, 1, 2, 2, 1, 2, 2);

pub const AEOLIAN: Mode = Mode(0, PRIMA);
pub const LOCRIAN: Mode = Mode(1, PRIMA);
pub const IONIAN: Mode = Mode(2, PRIMA);
pub const DORIAN: Mode = Mode(3, PRIMA);
pub const PHRYGIAN: Mode = Mode(4, PRIMA);
pub const LYDIAN: Mode = Mode(5, PRIMA);
pub const MIXOLYDIAN: Mode = Mode(6, PRIMA);

pub const MAJOR: Mode = IONIAN;
pub const MINOR: Mode = AEOLIAN;

#[test]
fn shift_test() {
    assert_eq!(MAJOR >> 1, DORIAN);
    assert_eq!(MAJOR << 2, MINOR);
    assert_eq!(MAJOR << 7, MAJOR);
}

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_secunda
pub const SECUNDA: ModeSystem = mode_system_rotation!(2, 1, 2, 2, 2, 2, 1);

pub const MELODIC_ASCENDING_MINOR: Mode = Mode(0, SECUNDA);
pub const PHRYGIAN_RAISED_SIXTH: Mode = Mode(1, SECUNDA);
pub const LYDIAN_RAISED_FIFTH: Mode = Mode(2, SECUNDA);
pub const ACOUSTIC: Mode = Mode(3, SECUNDA);
pub const MAJOR_MINOR: Mode = Mode(4, SECUNDA);
pub const HALF_DIMINISHED: Mode = Mode(5, SECUNDA);
pub const ALTERED: Mode = Mode(6, SECUNDA);

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_tertia
pub const TERTIA: ModeSystem = mode_system_rotation!(1, 2, 2, 2, 2, 2, 1);

// https://en.wikipedia.org/wiki/Double_harmonic_scale
pub const DOUBLE_HARMONIC: ModeSystem = mode_system_rotation!(1, 3, 1, 2, 1, 3, 1);
