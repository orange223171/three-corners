//! Содержит определения для работы с синергиями

use serde::{Deserialize, Serialize};

use crate::effect_info::EffectInfo;

mod tests;

/// Информация о синергии
#[derive(Serialize, Deserialize)]
pub struct SynergyInfo {
    name: String,
    buildings: [String; 6],

    effects: Vec<EffectInfo>,
}
