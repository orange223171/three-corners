use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use core_3c::{
    board::{Board, Triangle},
    kit::Kit,
    vector::Vector,
};
use network_client::connection::Connection;
use sfml::{
    graphics::{Color, RcSprite, RcTexture, RenderTarget, RenderWindow, Transformable},
    window::{ContextSettings, Event, Style, VideoMode},
};

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

    let mut board = Board::new(
        Vector { x: 11, y: 10 },
        Kit::from_files(String::from("core_3c/data/")).unwrap(),
    );

    let texture_pack = TexturePack::from_kit(board.kit());

    let connection = Connection::init(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        23171,
    ))
    .unwrap();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        window.clear(Color::rgb(255, 127, 127));

        let texture =
            RcTexture::from_file("client/sprites/triangle.png").expect("triangle.png not found");

        for i in 0..11 {
            for j in 0..10 {
                draw_triangle(
                    &mut window,
                    board
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
