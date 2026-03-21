//! Содержит определения для работы со строениями.
//!
//! Определения для работы с конкретным экземпляром строения содержит модуль [building](crate::building)

use serde::{Deserialize, Serialize};

use crate::effect_info::EffectInfo;

mod tests;

/// Информация о строении
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
