//! Player state definitions

/// A player state
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// A number of economic resource
    pub economic: u32,
    /// A number of politic resource
    pub politic: u32,
    /// A number of authority resource
    pub authority: u32,
}
