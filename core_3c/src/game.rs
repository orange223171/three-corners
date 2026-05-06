//! Game definitions

use std::collections::HashMap;

use crate::{board::Board, player_state::PlayerState};

/// A game struct
pub struct Game {
    /// A board of game
    pub board: Board,
    /// A list of players' states. Key is nickname of player
    pub player_states: HashMap<String, PlayerState>,
}
