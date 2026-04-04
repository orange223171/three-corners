//! Kit definitions

use std::collections::HashMap;

use crate::info::{building::BuildingInfo, synergy::SynergyInfo};

pub mod building;
pub mod synergy;

/// A kit of the game objects
/// # Initialization
pub struct Kit {
    /// A kit of building
    building_kit: HashMap<String, BuildingInfo>,
    /// A kit of synergy
    synergy_kit: HashMap<String, SynergyInfo>,
}

impl Kit {
    /// Creates empty kit
    pub fn new() -> Kit {
        Kit {
            building_kit: HashMap::new(),
            synergy_kit: HashMap::new(),
        }
    }

    /// Returns reference on the building kit
    pub fn building_kit(&self) -> &HashMap<String, BuildingInfo> {
        &self.building_kit
    }

    /// Returns reference on the synergy kit
    pub fn synergy_kit(&self) -> &HashMap<String, SynergyInfo> {
        &self.synergy_kit
    }

    /// Adds building from JSON format
    /// # Errors
    /// - if effect's name undefined returns [EffectNameUndefined](KitError::EffectNameUndefined)
    /// - if JSON is uncorrected returns [JsonParsingError](KitError::JsonParsingError)
    pub fn add_building(&mut self, json: &str) -> Result<(), KitError> {
        match serde_json::from_str::<BuildingInfo>(json) {
            serde_json::Result::Ok(building_info) => {
                self.building_kit
                    .insert(building_info.name.clone(), building_info);
                Result::Ok(())
            }
            serde_json::Result::Err(_) => Result::Err(KitError::JsonParsingError),
        }
    }

    /// Adds synergy from JSON format
    /// # Errors
    /// - if building's name undefiend returns [BuildingNameUndefined](KitError::BuildingNameUndefined)
    /// - if effect's name undefined returns [EffectNameUndefined](KitError::EffectNameUndefined)
    /// - if JSON is uncorrected returns [JsonParsingError](KitError::JsonParsingError
    pub fn add_synergy(&mut self, json: &str) -> Result<(), KitError> {
        match serde_json::from_str::<SynergyInfo>(json) {
            serde_json::Result::Ok(synergy_info) => {
                for i in 0..6 {
                    if self
                        .building_kit()
                        .get(&synergy_info.buildings[i])
                        .is_none()
                    {
                        return Result::Err(KitError::BuildingNameUndefined);
                    }
                }
                Result::Ok(())
            }
            serde_json::Result::Err(_) => Result::Err(KitError::JsonParsingError),
        }
    }
}

/// A kit error
pub enum KitError {
    BuildingNameUndefined,
    EffectNameUndefined,
    JsonParsingError,
}
