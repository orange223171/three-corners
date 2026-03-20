//! Содержит определения для работы с различными эффектами строений и синергий

use serde::{Deserialize, Serialize};

use crate::effect_info::{
    buff_neighbors::BuffNeighborsInfo, buff_synergistic::BuffSynergisticInfo,
    debuff_neighbors::DebuffNeighborsInfo, merge_synergistic::MergeSynergisticInfo,
};

pub mod buff_neighbors;
pub mod buff_synergistic;
pub mod debuff_neighbors;
pub mod merge_synergistic;

/// Информация об эффекте
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EffectInfo {
    /// Объединение синергирующих строений в одно, используются новые характеристики для всех шести строений одновременно
    MergeSynergistic(MergeSynergisticInfo),
    /// Увеличение характеристик соседних дружественных строений
    BuffNeighbors(BuffNeighborsInfo),
    /// Увеличение характеристик синергирующих строений
    BuffSynergistic(BuffSynergisticInfo),
    /// Уменьшение характеристик соседних строений соперника
    DebuffNeighbors(DebuffNeighborsInfo),
}
