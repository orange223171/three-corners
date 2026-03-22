use std::iter::Map;

use crate::info::building::BuildingInfo;

/// # Набор строений
/// Используется для хранения используемого в игровой сессии набора строений.
///
/// ## Инициализация
pub struct BuildingKit {
    /// Набор, ключом является название строения
    kit: Map<String, BuildingInfo>,
}
