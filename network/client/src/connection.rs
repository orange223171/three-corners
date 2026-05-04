use std::net::SocketAddr;
use std::net::TcpStream as StdTcpStream;

use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio::{io::AsyncReadExt, net::TcpStream};

use network_core::message::Message;

/// A clint connection
pub struct Connection {
    /// A reciever for recieve Message from channel
    pub reciever: mpsc::Receiver<Message>,
    /// A sender for send Message to channel
    pub sender: mpsc::Sender<Message>,
}

impl Connection {
    pub fn from(socket: &SocketAddr) -> Result<Connection, std::io::Error> {
        let (connection_sender, handler_reciever) = mpsc::channel::<Message>(32);
        let (handler_sender, connection_reciever) = mpsc::channel::<Message>(32);

        let stream = TcpStream::from_std(StdTcpStream::connect(socket)?)?;

        let (reading_stream, writing_stream) = stream.into_split();

        tokio::spawn(Self::reading_loop(reading_stream, connection_sender));
        tokio::spawn(Self::writing_loop(writing_stream, connection_reciever));

        Result::Ok(Connection {
            reciever: handler_reciever,
            sender: handler_sender,
        })
    }

    async fn reading_loop(mut stream: OwnedReadHalf, sender: mpsc::Sender<Message>) {
        loop {
            let mut buf: [u8; 8192] = [0; 8192];
            match stream.read(&mut buf).await {
                Ok(0) => break,
                Ok(_) => (),
                Err(_) => (),
            }

            match sender
                .send(Message::from_bytes(&buf).expect("wrong message"))
                .await
            {
                Ok(_) => (),
                Err(_) => break,
            }
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
