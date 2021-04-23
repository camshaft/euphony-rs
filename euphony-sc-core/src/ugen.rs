#![allow(dead_code)]

pub struct UGen {
    name: &'static str,
    rate: Rate,
}

#[derive(Clone, Copy, Debug)]
pub enum Rate {
    Scalar = 0,
    Control = 1,
    Audio = 2,
}
