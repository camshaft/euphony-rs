#[deny(unreachable_patterns)]
pub fn load(id: u64) -> Option<::euphony_node::BoxProcessor> {
    match id {
        100 => Some(crate::osc::Sine::spawn()),
        101 => Some(crate::osc::SineFast::spawn()),
        102 => Some(crate::osc::SineFaster::spawn()),
        103 => Some(crate::osc::Pulse::spawn()),
        104 => Some(crate::osc::Sawtooth::spawn()),
        105 => Some(crate::osc::Triangle::spawn()),
        106 => Some(crate::osc::Silence::spawn()),
        107 => Some(crate::osc::Phase::spawn()),
        _ => None,
    }
}