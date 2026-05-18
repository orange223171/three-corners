//! Board definitions

use core_3c::{board::Board, vector::Vector};
use sfml::{
    graphics::{Drawable, RcSprite, Rect, Sprite, Transform, Transformable},
    system::Vector2f,
};

use crate::texture_pack::TexturePack;

/// A board's box
pub struct BoardBox {
    board: Board,
    texture_pack: TexturePack,
}

impl<'shader> BoardBox {
    fn draw_triangle<'a: 'shader, 'texture, 'shader_texture>(
        &'a self,
        triangle_position: Vector,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let texture = self
            .texture_pack
            .texture(
                self.board
                    .triangle(triangle_position)
                    .expect("out of board's bounds while drawing"),
            )
            .expect("not found texture");

        let mut sprite = RcSprite::with_texture(texture);
        sprite.set_origin((16.0, 16.0));
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

        for y in 0..self.board.scale().y {
            render_states
                .transform
                .translate(-(16.0 * self.board.scale().x as f32), 0.0);
            for x in 0..self.board.scale().x {
                self.draw_triangle(Vector { x, y }, target, &render_states);
                render_states.transform.translate(16.0, 0.0);
            }
            render_states.transform.translate(0.0, 31.0);
        }
    }
}
