use super::*;

ugen!(
    /// Comb delay line with cubic interpolation.
    ///
    /// Comb delay line with cubic interpolation. See also CombNwhich uses no interpolation,
    /// and CombL which uses linear interpolation. Cubic interpolation is more computationally
    /// expensive than linear, but more accurate.
    ///
    /// The feedback coefficient is given by the equation:
    ///
    /// ```text
    /// fb == 0.001 ** (delay / decay.abs) * decay.sign
    /// ```
    ///
    /// where `0.001` is `-60` dBFS.
    #[rates = [ar, kr]]
    #[new(signal: impl Into<ValueVec>)]
    #[output = Value]
    struct CombC {
        /// The input signal
        signal: ValueVec,

        /// The maximum delay time in seconds.
        ///
        /// Used to initialize the delay buffer size.
        #[default = 0.2]
        max_delay: ValueVec,

        /// Delay time in seconds.
        #[default = 0.2]
        delay: ValueVec,

        /// Time for the echoes to decay by 60 decibels.
        ///
        /// If this time is negative then the feedback coefficient will be negative,
        /// thus emphasizing only odd harmonics at an octave lower.
        ///
        /// Large decay times are sensitive to DC bias, so use a LeakDC if this is an issue.
        ///
        /// Infinite decay times are permitted. A decay time of inf leads to a feedback
        /// coefficient of 1, and a decay time of -inf leads to a feedback coefficient of -1.
        #[default = 1.0]
        decay: ValueVec,
    }
);

ugen!(
    #[rates = [ar, kr]]
    #[new(signal: impl Into<ValueVec>)]
    #[output = Value]
    struct CombN {
        /// The input signal
        signal: ValueVec,

        /// The maximum delay time in seconds.
        ///
        /// Used to initialize the delay buffer size.
        #[default = 0.2]
        max_delay: ValueVec,

        /// Delay time in seconds.
        #[default = 0.2]
        delay: ValueVec,

        /// Time for the echoes to decay by 60 decibels.
        ///
        /// If this time is negative then the feedback coefficient will be negative,
        /// thus emphasizing only odd harmonics at an octave lower.
        ///
        /// Large decay times are sensitive to DC bias, so use a LeakDC if this is an issue.
        ///
        /// Infinite decay times are permitted. A decay time of inf leads to a feedback
        /// coefficient of 1, and a decay time of -inf leads to a feedback coefficient of -1.
        #[default = 1.0]
        decay: ValueVec,
    }
);
