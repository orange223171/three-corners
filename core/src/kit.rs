//! Kit definitions

use std::{collections::HashMap, fs};

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

    /// Creates kit from JSON-files
    ///
    /// Files must be located in buildings/ and synergies/ directories on the path
    pub fn from_files(path: String) -> Result<Kit, KitError> {
        let mut kit = Kit::new();

        for file in fs::read_dir(path.clone() + "/buildings").unwrap() {
            kit.add_building(
                fs::read_to_string(file.unwrap().path().to_str().unwrap())
                    .unwrap()
                    .as_str(),
            )?;
        }

        for file in fs::read_dir(path + "/synergies").unwrap() {
            kit.add_synergy(
                fs::read_to_string(file.unwrap().path().to_str().unwrap())
                    .unwrap()
                    .as_str(),
            )?;
        }

        Result::Ok(kit)
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
                self.synergy_kit
                    .insert(synergy_info.name.clone(), synergy_info);
                Result::Ok(())
            }
            serde_json::Result::Err(_) => Result::Err(KitError::JsonParsingError),
        }
    }

    /// Removes building from the kit
    pub fn remove_building(&mut self, name: String) {
        self.building_kit.remove(&name);
    }

    /// Removes synergy from the kit
    pub fn remove_synergy(&mut self, name: String) {
        self.synergy_kit.remove(&name);
    }
}

/// A kit error
pub enum KitError {
    BuildingNameUndefined,
    EffectNameUndefined,
    JsonParsingError,
}
