use crate::pitch::interval::Interval;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct Chord {
    pub chord_system: ChordSystem,
    pub position: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct ChordSystem(&'static [Interval]);
