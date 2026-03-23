//! Содержит определения для работы с треугольником игрового поля

use crate::building::Building;

/// Треугольник
pub struct Triangle {
    /// Cтроение или его отсутствие
    building: Option<Building>,
}
