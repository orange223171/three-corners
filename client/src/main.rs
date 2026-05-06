use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use core_3c::{
    board::{Board, Triangle},
    game::{self, Game},
    kit::Kit,
    vector::Vector,
};
use network_client::connection::Connection;
use network_core::message::Message;
use sfml::{
    graphics::{Color, RcSprite, RenderTarget, RenderWindow, Transformable},
    window::{ContextSettings, Event, Style, VideoMode},
};
use tokio::sync::{Mutex, mpsc};

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

    let game = Game {
        board: Board::new(Vector { x: 11, y: 10 }, kit),
        player_states: HashMap::new(),
    };
    let game_mutex = Arc::new(Mutex::new(game));

    let connection = Connection::init(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ))
    .unwrap();

    tokio::spawn(handle_message_loop(
        game_mutex.clone(),
        connection.reciever,
        connection.sender.clone(),
    ));

    connection.sender.clone().send(Message::Ok).await.unwrap();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            handler_sfml_event(
                event,
                &mut window,
                &mut *game_mutex.lock().await,
                connection.sender.clone(),
            );
        }

        window.clear(Color::rgb(255, 127, 127));

        draw(&mut window, &*game_mutex.lock().await, &texture_pack);
    }
}

fn draw(window: &mut RenderWindow, game: &Game, texture_pack: &TexturePack) {
    draw_board(window, &game.board, texture_pack);
    window.display();
}

fn draw_board(window: &mut RenderWindow, board: &Board, texture_pack: &TexturePack) {
    for i in 0..board.scale().x {
        for j in 0..board.scale().y {
            draw_triangle(
                window,
                board
                    .triangle(Vector { x: i, y: j })
                    .expect("out of bounds"),
                Vector { x: i, y: j },
                texture_pack,
            );
        }
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

fn handler_sfml_event(
    event: Event,
    window: &mut RenderWindow,
    game: &mut Game,
    sender: mpsc::Sender<Message>,
) {
    if event == Event::Closed {
        window.close();
    }
}

async fn handle_message_loop(
    game_mutex: Arc<Mutex<Game>>,
    mut reciever: mpsc::Receiver<Message>,
    sender: mpsc::Sender<Message>,
) {
    loop {
        match reciever.recv().await {
            Some(message) => {
                let mut board_lock = game_mutex.lock().await;
                handler_message(message, &mut *board_lock, sender.clone());
            }
            None => break,
        }
    }
}

fn handler_message(message: Message, game: &mut Game, sender: mpsc::Sender<Message>) {
    match message {
        Message::Ok => todo!(),
        Message::Error(error_message) => todo!(),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => todo!(),
        Message::Build(build_message) => todo!(),
        Message::Destroy(destroy_message) => todo!(),
        Message::Grab(grab_message) => todo!(),
        Message::SetTriangle(set_triangle_message) => {
            game.board
                .set_triangle(set_triangle_message.triangle, set_triangle_message.location)
                .unwrap();
        }
    }
}
