use super::*;

ugen!(
    #[rates = [ar, kr]]
    #[output = Value]
    struct Dust {
        /// Average number of impulses per second
        #[default = 0.0]
        density: ValueVec,
    }
);

ugen!(
    #[rates = [ar, kr]]
    #[output = Value]
    struct WhiteNoise {}
);
