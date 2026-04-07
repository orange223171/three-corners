//! Info of merge synergistic buildings definitions

use serde::{Deserialize, Serialize};

/// An info of merge synergistic buildings
#[derive(Serialize, Deserialize)]
pub struct MergeSynergisticInfo {
    /// A base destoy price of the building
    pub base_destroy_price: u32,
    /// A base grab price of the building
    pub base_grab_price: u32,

    /// A base economic profit of the building
    pub base_economic_profit: u32,
    /// A base politic profit of the building
    pub base_politic_profit: u32,
    /// A base authority profit of the building
    pub base_authority_profit: u32,

    /// A base grab number of economic of the building
    pub base_economic_grab_n: u32,
    /// A base grab number of politic of the building
    pub base_politic_grab_n: u32,
    /// A base grab number of authority of the building
    pub base_authority_grab_n: u32,

    /// A minimal destoy price of the building
    pub minimum_destroy_price: u32,
    /// A minimal grab price of the building
    pub minimum_grab_price: u32,

    /// A minimal economic profit of the building
    pub minimum_economic_profit: u32,
    /// A minimal politic profit of the building
    pub minimum_politic_profit: u32,
    /// A minimal authority profit of the building
    pub minimum_authority_profit: u32,

    /// A minimal grab number of economic of the building
    pub minimum_economic_grab_n: u32,
    /// A minimal grab number of politic of the building
    pub minimum_politic_grab_n: u32,
    /// A minimal grab number of authority of the building
    pub minimum_authority_grab_n: u32,
}
