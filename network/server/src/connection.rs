use network_core::message::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_native_tls::{TlsAcceptor, TlsStream};

use std::fs;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::path::Path;

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
    /// Returns a connection
    pub fn init(socket: SocketAddr) -> Self {
        let (read_sender, read_reciever) = mpsc::channel::<ConnectionMessage>(32);

        let std_tcp_listener = StdTcpListener::bind(socket).expect("Error to bind server");
        std_tcp_listener
            .set_nonblocking(true)
            .expect("Error to set nonblocking");

        let password = fs::read_to_string("/etc/three_corners/server/password")
            .expect("Error to find passwors file");

        let tls_acceptor = Self::build_tls_acceptor(
            std::path::Path::new("/etc/three_corners/server/certificate.pfx"),
            password.trim(),
        );

        tokio::spawn(Self::connecting_loop(
            TcpListener::from_std(std_tcp_listener).expect("Error to create async TcpListener"),
            read_sender,
            tls_acceptor,
        ));

        Self {
            reciever: read_reciever,
        }
    }

    fn build_tls_acceptor(cert_path: &Path, cert_password: &str) -> TlsAcceptor {
        let pkcs12 = std::fs::read(cert_path)
            .expect("Error to read certificate file. Make sure the PKCS12 (.pfx) file exists.");
        let identity = native_tls::Identity::from_pkcs12(&pkcs12, cert_password)
            .expect("Error to parse certificate");
        let native_acceptor = native_tls::TlsAcceptor::builder(identity)
            .build()
            .expect("Error to build TLS acceptor");

        native_acceptor.into()
    }

    async fn connecting_loop(
        listener: TcpListener,
        read_sender: mpsc::Sender<ConnectionMessage>,
        tls_acceptor: TlsAcceptor,
    ) {
        loop {
            match listener.accept().await {
                Ok((tcp_stream, socket)) => {
                    let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                        Ok(stream) => stream,
                        Err(_) => continue,
                    };

                    let (reading_stream, writing_stream) = tokio::io::split(tls_stream);

                    let (write_sender, write_reciever) = mpsc::channel::<Message>(32);

                    match read_sender
                        .send(ConnectionMessage::Connect(socket, write_sender))
                        .await
                    {
                        Ok(_) => (),
                        Err(_) => return,
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
        mut stream: ReadHalf<TlsStream<TcpStream>>,
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
