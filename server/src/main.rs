use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use network_core::message::Message;
use network_server::connection::{self, Connection, ConnectionMessage};
use tokio::{
    sync::{Mutex, mpsc},
    time::{self, Instant},
};

use logic_3c::game::Game;

#[tokio::main]
async fn main() {
    let mut connection = Connection::init(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ));

    let connections_list: Arc<Mutex<HashMap<SocketAddr, mpsc::Sender<Message>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let mut players_list: HashMap<SocketAddr, String> = HashMap::new();

    let game = Arc::new(Mutex::new(Game::new()));

    let game_mutex = game.clone();
    let connections_list_mutex = connections_list.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(15));

        loop {
            interval.tick().await;

            let messages = game_mutex.lock().await.round();

            let connections_list = connections_list_mutex.lock().await;

            for message in messages {
                for (_, sender) in connections_list.clone() {
                    sender.send(message.clone()).await.unwrap();
                }
            }
        }
    });

    loop {
        match connection.reciever.recv().await {
            Some(connection_message) => match connection_message {
                ConnectionMessage::Connect(socket, sender) => {
                    connections_list.lock().await.insert(socket, sender);
                }
                ConnectionMessage::Disconnect(socket) => {
                    connections_list.lock().await.remove(&socket);
                    players_list.remove(&socket);
                }
                ConnectionMessage::Message(socket, message) => {
                    message_handler(
                        message,
                        &socket,
                        &*connections_list.lock().await,
                        &mut players_list,
                        &mut *game.lock().await,
                    )
                    .await;
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
    players_list: &mut HashMap<SocketAddr, String>,
    game: &mut Game,
) {
    match message {
        Message::Ok => (),
        Message::Error(error_message) => println!("{:?}", error_message),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => todo!(),
        Message::LogIn(log_in_message) => {
            players_list.insert(socket.clone(), log_in_message.player.clone());

            let messages = game.get_info();
            for message in messages {
                connections_list
                    .get(&socket)
                    .expect("Not found sender")
                    .send(message)
                    .await
                    .unwrap();
            }

            let message = game.add_player(log_in_message.player);
            for (_, sender) in connections_list {
                sender.send(message.clone()).await.unwrap()
            }
        }
        Message::Build(build_message) => match players_list.get(socket) {
            Some(player) => {
                let messages = game.build(build_message, player.clone());

                for message in messages {
                    for (_, sender) in connections_list {
                        sender.send(message.clone()).await.unwrap();
                    }
                }
            }
            None => {
                connections_list.get(&socket).expect("Not found sender").send(Message::Error(
                        network_core::bytes_represented::error_message::ErrorMessage::OperationDenied
                    )).await.unwrap();
            }
        },
        Message::Destroy(destroy_message) => match players_list.get(socket) {
            Some(player) => {
                let messages = game.destroy(destroy_message, player.clone());

                for message in messages {
                    for (_, sender) in connections_list {
                        sender.send(message.clone()).await.unwrap();
                    }
                }
            }
            None => {
                connections_list.get(&socket).expect("Not found sender").send(Message::Error(
                        network_core::bytes_represented::error_message::ErrorMessage::OperationDenied
                    )).await.unwrap();
            }
        },
        Message::Grab(grab_message) => match players_list.get(socket) {
            Some(player) => {
                let messages = game.grab(grab_message, player.clone());

                for message in messages {
                    for (_, sender) in connections_list {
                        sender.send(message.clone()).await.unwrap();
                    }
                }
            }
            None => {
                connections_list.get(&socket).expect("Not found sender").send(Message::Error(
                        network_core::bytes_represented::error_message::ErrorMessage::OperationDenied
                    )).await.unwrap();
            }
        },
        Message::SetTriangle(_) => connections_list
            .get(socket)
            .expect("Not found sender")
            .send(Message::Error(
                network_core::bytes_represented::error_message::ErrorMessage::UnexpectedMessage,
            ))
            .await
            .unwrap(),
        Message::PlayerState(_) => connections_list
            .get(socket)
            .expect("Not found sender")
            .send(Message::Error(
                network_core::bytes_represented::error_message::ErrorMessage::UnexpectedMessage,
            ))
            .await
            .unwrap(),
    }
}
