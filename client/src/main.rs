use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use core_3c::{
    board::{Board, Triangle},
    kit::Kit,
    vector::Vector,
};
use network_client::connection::Connection;
use network_core::message::Message;
use sfml::{
    graphics::{Color, RcSprite, RenderTarget, RenderWindow, Transformable},
    window::{ContextSettings, Event, Style, VideoMode},
};
use tokio::sync::Mutex;

use crate::texture_pack::TexturePack;

mod texture_pack;

#[tokio::main]
async fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600, 32),
        "three corners",
        Style::CLOSE,
        &ContextSettings::default(),
    )
    .unwrap();

    let kit = Kit::from_files(String::from("core_3c/data/")).unwrap();
    let texture_pack = TexturePack::from_kit(&kit);

    let board = Board::new(Vector { x: 11, y: 10 }, kit);
    let board_mutex_connection = Arc::new(Mutex::new(board));
    let board_mutex_drawing = board_mutex_connection.clone();

    let mut connection = Connection::init(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ))
    .unwrap();

    tokio::spawn(async move {
        loop {
            match connection.reciever.recv().await {
                Some(message) => {
                    let mut board_lock = board_mutex_connection.lock().await;
                    handler_message(message, &mut *board_lock);
                }
                None => break,
            }
        }
    });

    connection.sender.send(Message::Ok).await.unwrap();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        window.clear(Color::rgb(255, 127, 127));

        for i in 0..11 {
            for j in 0..10 {
                let board_lock = board_mutex_drawing.lock().await;
                draw_triangle(
                    &mut window,
                    &*board_lock
                        .triangle(Vector { x: i, y: j })
                        .expect("out of bounds"),
                    Vector { x: i, y: j },
                    &texture_pack,
                );
            }
        }

        window.display();
    }
}

fn draw_triangle(
    window: &mut RenderWindow,
    triangle: &Triangle,
    position: Vector,
    texture_pack: &TexturePack,
) {
    let mut sprite =
        RcSprite::with_texture(texture_pack.texture(triangle).expect("Not found texture"));

    sprite.set_position((
        100.0 + (position.x * 16) as f32,
        100.0 + (position.y * 31) as f32,
    ));

    sprite.set_origin((16.0, 16.0));
    if position.x % 2 == 0 {
        if position.y % 2 == 1 {
            sprite.set_rotation(180.0);
        }
    } else {
        if position.y % 2 == 0 {
            sprite.set_rotation(180.0);
        }
    }

    window.draw(&sprite);
}

fn handler_sfml_event(event: Event) {}

async fn handler_message_loop(board_mutex: Arc<Mutex<Board>>) {}

fn handler_message(message: Message, board: &mut Board) {
    match message {
        Message::Ok => todo!(),
        Message::Error(error_message) => todo!(),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => todo!(),
        Message::Build(build_message) => todo!(),
        Message::Destroy(destroy_message) => todo!(),
        Message::Grab(grab_message) => todo!(),
        Message::SetTriangle(set_triangle_message) => {
            board
                .set_triangle(set_triangle_message.triangle, set_triangle_message.location)
                .unwrap();
        }
    }
}
