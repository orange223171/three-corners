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

/// Triangle
type Triangle = Option<Building>;

/// Game board
pub struct Board {
    /// Scale of board
    scale: (usize, usize),
    /// Board
    board: Vec<Vec<Triangle>>,

    /// Kit of the board
    kit: Kit,
    /// List of all sinergies on the board. The key is a id of synergy on the board
    sinergies: HashMap<u32, Synergy>,
    /// List of all effects on the board. The key is a id of effect on the board
    effects: HashMap<u32, Effect>,
}

impl Board {
    /// Create empty board
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

    /// Return a reference on triangle with specified coordinates
    pub fn triangle(&self, coordinates: (usize, usize)) -> &Triangle {
        &self.board[coordinates.0][coordinates.1]
    }
    /// Return a mutable reference on triangle with specified coordinates
    pub fn mut_triangle(&mut self, coordinates: (usize, usize)) -> &mut Triangle {
        &mut self.board[coordinates.0][coordinates.1]
    }

    /// Return a reference on the kit
    pub fn kit(&self) -> &Kit {
        &self.kit
    }

    /// Return a reference on sinergy with specified id or [None]
    pub fn sinergy(&self, id: u32) -> Option<&Synergy> {
        self.sinergies.get(&id)
    }

    /// Return a reference on effect with specified id or [None]
    pub fn effect(&self, id: u32) -> Option<&Effect> {
        self.effects.get(&id)
    }

    /// Add synergy on the board
    /// # Errors
    /// - if synergy with specified name doesn't exist in the kit return [SynergyNameUndefined](BoardError::SynergyNameUndefined)
    /// - if specified coordinates out of board's bounds return [SynergyOutBounds](BoardError::SynergyOutOfBounds)
    pub fn add_sinergy(&mut self, id: u32, sinergy: Synergy) -> Result<(), BoardError> {
        if (sinergy.location.0 >= self.scale.0) || (sinergy.location.1 >= self.scale.1) {
            return Result::Err(BoardError::SynergyOutOfBounds);
        }

        if self.kit.synergy_kit.kit().get(&sinergy.name).is_none() {
            return Result::Err(BoardError::SinergyNameUndefined);
        }

        self.sinergies.insert(id, sinergy);

        Result::Ok(())
    }

    /// Add effect on the board
    /// # Errors
    /// - if source of effect not found return [EffectObjectNotFound](BoardError::EffectObjectNotFound)
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

    /// Remove sinergy from the board
    /// # Errors
    /// - if sinergy doesn't exist on the board return [SynergyNotFound](BoardError::SynergyNotFound)
    pub fn remove_sinergy(&mut self, id: u32) -> Result<(), BoardError> {
        match self.sinergies.remove(&id) {
            None => Result::Err(BoardError::SynergyNotfound),
            Some(_) => Result::Ok(()),
        }
    }

    /// Remove effect from the board
    /// # Errors
    /// - if effect doesn't exist on the board return [EffectNotFound](BoardError::EffectNotFound)
    pub fn remove_effect(&mut self, id: u32) -> Result<(), BoardError> {
        match self.sinergies.remove(&id) {
            None => Result::Err(BoardError::EffectNotFound),
            Some(_) => Result::Ok(()),
        }
    }

    /// Return true if effect object is exist, else return false
    fn effect_object_is_exist(&self, effect_object: &EffectObject) -> bool {
        match effect_object {
            EffectObject::Triangle(x, y) => (x < &self.scale.0) && (y < &self.scale.1),
            EffectObject::Synergy(id) => self.sinergies.get(&id).is_some(),
        }
    }
}

/// Board error
pub enum BoardError {
    EffectObjectNotFound,
    EffectNotFound,
    SynergyOutOfBounds,
    SinergyNameUndefined,
    SynergyNotfound,
}
