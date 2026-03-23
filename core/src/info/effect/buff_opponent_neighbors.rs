use serde::{Deserialize, Serialize};

/// Информация об изменении характеристик соседних строений соперника
#[derive(Serialize, Deserialize)]
pub struct BuffOpponentNeighborsInfo {
    /// Увеличение цены строительства
    build_price_increase: u16,
    /// Уменьшение стоимости уничтожения
    destroy_price_decrease: u16,
    /// Уменьшение стоимости захвата ресурса
    grab_price_decrease: u16,

    /// Уменьшение выработки экономического ресурса
    economic_profit_decrease: u16,
    /// Уменьшение выработки политического ресурса
    politic_profit_decrease: u16,
    /// Уменьшение выработки влияния
    authority_profit_decrease: u16,

    /// Увеличение количества захватываемого экономического ресурса
    economic_grab_n_increase: u16,
    /// Увеличение количества захватываемого политического ресурса
    politic_grab_n_increase: u16,
    /// Увеличение количества захватываемого влияния
    authority_grab_n_increase: u16,
}
