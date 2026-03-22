use std::iter::Map;

use crate::info::synergy::SynergyInfo;

/// # Набор синергий
/// Используется для хранения используемого в игровой сессии набора синергий
///
/// ## Инициализация
pub struct SynergyKit {
    /// Набор, ключом является название синергии
    kit: Map<String, SynergyInfo>,
}
