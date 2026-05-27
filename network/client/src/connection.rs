use std::net::SocketAddr;
use std::net::TcpStream as StdTcpStream;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_native_tls::TlsConnector;
use tokio_native_tls::TlsStream;

use network_core::message::Message;

/// A client connection
pub struct Connection {
    /// A reciever for recieve Message from channel
    pub reciever: mpsc::Receiver<Message>,
    /// A sender for send Message to channel
    pub sender: mpsc::Sender<Message>,
}

impl Connection {
    /// Returns a connection
    pub async fn init(socket: &SocketAddr, domain: &str) -> Result<Connection, std::io::Error> {
        let (connection_sender, handler_reciever) = mpsc::channel::<Message>(32);
        let (handler_sender, connection_reciever) = mpsc::channel::<Message>(32);

        let std_stream = StdTcpStream::connect(socket)?;
        std_stream
            .set_nonblocking(true)
            .expect("Error to set stream nonblocking");
        let tcp_stream = TcpStream::from_std(std_stream)?;

        let native_connector = native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let tls_connector: TlsConnector = native_connector.into();

        let tls_stream = tls_connector
            .connect(domain, tcp_stream)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let (reading_stream, writing_stream) = tokio::io::split(tls_stream);

        tokio::spawn(Self::reading_loop(reading_stream, connection_sender));
        tokio::spawn(Self::writing_loop(writing_stream, connection_reciever));

        Result::Ok(Connection {
            reciever: handler_reciever,
            sender: handler_sender,
        })
    }

    async fn reading_loop(
        mut stream: ReadHalf<TlsStream<TcpStream>>,
        sender: mpsc::Sender<Message>,
    ) {
        loop {
            let mut buf: [u8; 8192] = [0; 8192];
            match stream.read(&mut buf).await {
                Ok(0) => break,
                Ok(_) => (),
                Err(_) => break,
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

    async fn writing_loop(
        mut stream: WriteHalf<TlsStream<TcpStream>>,
        mut reciever: mpsc::Receiver<Message>,
    ) {
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
