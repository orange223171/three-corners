//! Info of buff to friend neighbors buildings definitions

use serde::{Deserialize, Serialize};

/// An info of buff to friend neighbors buildings
#[derive(Serialize, Deserialize)]
pub struct BuffFriendNeighborsInfo {
    /// Delta of build price
    build_price_effect: i16,
    /// Delta of destroy price
    destroy_price_effect: i16,
    /// Delta of grab price
    grab_price_effect: i16,

    /// Delta of economic profit
    economic_profit_effect: i16,
    /// Delta of politic profit
    politic_profit_effect: i16,
    /// Delta of authority profit
    authority_profit_effect: i16,

    /// Delta of grab number of economic
    economic_grab_n_effect: i16,
    /// Delta of grab number of politic
    politic_grab_n_effect: i16,
    /// Delta of grab number of authority
    authority_grab_n_effect: i16,
}
