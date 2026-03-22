//! Содержит определения для работы с игровым полем
//!
//! ## Координаты
//! Ось X направлена от левого края к правому. Ось Y направлена от верхнего края к нижнему

use crate::{board::triangle::Triangle, synergy::Synergy};

pub mod triangle;

/// Игровое поле
pub struct Board {
    /// Поле
    board: Vec<Vec<Triangle>>,
    sinergies: Vec<Synergy>,
}
