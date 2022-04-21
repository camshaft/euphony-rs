#[deny(unreachable_patterns)]
pub fn load(processor: u64, sample_offset: u64, id: u64) -> Option<::euphony_node::BoxProcessor> {
    match processor {
        100 => Some(crate::osc::Sine::spawn(sample_offset, id)),
        101 => Some(crate::osc::SineFast::spawn(sample_offset, id)),
        102 => Some(crate::osc::SineFaster::spawn(sample_offset, id)),
        103 => Some(crate::osc::Pulse::spawn(sample_offset, id)),
        104 => Some(crate::osc::Sawtooth::spawn(sample_offset, id)),
        105 => Some(crate::osc::Triangle::spawn(sample_offset, id)),
        106 => Some(crate::osc::Silence::spawn(sample_offset, id)),
        107 => Some(crate::osc::Phase::spawn(sample_offset, id)),
        200 => Some(crate::math::Add::spawn(sample_offset, id)),
        201 => Some(crate::math::Mul::spawn(sample_offset, id)),
        _ => None,
    }
}