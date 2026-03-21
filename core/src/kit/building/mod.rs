use std::iter::Map;

use crate::building_info::BuildingInfo;

/// # Набор строений
/// Используется для хранения используемого в игровой сессии набора строений.
///
/// ## Инициализация
pub struct BuildingKit {
    /// Набор, ключом является название строения
    kit: Map<String, BuildingInfo>,
}
