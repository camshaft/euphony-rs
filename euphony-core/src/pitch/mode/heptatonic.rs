//! https://en.wikipedia.org/wiki/Heptatonic_scale

use crate::pitch::mode::Mode;
use euphony_core_macros::mode_system;

named_interval!(I(0, 1));
named_interval!(FIRST(0, 1));
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
mode_system!(pub PRIMA = [2, 1, 2, 2, 1, 2, 2]);

named_mode!(AEOLIAN(0, PRIMA));
named_mode!(LOCRIAN(1, PRIMA));
named_mode!(IONIAN(2, PRIMA));
named_mode!(DORIAN(3, PRIMA));
named_mode!(PHRYGIAN(4, PRIMA));
named_mode!(LYDIAN(5, PRIMA));
named_mode!(MIXOLYDIAN(6, PRIMA));

pub const MINOR: Mode = AEOLIAN;
pub const MAJOR: Mode = IONIAN;

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_secunda
mode_system!(pub SECUNDA = [2, 1, 2, 2, 2, 2, 1]);

named_mode!(MELODIC_ASCENDING_MINOR(0, SECUNDA));
named_mode!(PHRYGIAN_RAISED_SIXTH(1, SECUNDA));
named_mode!(LYDIAN_RAISED_FIFTH(2, SECUNDA));
named_mode!(ACOUSTIC(3, SECUNDA));
named_mode!(MAJOR_MINOR(4, SECUNDA));
named_mode!(HALF_DIMINISHED(5, SECUNDA));
named_mode!(ALTERED(6, SECUNDA));

// https://en.wikipedia.org/wiki/Heptatonic_scale#Heptatonia_tertia
mode_system!(pub TERTIA = [1, 2, 2, 2, 2, 2, 1]);

// https://en.wikipedia.org/wiki/Double_harmonic_scale
mode_system!(pub DOUBLE_HARMONIC = [1, 3, 1, 2, 1, 3, 1]);
