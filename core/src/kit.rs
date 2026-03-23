//! Содержит определения для взвимодействия с игровыми объектами

use crate::kit::{building::BuildingKit, synergy::SynergyKit};

pub mod building;
pub mod synergy;

/// # Набор игровых объектов
/// Используется для хранения используемого в игровой сессии набора игровых объектов
///
/// ## Инициализация
pub struct Kit {
    /// Набор строений
    pub building_kit: BuildingKit,
    /// Набор синергий
    pub synergy_kit: SynergyKit,
}
