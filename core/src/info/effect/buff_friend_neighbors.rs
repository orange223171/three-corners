use serde::{Deserialize, Serialize};

/// Информация об изменении характеристик соседних дружественных строений
#[derive(Serialize, Deserialize)]
pub struct BuffFriendNeighborsInfo {
    /// Уменьшение цены строительства
    build_price_effect: i16,
    /// Увеличение стоимости уничтожения
    destroy_price_effect: i16,
    /// Увеличение стоимости захвата ресурса
    grab_price_effect: i16,

    /// Увелечение выработки экономического ресурса
    economic_profit_effect: i16,
    /// Увелечение выработки политического ресурса
    politic_profit_effect: i16,
    /// Увелечение выработки влияния
    authority_profit_effect: i16,

    /// Уменьшение количества захватываемого экономического ресурса
    economic_grab_n_effect: i16,
    /// Уменьшение количества захватываемого политического ресурса
    politic_grab_n_effect: i16,
    /// Уменьшение количества захватываемого влияния
    authority_grab_n_effect: i16,
}
