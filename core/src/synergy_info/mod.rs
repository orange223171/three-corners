use crate::effect_info::EffectInfo;

pub struct SynergyInfo {
    name: String,
    buildings: [String; 6],

    effects_info: Vec<EffectInfo>,
}
