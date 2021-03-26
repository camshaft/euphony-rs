//! https://en.wikipedia.org/wiki/Ditonic_scale

use euphony_core_macros::mode_system;

mode_system!(ONE_FIVE = [1, 5]);
named_mode!(VIETNAMESE(0, ONE_FIVE));
named_mode!(WARAO(1, ONE_FIVE));

// https://en.wikipedia.org/wiki/Shamisen#Tuning
mode_system!(SHAMISEN = [5, 7]);
named_mode!(HONCHOSHI(0, SHAMISEN));
named_mode!(NI_AGARI(1, SHAMISEN));
