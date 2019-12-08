//! https://en.wikipedia.org/wiki/Ditonic_scale

use crate::pitch::mode::system::ModeSystem;

const ONE_FIVE: ModeSystem = mode_system_rotation!(1, 5);
named_mode!(VIETNAMESE(0, ONE_FIVE));
named_mode!(WARAO(1, ONE_FIVE));

/// https://en.wikipedia.org/wiki/Shamisen#Tuning
pub const SHAMISEN: ModeSystem = mode_system_rotation!(5, 7);
named_mode!(HONCHOSHI(0, SHAMISEN));
named_mode!(NI_AGARI(1, SHAMISEN));
