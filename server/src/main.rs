use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use db::Db;
use network_core::message::Message;
use network_server::connection::{Connection, ConnectionMessage};
use tokio::{
    sync::{Mutex, mpsc},
    time::{self},
};

use logic_3c::game::Game;
use totp_rs::Secret;

use crate::message_handlers::{
    build_message_handler, destroy_message_handler, error_message_handler, grab_message_handler,
    log_in_message_handler, player_state_message_handler, set_triangle_message_handler,
    sign_up_message_handler,
};

mod message_handlers;

#[tokio::main]
async fn main() {
    let secret = Secret::generate_secret();
    println!("{}", secret.to_string());
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

    let mut db = Db::init().await.expect("Error to connect to database");

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
                        &mut db,
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
    db: &mut Db,
) {
    match message {
        Message::Ok => (),
        Message::Error(error_message) => error_message_handler(error_message),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => todo!(),
        Message::LogIn(log_in_message) => {
            log_in_message_handler(
                log_in_message,
                socket,
                connections_list,
                players_list,
                game,
                db,
            )
            .await
        }
        Message::SignUp(sign_up_message) => {
            sign_up_message_handler(sign_up_message, socket, connections_list, db).await
        }
        Message::Build(build_message) => {
            build_message_handler(build_message, socket, connections_list, players_list, game).await
        }
        Message::Destroy(destroy_message) => {
            destroy_message_handler(
                destroy_message,
                socket,
                connections_list,
                players_list,
                game,
            )
            .await
        }
        Message::Grab(grab_message) => {
            grab_message_handler(grab_message, socket, connections_list, players_list, game).await
        }
        Message::SetTriangle(_) => set_triangle_message_handler(socket, connections_list).await,
        Message::PlayerState(_) => player_state_message_handler(socket, connections_list).await,
    }
}
