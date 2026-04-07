//! Building kit definitions

use std::collections::HashMap;

use crate::info::building::BuildingInfo;

/// A building kit
/// # Initialization
pub struct BuildingKit {
    /// A kit. A key is a name of building
    kit: HashMap<String, BuildingInfo>,
}

impl BuildingKit {
    /// Creates empty kit of buildings
    pub fn new() -> BuildingKit {
        BuildingKit {
            kit: HashMap::new(),
        }
    }

    /// Returns the kit of buildings
    pub fn kit(&self) -> &HashMap<String, BuildingInfo> {
        &self.kit
    }
}
