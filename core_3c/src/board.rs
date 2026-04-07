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
    vector::Vector,
};

/// A triangle
pub type Triangle = Option<Building>;

/// A game board
pub struct Board {
    /// A scale of the board
    scale: Vector,
    /// A board
    board: Vec<Triangle>,

    /// A kit of the board
    kit: Kit,
    /// A list of all sinergies on the board. The key is a id of synergy on the board
    sinergies: HashMap<u32, Synergy>,
    /// A list of all effects on the board. The key is a id of effect on the board
    effects: HashMap<u32, Effect>,
}

impl Board {
    /// Creates empty board
    pub fn new(scale: Vector, kit: Kit) -> Self {
        let board: Vec<Triangle> = vec![Triangle::None; (scale.x as usize) * (scale.y as usize)];

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
    /// - if specified coordinates out of board's bounds returns [OutBounds](BoardError::OutOfBounds)
    pub fn triangle(&self, coordinates: Vector) -> Result<&Triangle, BoardError> {
        match self
            .board
            .get((coordinates.y as usize * self.scale.x as usize) + coordinates.x as usize)
        {
            Some(triangle) => Result::Ok(triangle),
            None => Result::Err(BoardError::SynergyOutOfBounds),
        }
    }

    /// Sets a triangle with specified coordinates to specified state
    /// # Errors
    /// - if specified coordinates out of board's bounds returns [OutBounds](BoardError::OutOfBounds)
    /// - if name of building in specified state of triangle doesn't exist in the kit returns
    /// [BuildingNameUndefined](BoardError::BuildingNameUndefined)
    pub fn set_triangle(
        &mut self,
        triangle: Triangle,
        coordinates: Vector,
    ) -> Result<(), BoardError> {
        if let Some(building) = &triangle {
            if self.kit.building_kit().get(&building.name).is_none() {
                return Result::Err(BoardError::BuildingNameUndefined);
            }
        }

        match self
            .board
            .get_mut((coordinates.y as usize * self.scale.x as usize) + coordinates.x as usize)
        {
            Some(setting_triangle) => {
                *setting_triangle = triangle;
                Result::Ok(())
            }
            None => Result::Err(BoardError::OutOfBounds),
        }
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
        if (synergy.location.x >= self.scale.x) || (synergy.location.y >= self.scale.y + 1) {
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
            EffectObject::Triangle(coordinates) => {
                (coordinates.x < self.scale.x) && (coordinates.y < self.scale.y)
            }
            EffectObject::Synergy(id) => self.sinergies.get(&id).is_some(),
        }
    }
}

/// A board error
#[derive(Debug)]
pub enum BoardError {
    OutOfBounds,
    BuildingNameUndefined,
    EffectObjectNotFound,
    EffectNotFound,
    SynergyOutOfBounds,
    SynergyNameUndefined,
    SynergyNotfound,
}
