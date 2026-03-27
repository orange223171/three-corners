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
    scale: (usize, usize),
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
    /// Создаёт пустой экземпляр игровой доски
    pub fn new(scale: (usize, usize), kit: Kit) -> Self {
        let board: Vec<Vec<Triangle>> = vec![vec![Triangle::None; scale.1]; scale.0];

        Board {
            scale: scale,
            board: board,
            kit: kit,
            sinergies: HashMap::new(),
            effects: HashMap::new(),
        }
    }

    /// Возвращает изменяемую ссылку на треугольник, находящийся по указанным координатам
    pub fn triangle(&mut self, coordinates: (usize, usize)) -> &mut Triangle {
        &mut self.board[coordinates.0][coordinates.1]
    }

    /// Возвращает ссылку на набор игровых объектов
    pub fn kit(&self) -> &Kit {
        &self.kit
    }

    /// Возвращает ссылку на синергию c указанным id
    ///
    /// Если синергию с указанным id не существует, возвращает [None]
    pub fn sinergy(&self, id: u32) -> Option<&Synergy> {
        self.sinergies.get(&id)
    }

    /// Возвращает ссылку на эффект с указанным id
    ///
    /// Если эффекта с указанным id не существует, возвращает [None]
    pub fn effect(&self, id: u32) -> Option<&Effect> {
        self.effects.get(&id)
    }

    /// Добавляет синергию на игровую доску
    /// # Ошибки
    /// - Если в наборе игровых объектов не существует синергии с указанным именем, возвращает
    /// - Если указанные координаты назодятся за пределами игровой доски, возвращает
    pub fn add_sinergy(&mut self, id: u32, sinergy: Synergy) -> Result<(), &str> {
        if (sinergy.location.0 >= self.scale.0) || (sinergy.location.1 >= self.scale.1) {
            return Result::Err("err");
        }

        self.sinergies.insert(id, sinergy);

        Result::Ok(())
    }

    /// Добавляет эффект на игровую доску
    pub fn add_effect(&mut self, id: u32, effect: Effect) {
        self.effects.insert(id, effect);
    }

    /// Убирает синергию с игровой доски
    pub fn remove_sinergy(&mut self, id: u32) -> Result<(), &str> {
        match self.sinergies.remove(&id) {
            None => Result::Err("err"),
            Some(_) => Result::Ok(()),
        }
    }

    /// Убирает эффект с игровой доски
    pub fn remove_effect(&mut self, id: u32) -> Result<(), &str> {
        match self.sinergies.remove(&id) {
            None => Result::Err("err"),
            Some(_) => Result::Ok(()),
        }
    }
}
