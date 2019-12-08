//! https://en.wikipedia.org/wiki/Heptatonic_scale

use crate::pitch::mode::{system::ModeSystem, Mode};

named_interval!(I(0, 7));
named_interval!(FIRST(0, 7));
named_interval!(II(1, 7));
named_interval!(SECOND(1, 7));
named_interval!(III(2, 7));
named_interval!(THIRD(2, 7));
named_interval!(IV(3, 7));
named_interval!(FOURTH(3, 7));
named_interval!(V(4, 7));
named_interval!(FIFTH(4, 7));
named_interval!(VI(5, 7));
named_interval!(SIXTH(5, 7));
named_interval!(VII(6, 7));
named_interval!(SEVENTH(6, 7));

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_prima
pub const PRIMA: ModeSystem = mode_system_rotation!(2, 1, 2, 2, 1, 2, 2);

named_mode!(AEOLIAN(0, PRIMA));
named_mode!(LOCRIAN(1, PRIMA));
named_mode!(IONIAN(2, PRIMA));
named_mode!(DORIAN(3, PRIMA));
named_mode!(PHRYGIAN(4, PRIMA));
named_mode!(LYDIAN(5, PRIMA));
named_mode!(MIXOLYDIAN(6, PRIMA));

pub const MAJOR: Mode = IONIAN;
pub const MINOR: Mode = AEOLIAN;

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_secunda
pub const SECUNDA: ModeSystem = mode_system_rotation!(2, 1, 2, 2, 2, 2, 1);

named_mode!(MELODIC_ASCENDING_MINOR(0, SECUNDA));
named_mode!(PHRYGIAN_RAISED_SIXTH(1, SECUNDA));
named_mode!(LYDIAN_RAISED_FIFTH(2, SECUNDA));
named_mode!(ACOUSTIC(3, SECUNDA));
named_mode!(MAJOR_MINOR(4, SECUNDA));
named_mode!(HALF_DIMINISHED(5, SECUNDA));
named_mode!(ALTERED(6, SECUNDA));

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_tertia
pub const TERTIA: ModeSystem = mode_system_rotation!(1, 2, 2, 2, 2, 2, 1);

// https://en.wikipedia.org/wiki/Double_harmonic_scale
pub const DOUBLE_HARMONIC: ModeSystem = mode_system_rotation!(1, 3, 1, 2, 1, 3, 1);
