//! Содержит определения для работы с синергиями

use crate::effect_info::EffectInfo;

/// Информация о синергии
pub struct SynergyInfo {
    name: String,
    buildings: [String; 6],

    effects_info: Vec<EffectInfo>,
}
