use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use network_core::message::Message;
use network_server::connection::{self, Connection};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    Connection::init(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 23117),
        message_handler,
    );

    loop {}
}

fn message_handler(
    message: Message,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
) {
}
