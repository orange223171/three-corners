//! Synergy info definitions

use serde::{Deserialize, Serialize};

use crate::info::effect::EffectInfo;

/// A synergy info
#[derive(Serialize, Deserialize)]
pub struct SynergyInfo {
    /// A name of the synergy
    pub name: String,
    /// A set of buildings that combine to the synergy
    pub buildings: [String; 6],

    /// Synergy's effects
    pub effects: Vec<EffectInfo>,
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
