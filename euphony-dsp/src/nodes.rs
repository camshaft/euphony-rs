#![deny(unreachable_patterns)]

#[rustfmt::skip]
use euphony_node::{BoxProcessor, Error, ParameterValue as Value};

#[rustfmt::skip]
pub fn load(processor: u64) -> Option<BoxProcessor> {
    match processor {
        100 => Some(crate::osc::Sine::spawn()),
        101 => Some(crate::osc::SineFast::spawn()),
        102 => Some(crate::osc::SineFaster::spawn()),
        103 => Some(crate::osc::Pulse::spawn()),
        104 => Some(crate::osc::Sawtooth::spawn()),
        105 => Some(crate::osc::Triangle::spawn()),
        106 => Some(crate::osc::Silence::spawn()),
        107 => Some(crate::osc::Phase::spawn()),
        200 => Some(crate::math::Add::spawn()),
        201 => Some(crate::math::Mul::spawn()),
        _ => None,
    }
}

#[rustfmt::skip]
pub fn name(processor: u64) -> Option<&'static str> {
    match processor {
        100 => Some("Sine"),
        101 => Some("SineFast"),
        102 => Some("SineFaster"),
        103 => Some("Pulse"),
        104 => Some("Sawtooth"),
        105 => Some("Triangle"),
        106 => Some("Silence"),
        107 => Some("Phase"),
        200 => Some("Add"),
        201 => Some("Mul"),
        _ => None,
    }
}

#[rustfmt::skip]
pub fn validate_parameter(processor: u64, parameter: u64, value: Value) -> Result<(), Error> {
    match processor {
        100 => crate::osc::Sine::validate_parameter(parameter, value),
        101 => crate::osc::SineFast::validate_parameter(parameter, value),
        102 => crate::osc::SineFaster::validate_parameter(parameter, value),
        103 => crate::osc::Pulse::validate_parameter(parameter, value),
        104 => crate::osc::Sawtooth::validate_parameter(parameter, value),
        105 => crate::osc::Triangle::validate_parameter(parameter, value),
        106 => crate::osc::Silence::validate_parameter(parameter, value),
        107 => crate::osc::Phase::validate_parameter(parameter, value),
        200 => crate::math::Add::validate_parameter(parameter, value),
        201 => crate::math::Mul::validate_parameter(parameter, value),
        _ => unreachable!("processor ({}) param ({}) doesn't exist", processor, parameter)
    }
}
