use euphony_macros::mode_system;

// https://en.wikipedia.org/wiki/Pentatonic_scale#The_pentatonic_scales_found_by_running_up_the_keys_C,_D,_E,_G_and_A
mode_system!(pub PRIMA = [2, 2, 3, 2, 3]);

named_mode!(MAJOR(0, PRIMA));
named_mode!(EGYPTIAN(1, PRIMA));
named_mode!(BLUES_MINOR(2, PRIMA));
named_mode!(BLUES_MAJOR(3, PRIMA));
named_mode!(MINOR(4, PRIMA));
