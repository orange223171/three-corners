//! Содержит определения для работы с игровым полем
//!
//! ## Координаты
//! Ось X направлена от левого края к правому. Ось Y направлена от верхнего края к нижнему

use crate::{effect::Effect, synergy::Synergy, triangle::Triangle};

/// Игровое поле
pub struct Board {
    /// Поле
    board: Vec<Vec<Triangle>>,
    /// Список всех синергий на поле
    sinergies: Vec<Synergy>,
    /// Список всех эффектов на поле
    effects: Vec<Effect>,
}
