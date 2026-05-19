//! Board definitions

use std::sync::Arc;

use core_3c::{board::Board, vector::Vector};
use sfml::graphics::{Drawable, RcSprite, Transformable};
use tokio::sync::Mutex;

use crate::texture_pack::TexturePack;

/// A board's box
pub struct BoardBox {
    board: Arc<Mutex<Board>>,
    texture_pack: TexturePack,
}

impl<'shader> BoardBox {
    /// Returns new board box
    pub fn new(board: Arc<Mutex<Board>>, texture_pack: TexturePack) -> Self {
        Self {
            board,
            texture_pack,
        }
    }

    /// Draws simple triangle of the board
    fn draw_triangle<'a: 'shader, 'texture, 'shader_texture>(
        &'a self,
        board: &Board,
        triangle_position: Vector,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let texture = self
            .texture_pack
            .texture(
                board
                    .triangle(triangle_position)
                    .expect("out of board's bounds while drawing"),
            )
            .expect("not found texture");

        let mut sprite = RcSprite::with_texture(texture);
        sprite.set_origin((16.0, 16.0));
        sprite.set_position((0.0, 0.0));
        target.draw_with_renderstates(&sprite, states);
    }
}

impl Drawable for BoardBox {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut render_states = states.clone();

        let board = loop {
            if let Ok(board) = self.board.try_lock() {
                break board;
            }
        };

        for y in 0..board.scale().y {
            for x in 0..board.scale().x {
                let is_rotate = ((x % 2 == 0) && (y % 2 == 1)) || ((x % 2 == 1) && (y % 2 == 0));
                if is_rotate {
                    render_states.transform.rotate(180.0)
                }
                self.draw_triangle(&board, Vector { x, y }, target, &render_states);
                if is_rotate {
                    render_states.transform.rotate(180.0)
                }
                render_states.transform.translate(16.0, 0.0);
            }
            render_states
                .transform
                .translate(-(16.0 * board.scale().x as f32), 31.0);
        }
    }
}
