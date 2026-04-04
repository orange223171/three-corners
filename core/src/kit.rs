//! Kit definitions

use crate::kit::{building::BuildingKit, synergy::SynergyKit};

pub mod building;
pub mod synergy;

/// A kit of the game objects
/// # Initialization
pub struct Kit {
    /// A kit of building
    pub building_kit: BuildingKit,
    /// A kit of synergy
    pub synergy_kit: SynergyKit,
}
