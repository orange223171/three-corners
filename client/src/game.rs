//! Game's definitions

use std::{collections::HashMap, sync::Arc};

use core_3c::{board::Board, player_state::PlayerState};
use tokio::sync::Mutex;

/// A game
pub struct Game {
    /// Board of the game
    pub board: Arc<Mutex<Board>>,
    /// Players' states of the game
    pub players_states: Arc<Mutex<HashMap<String, PlayerState>>>,
}
