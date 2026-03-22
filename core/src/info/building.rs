//! Содержит определения для работы со строениями.
//!
//! Определения для работы с конкретным экземпляром строения содержит модуль [building](crate::building)

use std::iter::Map;

use serde::{Deserialize, Serialize};

use crate::info::effect::EffectInfo;

/// # Информация о строении
///
/// Используется для хранения характеристик строения
#[derive(Serialize, Deserialize)]
pub struct BuildingInfo {
    /// Название строения
    name: String,

    /// Базовая цена строительства
    base_build_price: u16,
    /// Базовая цена уничтожения
    base_destroy_price: u16,
    /// Базовая цена захвата ресурса
    base_grab_price: u16,

    /// Базовая выработка экономического ресурса
    base_economic_profit: u16,
    /// Базовая выработка политического ресурса
    base_politic_profit: u16,
    /// Базовая выработка влияния
    base_authority_profit: u16,

    /// Базовое количество захватываемого экономического ресурса
    base_economic_grab_n: u16,
    /// Базовое количество захватываемого политического ресурса
    base_politic_grab_n: u16,
    /// Базовое количество захватываемого влияния
    base_authority_grab_n: u16,

    /// Минимальная цена строительства
    minimum_build_price: u16,
    /// Минимальная цена уничтожения
    minimum_destroy_price: u16,
    /// Минимальная цена захвата ресурса
    minimum_grab_price: u16,

    /// Минимальная выработка экономического ресурса
    minimum_economic_profit: u16,
    /// Минимальная выработка политического ресурса
    minimum_politic_profit: u16,
    /// Минимальная выработка влияния
    minimum_authority_profit: u16,

    /// Минимальное количество захватываемого экономического ресурса
    minimum_economic_grab_n: u16,
    /// Минимальное количество захватываемого политического ресурса
    minimum_politic_grab_n: u16,
    /// Минимальное количество захватываемого влияния
    minimum_authority_grab_n: u16,

    /// Эффекты строения
    effects: Vec<EffectInfo>,
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn deserialize_building_info() {
        let json: String = std::fs::read_to_string("data/buildings/townhall.json").unwrap();
        let _building_info: BuildingInfo = serde_json::from_str(json.as_str()).unwrap();

        assert!(true);
    }

    #[test]
    fn deserialize_building_info_with_effect() {
        let json: String = std::fs::read_to_string("data/buildings/windmill.json").unwrap();
        let _building_info: BuildingInfo = serde_json::from_str(json.as_str()).unwrap();

        assert!(true);
    }
}
