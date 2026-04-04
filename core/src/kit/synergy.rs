//! Synergy kit definitions

use std::collections::HashMap;

use crate::info::synergy::SynergyInfo;

/// A synergy kit
/// # Initialization
pub struct SynergyKit {
    /// A kit. A key is a name of synergy
    kit: HashMap<String, SynergyInfo>,
}

impl SynergyKit {
    /// Return the kit of sinergies
    pub fn kit(&self) -> &HashMap<String, SynergyInfo> {
        &self.kit
    }
}
