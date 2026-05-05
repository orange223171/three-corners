use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use network_client::connection::Connection;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style, VideoMode},
};

#[tokio::main]
async fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600, 32),
        "three corners",
        Style::CLOSE,
        &ContextSettings::default(),
    )
    .unwrap();

    let connection = Connection::init(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        window.clear(Color::rgb(255, 127, 127));
        window.display();
    }
}
