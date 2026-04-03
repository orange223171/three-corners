use std::collections::HashMap;

use crate::info::building::BuildingInfo;

/// A building kit
/// # Initialization
pub struct BuildingKit {
    /// A kit. A key is a name of building
    kit: HashMap<String, BuildingInfo>,
}

impl BuildingKit {
    pub fn kit(&self) -> &HashMap<String, BuildingInfo> {
        &self.kit
    }
}
