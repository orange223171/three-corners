//! Содержит определения для работы с различными эффектами строений и синергий

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

/// Информация об эффекте
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectInfo {
    /// Объединение синергирующих строений в одно, используются новые характеристики для всех шести строений одновременно
    MergeSynergistic(MergeSynergisticInfo),
    /// Изменение характеристик синергирующих строений
    BuffSynergistic(BuffSynergisticInfo),
    /// Изменение характеристик соседних дружественных строений
    BuffFriendNeighbors(BuffFriendNeighborsInfo),
    /// Изменение характеристик соседних строений соперника
    BuffOpponentNeighbors(BuffOpponentNeighborsInfo),
}
