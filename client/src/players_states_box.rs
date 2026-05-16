//! Players' states box definitions

use std::collections::HashMap;

use core_3c::player_state::PlayerState;
use sfml::graphics::Drawable;

/// A players' states box
pub struct PlayersStatesBox {
    pub players_states: HashMap<String, PlayerState>,
}

impl Drawable for PlayersStatesBox {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        todo!()
    }
}
