use network_core::message::Message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpListener,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc,
};

use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener as StdTcpListener},
};

/// A message for updating connections list
enum ConnectionsMessage {
    Connected(SocketAddr, mpsc::Sender<Message>),
    Disconnected(SocketAddr),
    Message(SocketAddr, Message),
}

/// A server connection
pub struct Connection {}

impl Connection {
    pub fn init<
        F: FnMut(Message, &SocketAddr, &HashMap<SocketAddr, mpsc::Sender<Message>>) + Send + 'static,
    >(
        socket: SocketAddr,
        handler: F,
    ) {
        let (read_sender, read_reciever) = mpsc::channel::<ConnectionsMessage>(32);

        tokio::spawn(Self::handling_loop(handler, read_reciever));

        tokio::spawn(Self::connecting_loop(
            TcpListener::from_std(StdTcpListener::bind(socket).expect("Error to bind server"))
                .expect("Error to create async TcpListener"),
            read_sender,
        ));
    }

    async fn connecting_loop(listener: TcpListener, read_sender: mpsc::Sender<ConnectionsMessage>) {
        loop {
            match listener.accept().await {
                Ok((stream, socket)) => {
                    let (reading_stream, writing_stream) = stream.into_split();

                    let (write_sender, write_reciever) = mpsc::channel::<Message>(32);

                    match read_sender
                        .send(ConnectionsMessage::Connected(socket, write_sender))
                        .await
                    {
                        Ok(_) => (),
                        Err(_) => (),
                    }

                    tokio::spawn(Self::reading_loop(
                        reading_stream,
                        read_sender.clone(),
                        socket,
                    ));
                    tokio::spawn(Self::writing_loop(writing_stream, write_reciever));
                }
                Err(_) => break,
            }
        }
    }

    async fn reading_loop(
        mut stream: OwnedReadHalf,
        read_sender: mpsc::Sender<ConnectionsMessage>,
        socket: SocketAddr,
    ) {
        loop {
            let mut buf: [u8; 8192] = [0; 8192];
            match stream.read(&mut buf).await {
                Ok(0) => break,
                Ok(_) => (),
                Err(_) => break,
            }

            match read_sender
                .send(ConnectionsMessage::Message(
                    socket,
                    Message::from_bytes(&buf).expect("wrong message"),
                ))
                .await
            {
                Ok(_) => (),
                Err(_) => break,
            }
        }

        match read_sender
            .send(ConnectionsMessage::Disconnected(socket))
            .await
        {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    async fn writing_loop(mut stream: OwnedWriteHalf, mut reciever: mpsc::Receiver<Message>) {
        loop {
            match reciever.recv().await {
                Some(message) => match stream.write(&message.as_bytes()).await {
                    Ok(_) => (),
                    Err(_) => break,
                },
                None => break,
            }
        }
    }

    async fn handling_loop<
        F: FnMut(Message, &SocketAddr, &HashMap<SocketAddr, mpsc::Sender<Message>>),
    >(
        mut message_handler: F,
        mut reciever: mpsc::Receiver<ConnectionsMessage>,
    ) {
        let mut connections_list: HashMap<SocketAddr, mpsc::Sender<Message>> = HashMap::new();

        loop {
            match reciever.recv().await {
                Some(connection_message) => match connection_message {
                    ConnectionsMessage::Connected(socket, sender) => {
                        connections_list.insert(socket, sender);
                    }
                    ConnectionsMessage::Disconnected(socket) => {
                        connections_list.remove(&socket);
                    }
                    ConnectionsMessage::Message(socket, message) => {
                        message_handler(message, &socket, &connections_list);
                    }
                },
                None => break,
            }
        }
    }
}
