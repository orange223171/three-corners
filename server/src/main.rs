use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use core_3c::{
    board::Board, building::Building, game::Game, kit::Kit, player_state::PlayerState,
    vector::Vector,
};
use network_core::bytes_represented::{
    player_state_message::PlayerStateMessage, set_triangle_message::SetTriangleMessage,
};
use network_core::message::Message;
use network_server::connection::{Connection, ConnectionMessage};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let mut connection = Connection::init(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ));

    let mut connections_list: HashMap<SocketAddr, mpsc::Sender<Message>> = HashMap::new();

    let mut game = Game {
        board: Board::new(
            Vector { x: 10, y: 11 },
            Kit::from_files(String::from("core_3c/data/"))
                .expect("Error to load kit from core_3c/data/"),
        ),
        player_states: HashMap::new(),
    };

    loop {
        match connection.reciever.recv().await {
            Some(connection_message) => match connection_message {
                ConnectionMessage::Connect(socket, sender) => {
                    connections_list.insert(socket, sender);
                }
                ConnectionMessage::Disconnect(socket) => {
                    connections_list.remove(&socket);
                }
                ConnectionMessage::Message(socket, message) => {
                    message_handler(message, &socket, &connections_list, &mut game).await;
                }
            },
            None => break,
        }
    }
}

async fn message_handler(
    message: Message,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    game: &mut Game,
) {
    match message {
        Message::Ok => {
            connections_list
                .get(socket)
                .unwrap()
                .send(Message::SetTriangle(SetTriangleMessage {
                    location: Vector { x: 5, y: 5 },
                    triangle: Some(Building {
                        name: String::from("field"),
                        player: String::from("orange"),
                        build_in_current_round: false,
                        synergies: Vec::new(),
                    }),
                }))
                .await
                .unwrap();
        }
        Message::Error(error_message) => todo!(),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => todo!(),
        Message::LogIn(log_in_message) => {
            let player = log_in_message.player;
            let state = PlayerState {
                economic: 5,
                politic: 5,
                authority: 5,
            };

            game.player_states.insert(player.clone(), state.clone());
            for (_, sender) in connections_list {
                sender
                    .send(Message::PlayerState(PlayerStateMessage {
                        player: player.clone(),
                        state: state.clone(),
                    }))
                    .await
                    .unwrap()
            }
        }
        Message::Build(build_message) => {
            game.board
                .set_triangle(
                    Some(Building {
                        name: build_message.build_name.clone(),
                        player: String::from("orange"),
                        build_in_current_round: true,
                        synergies: Vec::new(),
                    }),
                    build_message.location,
                )
                .expect("board error, here must be normall error");

            for (_, sender) in connections_list {
                sender
                    .send(Message::SetTriangle(SetTriangleMessage {
                        location: build_message.location,
                        triangle: Some(Building {
                            name: build_message.build_name.clone(),
                            player: String::from(""),
                            build_in_current_round: true,
                            synergies: Vec::new(),
                        }),
                    }))
                    .await
                    .unwrap()
            }
        }
        Message::Destroy(destroy_message) => {
            for (_, sender) in connections_list {
                sender
                    .send(Message::SetTriangle(SetTriangleMessage {
                        location: destroy_message.location,
                        triangle: None,
                    }))
                    .await
                    .unwrap()
            }
        }
        Message::Grab(grab_message) => {
            let building = game
                .board
                .triangle(grab_message.location)
                .expect("out of bounds, here must be normal error")
                .as_ref()
                .expect("empty triangle, here must be normal error");

            let building_info = game
                .board
                .kit()
                .building_kit()
                .get(&building.name)
                .expect("Not found building in kit, here must be normal error");

            let mut state = game
                .player_states
                .get(&building.player)
                .expect("Not found player, here must be normal error")
                .clone();

            if state.economic < building_info.base_economic_grab_n {
                state.economic = 0;
            } else {
                state.economic -= building_info.base_economic_grab_n;
            }

            if state.politic < building_info.base_politic_grab_n {
                state.politic = 0;
            } else {
                state.politic -= building_info.base_politic_grab_n;
            }

            if state.authority < building_info.base_authority_grab_n {
                state.authority = 0;
            } else {
                state.authority -= building_info.base_authority_grab_n;
            }

            game.player_states
                .insert(building.player.clone(), state.clone());

            for (_, sender) in connections_list {
                sender
                    .send(Message::PlayerState(PlayerStateMessage {
                        player: building.player.clone(),
                        state: state.clone(),
                    }))
                    .await
                    .unwrap();
            }
        }
        Message::SetTriangle(_) => connections_list
            .get(socket)
            .unwrap()
            .send(Message::Error(
                network_core::bytes_represented::error_message::ErrorMessage::UnexpectedMessage,
            ))
            .await
            .unwrap(),
        Message::PlayerState(_) => connections_list
            .get(socket)
            .unwrap()
            .send(Message::Error(
                network_core::bytes_represented::error_message::ErrorMessage::UnexpectedMessage,
            ))
            .await
            .unwrap(),
    }
}
