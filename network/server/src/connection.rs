use network_core::message::Message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpListener,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc,
};

use std::net::{SocketAddr, TcpListener as StdTcpListener};

/// A message for updating connections list
pub enum ConnectionMessage {
    Connect(SocketAddr, mpsc::Sender<Message>),
    Disconnect(SocketAddr),
    Message(SocketAddr, Message),
}

/// A server connection
pub struct Connection {
    pub reciever: mpsc::Receiver<ConnectionMessage>,
}

impl Connection {
    pub fn init(socket: SocketAddr) -> Self {
        let (read_sender, read_reciever) = mpsc::channel::<ConnectionMessage>(32);

        let std_tcp_listener = StdTcpListener::bind(socket).expect("Error to bind server");
        std_tcp_listener
            .set_nonblocking(true)
            .expect("Error to set nonblocking");

        tokio::spawn(Self::connecting_loop(
            TcpListener::from_std(std_tcp_listener).expect("Error to create async TcpListener"),
            read_sender,
        ));

        Self {
            reciever: read_reciever,
        }
    }

    async fn connecting_loop(listener: TcpListener, read_sender: mpsc::Sender<ConnectionMessage>) {
        loop {
            match listener.accept().await {
                Ok((stream, socket)) => {
                    let (reading_stream, writing_stream) = stream.into_split();

                    let (write_sender, write_reciever) = mpsc::channel::<Message>(32);

                    match read_sender
                        .send(ConnectionMessage::Connect(socket, write_sender))
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
        read_sender: mpsc::Sender<ConnectionMessage>,
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
                .send(ConnectionMessage::Message(
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
            .send(ConnectionMessage::Disconnect(socket))
            .await
        {
            Ok(_) => (),
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
}
