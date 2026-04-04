//! Board definitions
//!
//! # Coordinates
//! The X axis is directed from left edge to right. The Y axis is directed from top edge to bottom

use std::collections::HashMap;

use crate::{
    building::Building,
    effect::{Effect, EffectObject},
    kit::Kit,
    synergy::Synergy,
};

/// A triangle
pub type Triangle = Option<Building>;

/// A game board
pub struct Board {
    /// A scale of the board
    scale: (usize, usize),
    /// A board
    board: Vec<Vec<Triangle>>,

    /// A kit of the board
    kit: Kit,
    /// A list of all sinergies on the board. The key is a id of synergy on the board
    sinergies: HashMap<u32, Synergy>,
    /// A list of all effects on the board. The key is a id of effect on the board
    effects: HashMap<u32, Effect>,
}

impl Board {
    /// Creates empty board
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

    /// Returns a reference on triangle with specified coordinates
    /// # Errors
    /// - if specified coordinates out of board's bounds returns [SynergyOutBounds](BoardError::SynergyOutOfBounds)
    pub fn triangle(&self, coordinates: (usize, usize)) -> Result<&Triangle, BoardError> {
        if (coordinates.0 >= self.scale.0) || (coordinates.1 >= self.scale.1) {
            return Result::Err(BoardError::SynergyOutOfBounds);
        }

        Result::Ok(&self.board[coordinates.0][coordinates.1])
    }

    /// Sets a triangle with specified coordinates to specified state
    /// # Errors
    /// - if specified coordinates out of board's bounds returns [SynergyOutBounds](BoardError::SynergyOutOfBounds)
    /// - if name of building in specified state of triangle doesn't exist in the kit returns
    /// [BuildingNameUndefined](BoardError::BuildingNameUndefined)
    pub fn set_triangle(
        &mut self,
        triangle: Triangle,
        coordinates: (usize, usize),
    ) -> Result<(), BoardError> {
        if (coordinates.0 >= self.scale.0) || (coordinates.1 >= self.scale.1) {
            return Result::Err(BoardError::SynergyOutOfBounds);
        }

        if let Some(building) = &triangle {
            if self.kit.building_kit().get(&building.name).is_none() {
                return Result::Err(BoardError::BuildingNameUndefined);
            }
        }

        self.board[coordinates.0][coordinates.1] = triangle;

        Result::Ok(())
    }

    /// Returns a reference on the kit
    pub fn kit(&self) -> &Kit {
        &self.kit
    }

    /// Returns a reference on synergy with specified id or [None]
    pub fn synergy(&self, id: u32) -> Option<&Synergy> {
        self.sinergies.get(&id)
    }

    /// Returns a reference on effect with specified id or [None]
    pub fn effect(&self, id: u32) -> Option<&Effect> {
        self.effects.get(&id)
    }

    /// Adds synergy on the board
    /// # Errors
    /// - if synergy with specified name doesn't exist in the kit returns [SynergyNameUndefined](BoardError::SynergyNameUndefined)
    /// - if specified coordinates out of board's bounds returns [SynergyOutBounds](BoardError::SynergyOutOfBounds)
    pub fn add_synergy(&mut self, id: u32, synergy: Synergy) -> Result<(), BoardError> {
        if (synergy.location.0 >= self.scale.0) || (synergy.location.1 >= self.scale.1) {
            return Result::Err(BoardError::SynergyOutOfBounds);
        }

        if self.kit.synergy_kit().get(&synergy.name).is_none() {
            return Result::Err(BoardError::SynergyNameUndefined);
        }

        self.sinergies.insert(id, synergy);

        Result::Ok(())
    }

    /// Adds effect on the board
    /// # Errors
    /// - if source of effect not found returns [EffectObjectNotFound](BoardError::EffectObjectNotFound)
    pub fn add_effect(&mut self, id: u32, effect: Effect) -> Result<(), BoardError> {
        if !self.effect_object_is_exist(&effect.source) {
            return Result::Err(BoardError::EffectNotFound);
        }

        if !self.effect_object_is_exist(&effect.destination) {
            return Result::Err(BoardError::EffectNotFound);
        }

        self.effects.insert(id, effect);

        Result::Ok(())
    }

    /// Removes synergy from the board
    /// # Errors
    /// - if synergy doesn't exist on the board returns [SynergyNotFound](BoardError::SynergyNotFound)
    pub fn remove_synergy(&mut self, id: u32) -> Result<(), BoardError> {
        match self.sinergies.remove(&id) {
            None => Result::Err(BoardError::SynergyNotfound),
            Some(_) => Result::Ok(()),
        }
    }

    /// Removes effect from the board
    /// # Errors
    /// - if effect doesn't exist on the board returns [EffectNotFound](BoardError::EffectNotFound)
    pub fn remove_effect(&mut self, id: u32) -> Result<(), BoardError> {
        match self.sinergies.remove(&id) {
            None => Result::Err(BoardError::EffectNotFound),
            Some(_) => Result::Ok(()),
        }
    }

    /// Returns true if effect object is exist, else returns false
    fn effect_object_is_exist(&self, effect_object: &EffectObject) -> bool {
        match effect_object {
            EffectObject::Triangle(x, y) => (x < &self.scale.0) && (y < &self.scale.1),
            EffectObject::Synergy(id) => self.sinergies.get(&id).is_some(),
        }
    }
}

/// A board error
pub enum BoardError {
    BuildingNameUndefined,
    EffectObjectNotFound,
    EffectNotFound,
    SynergyOutOfBounds,
    SynergyNameUndefined,
    SynergyNotfound,
}
