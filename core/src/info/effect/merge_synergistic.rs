//! Info of merge synergistic buildings definitions

use serde::{Deserialize, Serialize};

/// An info of merge synergistic buildings
#[derive(Serialize, Deserialize)]
pub struct MergeSynergisticInfo {
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
}
