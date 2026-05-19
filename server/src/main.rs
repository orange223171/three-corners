use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use core_3c::{
    board::Board, building::Building, info::synergy::SynergyInfo, kit::Kit,
    player_state::PlayerState, vector::Vector,
};
use network_core::bytes_represented::{
    player_state_message::PlayerStateMessage, set_triangle_message::SetTriangleMessage,
};
use network_core::message::Message;
use network_server::connection::{Connection, ConnectionMessage};
use tokio::sync::mpsc;

use logic_3c::game::Game;

#[tokio::main]
async fn main() {
    let mut connection = Connection::init(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ));

    let mut connections_list: HashMap<SocketAddr, mpsc::Sender<Message>> = HashMap::new();

    let mut game = Game::new();

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
            let message = game.add_player(log_in_message.player);

            for (_, sender) in connections_list {
                sender.send(message.clone()).await.unwrap()
            }
        }
        Message::Build(build_message) => {
            let messages = game.build(build_message, String::from("orange"));

            for message in messages {
                for (_, sender) in connections_list {
                    sender.send(message.clone()).await.unwrap();
                }
            }
        }
        Message::Destroy(destroy_message) => {
            let messages = game.destroy(destroy_message, String::from("orange"));

            for message in messages {
                for (_, sender) in connections_list {
                    sender.send(message.clone()).await.unwrap();
                }
            }
        }
        Message::Grab(grab_message) => {
            let messages = game.grab(grab_message, String::from("orange"));

            for message in messages {
                for (_, sender) in connections_list {
                    sender.send(message.clone()).await.unwrap();
                }
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
