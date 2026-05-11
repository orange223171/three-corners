use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use core_3c::{
    board::{Board, Triangle},
    game::Game,
    kit::Kit,
    vector::Vector,
};
use network_client::connection::Connection;
use network_core::{
    bytes_represented::{
        build_message::BuildMessage, destroy_message::DestroyMessage, log_in_message,
    },
    message::Message,
};
use sfml::{
    cpp::FBox,
    graphics::{Color, RcSprite, RenderTarget, RenderWindow, Transformable},
    window::{ContextSettings, Event, Style, VideoMode, mouse::Button},
};
use tokio::sync::{Mutex, mpsc};

use crate::texture_pack::TexturePack;

mod texture_pack;

#[tokio::main]
async fn main() {
    let (mut window, game_mutex, connection, texture_pack) = init();

    tokio::spawn(handle_message_loop(
        game_mutex.clone(),
        connection.reciever,
        connection.sender.clone(),
    ));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            handler_sfml_event(
                event,
                &mut window,
                game_mutex.clone(),
                connection.sender.clone(),
            )
            .await;
        }

        draw(&mut window, &*game_mutex.lock().await, &texture_pack);
    }
}

fn init() -> (
    FBox<RenderWindow>,
    Arc<Mutex<Game>>,
    Connection,
    TexturePack,
) {
    let window = RenderWindow::new(
        VideoMode::new(800, 600, 32),
        "three corners",
        Style::CLOSE,
        &ContextSettings::default(),
    )
    .unwrap();

    let game = Game {
        board: Board::new(
            Vector { x: 11, y: 10 },
            Kit::from_files(String::from("core_3c/data/")).expect("Error to load game kit"),
        ),
        player_states: HashMap::new(),
    };

    let texture_pack = TexturePack::from_kit(game.board.kit());

    let game_mutex = Arc::new(Mutex::new(game));

    let connection = Connection::init(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ))
    .unwrap();

    (window, game_mutex, connection, texture_pack)
}

fn draw(window: &mut RenderWindow, game: &Game, texture_pack: &TexturePack) {
    window.clear(Color::rgb(255, 127, 127));

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

async fn handler_sfml_event(
    event: Event,
    window: &mut RenderWindow,
    game: Arc<Mutex<Game>>,
    sender: mpsc::Sender<Message>,
) {
    match event {
        Event::Closed => window.close(),
        Event::Resized { width, height } => (),
        Event::LostFocus => (),
        Event::GainedFocus => (),
        Event::TextEntered { unicode } => (),
        Event::KeyPressed {
            code,
            scan,
            alt,
            ctrl,
            shift,
            system,
        } => (),
        Event::KeyReleased {
            code,
            scan,
            alt,
            ctrl,
            shift,
            system,
        } => (),
        Event::MouseWheelScrolled { wheel, delta, x, y } => (),
        Event::MouseButtonPressed { button, x, y } => {
            if (x < 100) || (y < 84) || (x > 276) || (y > 394) {
                ()
            } else {
                let x = x - 100;
                let y = y - 84;

                let x = if x % 16 <= 7 {
                    (x / 16) as u32
                } else {
                    (x / 16 + 1) as u32
                };
                let y = (y / 31) as u32;

                let location = Vector { x, y };

                if button == Button::Left {
                    sender
                        .send(Message::Build(BuildMessage {
                            location: location,
                            build_name: String::from("field"),
                        }))
                        .await
                        .unwrap()
                }

                if button == Button::Right {
                    sender
                        .send(Message::Destroy(DestroyMessage { location: location }))
                        .await
                        .unwrap()
                }
            }
        }
        Event::MouseButtonReleased { button, x, y } => (),
        Event::MouseMoved { x, y } => (),
        Event::MouseEntered => (),
        Event::MouseLeft => (),
        Event::JoystickButtonPressed { joystickid, button } => (),
        Event::JoystickButtonReleased { joystickid, button } => (),
        Event::JoystickMoved {
            joystickid,
            axis,
            position,
        } => (),
        Event::JoystickConnected { joystickid } => (),
        Event::JoystickDisconnected { joystickid } => (),
        Event::TouchBegan { finger, x, y } => (),
        Event::TouchMoved { finger, x, y } => (),
        Event::TouchEnded { finger, x, y } => (),
        Event::SensorChanged { type_, x, y, z } => (),
        _ => (),
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
                handler_message(message, game_mutex.clone(), sender.clone()).await;
            }
            None => break,
        }
    }
}

async fn handler_message(
    message: Message,
    game_mutex: Arc<Mutex<Game>>,
    sender: mpsc::Sender<Message>,
) {
    match message {
        Message::Ok => (),
        Message::Error(error_message) => todo!(),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => (),
        Message::LogIn(_) => (),
        Message::Build(_) => (),
        Message::Destroy(_) => (),
        Message::Grab(_) => (),
        Message::SetTriangle(set_triangle_message) => {
            game_mutex
                .lock()
                .await
                .board
                .set_triangle(set_triangle_message.triangle, set_triangle_message.location)
                .unwrap();
        }
    }
}
