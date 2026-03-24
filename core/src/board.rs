//! Содержит определения для работы с игровым полем
//!
//! ## Координаты
//! Ось X направлена от левого края к правому. Ось Y направлена от верхнего края к нижнему

use std::collections::HashMap;

use crate::{building::Building, effect::Effect, kit::Kit, synergy::Synergy};

/// Треугольник
type Triangle = Option<Building>;

/// Игровое поле
pub struct Board {
    /// Поле
    board: Vec<Vec<Triangle>>,

    /// Набор игровых объектов для данного поля
    kit: Kit,
    /// Список всех синергий на поле
    sinergies: HashMap<u32, Synergy>,
    /// Список всех эффектов на поле
    effects: HashMap<u32, Effect>,
}

impl Board {
    pub fn new(scale: (usize, usize), kit: Kit) -> Self {
        let board: Vec<Vec<Triangle>> = vec![vec![Triangle::None; scale.1]; scale.0];

        Board {
            board: board,
            kit: kit,
            sinergies: HashMap::new(),
            effects: HashMap::new(),
        }
    }

    pub fn triangle(&self, coordinates: (usize, usize)) -> Triangle {
        self.board[coordinates.0][coordinates.1].clone()
    }
}
