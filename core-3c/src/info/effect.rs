//! Effect info definitions

use serde::{Deserialize, Serialize};

use crate::info::effect::{
    buff_friend_neighbors::BuffFriendNeighborsInfo,
    buff_opponent_neighbors::BuffOpponentNeighborsInfo, buff_synergistic::BuffSynergisticInfo,
    merge_synergistic::MergeSynergisticInfo,
};

pub mod buff_friend_neighbors;
pub mod buff_opponent_neighbors;
pub mod buff_synergistic;
pub mod merge_synergistic;

/// An effect info
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectInfo {
    /// Represents a merge synergistic buildings into one
    MergeSynergistic(MergeSynergisticInfo),
    /// Represents a buff to synergistic buildings
    BuffSynergistic(BuffSynergisticInfo),
    /// Represents a buff to friend neighbors buildings
    BuffFriendNeighbors(BuffFriendNeighborsInfo),
    /// Represents a buff to neighbors buildings of opponent
    BuffOpponentNeighbors(BuffOpponentNeighborsInfo),
}
