use std::{collections::HashMap, net::SocketAddr};

use db::Db;
use logic_3c::game::Game;
use network_core::{
    bytes_represented::{
        add_2fa_responce_message::Add2faResponceMessage, build_message::BuildMessage,
        destroy_message::DestroyMessage, error_message::ErrorMessage, grab_message::GrabMessage,
        log_in_message::LogInMessage, sign_up_message::SignUpMessage,
        totp_responce_message::TotpResponceMessage,
    },
    message::Message,
};
use tokio::sync::mpsc;
use totp_rs::{Algorithm, Secret, TOTP};

/// Handles error message
pub fn error_message_handler(message: ErrorMessage) {
    println!("{:?}", message)
}

/// Handles log in message
pub async fn log_in_message_handler(
    message: LogInMessage,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    players_list: &mut HashMap<SocketAddr, String>,
    unauthorized_players_list: &mut HashMap<SocketAddr, String>,
    game: &mut Game,
    db: &mut Db,
) {
    match db.get_hash(message.player.clone()).await {
        Ok(hash) => {
            let hash = match hash {
                Some(hash) => hash,
                None => {
                    connections_list
                    .get(socket)
                    .expect("Not found sender")
                    .send(Message::Error(
                        network_core::bytes_represented::error_message::ErrorMessage::FailToLogIn
                    ))
                    .await
                    .unwrap();

                    return;
                }
            };

            if !bcrypt::verify(message.password.clone(), hash.as_str())
                .expect("Error to hash password")
            {
                match db.get_totp_secret(message.player.clone()).await {
                    Ok(totp_secret) => {
                        if totp_secret.is_some() {
                            unauthorized_players_list.insert(*socket, message.player.clone());

                            connections_list
                                .get(socket)
                                .expect("Not found sender")
                                .send(Message::TotpRequest)
                                .await
                                .unwrap();

                            return;
                        }
                    }
                    Err(_) => {
                        connections_list
                        .get(socket)
                        .expect("Not found sender")
                        .send(Message::Error(
                            network_core::bytes_represented::error_message::ErrorMessage::FailToLogIn,
                        ))
                        .await
                        .unwrap();

                        return;
                    }
                }

                connections_list
                    .get(socket)
                    .expect("Not found sender")
                    .send(Message::Error(
                        network_core::bytes_represented::error_message::ErrorMessage::FailToLogIn,
                    ))
                    .await
                    .unwrap();

                return;
            }
        }
        Err(_) => {
            connections_list
                .get(socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::FailToLogIn,
                ))
                .await
                .unwrap();

            return;
        }
    }

    players_list.insert(socket.clone(), message.player.clone());

    let messages = game.get_info();
    for message in messages {
        connections_list
            .get(&socket)
            .expect("Not found sender")
            .send(message)
            .await
            .unwrap();
    }

    let message = game.add_player(message.player);
    for (_, sender) in connections_list {
        sender.send(message.clone()).await.unwrap()
    }
}

/// Handles sign up message
pub async fn sign_up_message_handler(
    message: SignUpMessage,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    db: &mut Db,
) {
    if db
        .get_hash(message.player.clone())
        .await
        .expect("Error to access db")
        != None
    {
        connections_list
            .get(socket)
            .expect("Not found sender")
            .send(Message::Error(
                network_core::bytes_represented::error_message::ErrorMessage::FailToSignUp,
            ))
            .await
            .unwrap();

        return;
    }

    let hash = bcrypt::hash(message.password.clone(), bcrypt::DEFAULT_COST)
        .expect("Error to hash password");

    db.add_user(message.player, hash)
        .await
        .expect("Error to access db");
}

pub async fn totp_responce_message_handler(
    message: TotpResponceMessage,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    players_list: &mut HashMap<SocketAddr, String>,
    unauthorized_players_list: &mut HashMap<SocketAddr, String>,
    game: &mut Game,
    db: &mut Db,
) {
    let user = match unauthorized_players_list.get(&socket) {
        Some(user) => user.clone(),
        None => {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                ))
                .await
                .unwrap();

            return;
        }
    };

    let secret = match db.get_totp_secret(user.clone()).await {
        Ok(secret) => match secret {
            Some(secret) => secret,
            None => {
                connections_list
                        .get(&socket)
                        .expect("Not found sender")
                        .send(Message::Error(
                            network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                        ))
                        .await
                        .unwrap();

                return;
            }
        },
        Err(_) => {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                ))
                .await
                .unwrap();

            return;
        }
    };

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret).to_bytes().unwrap(),
    )
    .unwrap();
    let token = totp.generate_current().unwrap();

    if token == message.totp_code {
        unauthorized_players_list.remove(socket);
        players_list.insert(socket.clone(), user.clone());

        let messages = game.get_info();
        for message in messages {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(message)
                .await
                .unwrap();
        }

        let message = game.add_player(user);
        for (_, sender) in connections_list {
            sender.send(message.clone()).await.unwrap()
        }
    }
}

pub async fn add_2fa_request_message_handler(
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    players_list: &mut HashMap<SocketAddr, String>,
    db: &mut Db,
) {
    let secret = Secret::generate_secret().to_string();

    let user = match players_list.get(&socket) {
        Some(user) => user.clone(),
        None => {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                ))
                .await
                .unwrap();

            return;
        }
    };

    db.add_2fa(user, secret.clone());

    connections_list
        .get(&socket)
        .expect("Not found sender")
        .send(Message::Add2faResponce(Add2faResponceMessage {
            secret: secret,
        }))
        .await
        .unwrap()
}

pub async fn remove_2fa_message_handler(
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    players_list: &mut HashMap<SocketAddr, String>,
    db: &mut Db,
) {
    let user = match players_list.get(&socket) {
        Some(user) => user.clone(),
        None => {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                ))
                .await
                .unwrap();

            return;
        }
    };

    db.remove_2fa(user);
}

pub async fn build_message_handler(
    message: BuildMessage,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    players_list: &mut HashMap<SocketAddr, String>,
    game: &mut Game,
) {
    match players_list.get(socket) {
        Some(player) => {
            let messages = game.build(message, player.clone());

            for message in messages {
                for (_, sender) in connections_list {
                    sender.send(message.clone()).await.unwrap();
                }
            }
        }
        None => {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                ))
                .await
                .unwrap();
        }
    }
}

pub async fn destroy_message_handler(
    message: DestroyMessage,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    players_list: &mut HashMap<SocketAddr, String>,
    game: &mut Game,
) {
    match players_list.get(socket) {
        Some(player) => {
            let messages = game.destroy(message, player.clone());

            for message in messages {
                for (_, sender) in connections_list {
                    sender.send(message.clone()).await.unwrap();
                }
            }
        }
        None => {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                ))
                .await
                .unwrap();
        }
    }
}

pub async fn grab_message_handler(
    message: GrabMessage,
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
    players_list: &mut HashMap<SocketAddr, String>,
    game: &mut Game,
) {
    match players_list.get(socket) {
        Some(player) => {
            let messages = game.grab(message, player.clone());

            for message in messages {
                for (_, sender) in connections_list {
                    sender.send(message.clone()).await.unwrap();
                }
            }
        }
        None => {
            connections_list
                .get(&socket)
                .expect("Not found sender")
                .send(Message::Error(
                    network_core::bytes_represented::error_message::ErrorMessage::OperationDenied,
                ))
                .await
                .unwrap();
        }
    }
}

pub async fn set_triangle_message_handler(
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
) {
    connections_list
        .get(socket)
        .expect("Not found sender")
        .send(Message::Error(
            network_core::bytes_represented::error_message::ErrorMessage::UnexpectedMessage,
        ))
        .await
        .unwrap()
}

pub async fn player_state_message_handler(
    socket: &SocketAddr,
    connections_list: &HashMap<SocketAddr, mpsc::Sender<Message>>,
) {
    connections_list
        .get(socket)
        .expect("Not found sender")
        .send(Message::Error(
            network_core::bytes_represented::error_message::ErrorMessage::UnexpectedMessage,
        ))
        .await
        .unwrap()
}
