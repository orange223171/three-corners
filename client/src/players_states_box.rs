//! Players' states box definitions

use std::{collections::HashMap, sync::Arc};

use core_3c::player_state::{self, PlayerState};
use sfml::graphics::{Drawable, Font, Text};
use tokio::sync::Mutex;

/// A players' states box
pub struct PlayersStatesBox {
    players_states: Arc<Mutex<HashMap<String, PlayerState>>>,
}

impl PlayersStatesBox {
    pub fn new(players_states: Arc<Mutex<HashMap<String, PlayerState>>>) -> Self {
        Self {
            players_states: players_states,
        }
    }
}

impl Drawable for PlayersStatesBox {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let font =
            Font::from_file("/usr/share/fonts/TTF/DejaVuSans.ttf").expect("Error to load font");

        let players_states = loop {
            if let Ok(board) = self.players_states.try_lock() {
                break board;
            }
        };

        let mut render_states = states.clone();
        players_states.iter().for_each(|player_state| {
            let player_state_text = player_state.0.clone()
                + " "
                + player_state.1.economic.to_string().as_str()
                + " "
                + player_state.1.politic.to_string().as_str()
                + " "
                + player_state.1.authority.to_string().as_str();
            let text = Text::new(&player_state_text, &font, 16);

            render_states.transform.translate(0.0, 32.0);
            target.draw_with_renderstates(&text, &render_states);
        });
    }
}
