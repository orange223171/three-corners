use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use core_3c::{building::Building, vector::Vector};
use network_core::bytes_represented::set_triangle::SetTriangleMessage;
use network_core::message::Message;
use network_server::connection::{self, Connection};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    Connection::init(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 23171),
        message_handler,
    );

    loop {}
}

async fn message_handler(
    message: Message,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
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
                .await;
        }
        Message::Error(error_message) => todo!(),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => todo!(),
        Message::Build(build_message) => todo!(),
        Message::Destroy(destroy_message) => todo!(),
        Message::Grab(grab_message) => todo!(),
        Message::SetTriangle(set_triangle_message) => todo!(),
    }
}
