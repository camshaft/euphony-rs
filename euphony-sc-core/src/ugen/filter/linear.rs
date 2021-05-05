use super::*;

ugen!(
    #[rates = [ar, kr]]
    #[new(signal: impl Into<ValueVec>)]
    #[output = Value]
    struct Decay {
        /// The input signal
        signal: ValueVec,

        /// 60db decay time in seconds
        #[default = 1.0]
        decay: ValueVec,
    }
);
