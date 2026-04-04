//! Info of buff to synergistic buildings definitions

use serde::{Deserialize, Serialize};

/// An info of buff to synergistic buildings
#[derive(Serialize, Deserialize)]
pub struct BuffSynergisticInfo {
    /// Delta of destroy price
    pub destroy_price_effect: i32,
    /// Delta of grab price
    pub grab_price_effect: i32,

    /// Delta of economic profit
    pub economic_profit_effect: i32,
    /// Delta of politic profit
    pub politic_profit_effect: i32,
    /// Delta of authority profit
    pub authority_profit_effect: i32,

    /// Delta of grab number of economic
    pub economic_grab_n_effect: i32,
    /// Delta of grab number of politic
    pub politic_grab_n_effect: i32,
    /// Delta of grab number of authority
    pub authority_grab_n_effect: i32,
}
