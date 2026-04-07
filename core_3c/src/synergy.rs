//! Synergy definitions

use crate::vector::Vector;

/// A synergy
#[derive(Debug)]
pub struct Synergy {
    /// A name of the synergy from kit of the board
    pub name: String,

    /// Coordinates of the synergy on the board
    ///
    /// Coordinates of synergy is coordinates of top triangle
    pub location: Vector,
}
