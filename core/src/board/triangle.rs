//! Содержит определения для работы с треугольником игрового поля

use crate::{board::triangle::effect::TriangleEffect, building::Building};

pub mod effect;

/// Треугольник
pub struct Triangle {
    /// Cтроение или его отсутствие
    building: Option<Building>,

    /// Эффекты, оказываемые на треугольник
    by_effect: Vec<TriangleEffect>,
    /// Эффекты, оказываемые треугольником
    on_effect: Vec<TriangleEffect>,
}
