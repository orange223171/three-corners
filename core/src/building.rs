//! Building definitions

/// A building
#[derive(Debug, Clone)]
pub struct Building {
    /// A name of the building from kit of the board
    pub name: String,
    /// A player which is owner of the building
    pub player: String,

    /// Building is build in current round
    pub build_in_current_round: bool,

    /// Synergies' id on the board include in which the building
    pub synergies: Vec<u32>,
}
