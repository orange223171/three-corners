//! Game's definitions

use std::collections::HashMap;

use core_3c::{
    board::Board,
    building::Building,
    kit::Kit,
    player_state::{self, PlayerState},
    vector::Vector,
};
use network_core::{
    bytes_represented::{
        build_message::{self, BuildMessage},
        destroy_message::DestroyMessage,
        player_state_message::PlayerStateMessage,
        set_triangle_message::SetTriangleMessage,
    },
    message::Message,
};

/// A game struct
pub struct Game {
    /// A board of game
    board: Board,
    /// A list of players' states. Key is nickname of player
    player_states: HashMap<String, PlayerState>,
}

impl Game {
    /// Returns new game
    pub fn new() -> Self {
        Self {
            board: Board::new(
                Vector { x: 10, y: 11 },
                Kit::from_files(String::from("core_3c/data/"))
                    .expect("Error to load kit from core_3c/data/"),
            ),
            player_states: HashMap::new(),
        }
    }

    /// Builds from BuildMessage
    pub fn build(&mut self, build_message: BuildMessage, player: String) -> Vec<Message> {
        match self.board.triangle(build_message.location) {
            Ok(triangle) => {
                if triangle.is_none() {
                    return vec![Message::Error(
                        network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                    )];
                }
            }
            Err(_) => {
                return vec![Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                )];
            }
        }

        match self.player_states.get(&player) {
            Some(state) => {
                match self
                    .board
                    .kit()
                    .building_kit()
                    .get(&build_message.build_name)
                {
                    Some(building_info) => {
                        if state.economic >= building_info.base_build_price {
                            let mut state = state.clone();
                            state.economic -= building_info.base_build_price;
                            self.player_states.insert(player.clone(), state);
                        } else {
                            return vec![Message::Error(
                                network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                            )];
                        }
                    }
                    None => {
                        return vec![Message::Error(
                            network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                        )];
                    }
                }
            }
            None => {
                return vec![Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                )];
            }
        }

        self.board
            .set_triangle(
                Some(Building {
                    name: build_message.build_name.clone(),
                    player: player.clone(),
                    build_in_current_round: true,
                    synergies: Vec::new(),
                }),
                build_message.location,
            )
            .expect("Board error after checking triangle value");

        let set_triangle_message = Message::SetTriangle(SetTriangleMessage {
            location: build_message.location,
            triangle: Some(Building {
                name: build_message.build_name.clone(),
                player: player.clone(),
                build_in_current_round: true,
                synergies: Vec::new(),
            }),
        });
        let player_state_message = Message::PlayerState(PlayerStateMessage {
            player: player.clone(),
            state: self
                .player_states
                .get(&player)
                .expect("Fail to found player's state that was set before")
                .clone(),
        });

        vec![set_triangle_message, player_state_message]
    }

    pub fn destroy(&mut self, destroy_message: DestroyMessage) -> Message {
        Message::SetTriangle(SetTriangleMessage {
            location: destroy_message.location,
            triangle: None,
        })
    }
}
