//! Buillding info definitions
//!
//! Definitions of building instance contains module [crate::building]

use serde::{Deserialize, Serialize};

use crate::info::effect::EffectInfo;

/// Building info
#[derive(Serialize, Deserialize)]
pub struct BuildingInfo {
    /// A name of the building
    name: String,

    /// A base build price of the building
    base_build_price: u16,
    /// A base destoy price of the building
    base_destroy_price: u16,
    /// A base grab price of the building
    base_grab_price: u16,

    /// A base economic profit of the building
    base_economic_profit: u16,
    /// A base politic profit of the building
    base_politic_profit: u16,
    /// A base authority profit of the building
    base_authority_profit: u16,

    /// A base grab number of economic of the building
    base_economic_grab_n: u16,
    /// A base grab number of politic of the building
    base_politic_grab_n: u16,
    /// A base grab number of authority of the building
    base_authority_grab_n: u16,

    /// A minimal build price of the building
    minimum_build_price: u16,
    /// A minimal destoy price of the building
    minimum_destroy_price: u16,
    /// A minimal grab price of the building
    minimum_grab_price: u16,

    /// A minimal economic profit of the building
    minimum_economic_profit: u16,
    /// A minimal politic profit of the building
    minimum_politic_profit: u16,
    /// A minimal authority profit of the building
    minimum_authority_profit: u16,

    /// A minimal grab number of economic of the building
    minimum_economic_grab_n: u16,
    /// A minimal grab number of politic of the building
    minimum_politic_grab_n: u16,
    /// A minimal grab number of authority of the building
    minimum_authority_grab_n: u16,

    /// Building's effects
    effects: Vec<EffectInfo>,
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn deserialize_building_info() {
        let json: String = std::fs::read_to_string("data/buildings/townhall.json").unwrap();
        let _building_info: BuildingInfo = serde_json::from_str(json.as_str()).unwrap();

        assert!(true);
    }

    #[test]
    fn deserialize_building_info_with_effect() {
        let json: String = std::fs::read_to_string("data/buildings/windmill.json").unwrap();
        let _building_info: BuildingInfo = serde_json::from_str(json.as_str()).unwrap();

        assert!(true);
    }
}
