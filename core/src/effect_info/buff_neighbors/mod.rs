/// Информация об увеличении характеристик соседних дружественных строений
pub struct BuffNeighborsInfo {
    /// Уменьшение цены строительства
    build_price_decrease: u16,
    /// Увеличение стоимости уничтожения
    destroy_price_increase: u16,
    /// Увеличение стоимости захвата ресурса
    grab_price_increase: u16,

    /// Увелечение выработки экономического ресурса
    economic_profit_increase: u16,
    /// Увелечение выработки политического ресурса
    politic_profit_increase: u16,
    /// Увелечение выработки влияния
    authority_profit_increase: u16,

    /// Уменьшение количества захватываемого экономического ресурса
    economic_grab_n_decrease: u16,
    /// Уменьшение количества захватываемого политического ресурса
    politic_grab_n_decrease: u16,
    /// Уменьшение количества захватываемого влияния
    authority_grab_n_decrease: u16,
}
