//! Содержит определения для работы с игровым полем
//!
//! ## Координаты
//! Ось X направлена от левого края к правому. Ось Y направлена от верхнего края к нижнему

use std::iter::Map;

use crate::{building::Building, effect::Effect, synergy::Synergy};

/// Треугольник
type Triangle = Option<Building>;

/// Игровое поле
pub struct Board {
    /// Поле
    board: Vec<Vec<Triangle>>,

    /// Список всех синергий на поле
    sinergies: Map<u32, Synergy>,
    /// Список всех эффектов на поле
    effects: Map<u32, Effect>,
}
