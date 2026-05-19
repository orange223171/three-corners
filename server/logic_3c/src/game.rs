//! Game's definitions

use std::collections::HashMap;

use core_3c::{
    board::Board, building::Building, kit::Kit, player_state::PlayerState, vector::Vector,
};
use network_core::{
    bytes_represented::{
        build_message::BuildMessage, destroy_message::DestroyMessage, grab_message::GrabMessage,
        player_state_message::PlayerStateMessage, set_triangle_message::SetTriangleMessage,
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

    /// Returns a list of messages with all info of the game
    pub fn get_info(&self) -> Vec<Message> {
        let mut v: Vec<Message> = Vec::new();

        for x in 0..11 {
            for y in 0..10 {
                v.append(&mut vec![Message::SetTriangle(SetTriangleMessage {
                    location: Vector { x: x, y: y },
                    triangle: self
                        .board
                        .triangle(Vector { x: x, y: y })
                        .expect("out of bounds")
                        .clone(),
                })])
            }
        }

        self.player_states.iter().for_each(|player_state| {
            v.append(&mut vec![Message::PlayerState(PlayerStateMessage {
                player: player_state.0.clone(),
                state: player_state.1.clone(),
            })])
        });

        v
    }

    /// Adds player to the game with default states
    pub fn add_player(&mut self, player: String) -> Message {
        let state = PlayerState {
            economic: 5,
            politic: 5,
            authority: 5,
        };

        self.player_states.insert(player.clone(), state.clone());

        Message::PlayerState(PlayerStateMessage {
            player: player,
            state: state,
        })
    }

    /// Builds from BuildMessage
    pub fn build(&mut self, build_message: BuildMessage, player: String) -> Vec<Message> {
        match self.board.triangle(build_message.location) {
            Ok(triangle) => {
                if triangle.is_some() {
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

    pub fn destroy(&mut self, destroy_message: DestroyMessage, player: String) -> Vec<Message> {
        match self.board.triangle(destroy_message.location) {
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
                match self.board.kit().building_kit().get(
                    &self
                        .board
                        .triangle(destroy_message.location)
                        .expect("board error after checking")
                        .as_ref()
                        .expect("error: triangle is empty after checkings")
                        .name,
                ) {
                    Some(building_info) => {
                        if state.politic >= building_info.base_destroy_price {
                            let mut state = state.clone();
                            state.politic -= building_info.base_destroy_price;
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
            .set_triangle(None, destroy_message.location)
            .expect("Board error after checking triangle value");

        let set_triangle_message = Message::SetTriangle(SetTriangleMessage {
            location: destroy_message.location,
            triangle: None,
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

    pub fn grab(&mut self, grab_message: GrabMessage, player: String) -> Vec<Message> {
        match self.board.triangle(grab_message.location) {
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
                match self.board.kit().building_kit().get(
                    &self
                        .board
                        .triangle(grab_message.location)
                        .expect("board error after checking")
                        .as_ref()
                        .expect("error: triangle is empty after checkings")
                        .name,
                ) {
                    Some(building_info) => {
                        if state.authority >= building_info.base_grab_price {
                            let mut state = state.clone();
                            state.authority -= building_info.base_grab_price;
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

        let mut grabing_state = match self
            .board
            .triangle(grab_message.location)
            .expect("board error after checking")
        {
            Some(building) => self
                .player_states
                .get(&building.player)
                .expect("Not found player")
                .clone(),
            None => {
                return vec![Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                )];
            }
        };

        let building_info = self
            .board
            .kit()
            .building_kit()
            .get(
                &self
                    .board
                    .triangle(grab_message.location)
                    .expect("board error after checking")
                    .as_ref()
                    .expect("empty triangle after checking")
                    .name,
            )
            .expect("fail to found building in kit");

        let grabing_player = self
            .board
            .triangle(grab_message.location)
            .expect("board error after checking")
            .as_ref()
            .expect("empty triangle after checking")
            .player
            .clone();

        if grabing_state.economic < building_info.base_economic_grab_n {
            grabing_state.economic = 0;
        } else {
            grabing_state.economic -= building_info.base_economic_grab_n;
        }

        if grabing_state.politic < building_info.base_politic_grab_n {
            grabing_state.politic = 0;
        } else {
            grabing_state.politic -= building_info.base_politic_grab_n;
        }

        if grabing_state.authority < building_info.base_authority_grab_n {
            grabing_state.authority = 0;
        } else {
            grabing_state.authority -= building_info.base_authority_grab_n;
        }

        self.player_states
            .insert(grabing_player.clone(), grabing_state.clone());

        let graber_state_message = Message::PlayerState(PlayerStateMessage {
            player: player.clone(),
            state: self
                .player_states
                .get(&player)
                .expect("Fail to found player's state that was set before")
                .clone(),
        });

        let grabing_state_message = Message::PlayerState(PlayerStateMessage {
            player: grabing_player,
            state: grabing_state,
        });

        vec![graber_state_message, grabing_state_message]
    }
}
