//! Kit definitions

use std::collections::HashMap;

use crate::info::{building::BuildingInfo, synergy::SynergyInfo};

pub mod building;
pub mod synergy;

/// A kit of the game objects
/// # Initialization
pub struct Kit {
    /// A kit of building
    pub building_kit: HashMap<String, BuildingInfo>,
    /// A kit of synergy
    pub synergy_kit: HashMap<String, SynergyInfo>,
}

impl Kit {
    /// Creates empty kit
    pub fn new() -> Kit {
        Kit {
            building_kit: HashMap::new(),
            synergy_kit: HashMap::new(),
        }
    }
}
