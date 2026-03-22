//! Содержит определения для работы с синергиями

use serde::{Deserialize, Serialize};

use crate::info::effect::EffectInfo;

/// Информация о синергии
#[derive(Serialize, Deserialize)]
pub struct SynergyInfo {
    name: String,
    buildings: [String; 6],

    effects: Vec<EffectInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_building_info() {
        let json: String = std::fs::read_to_string("data/synergies/farm.json").unwrap();
        let _synergy_info: SynergyInfo = serde_json::from_str(json.as_str()).unwrap();

        assert!(true);
    }
}
