//! Board definitions

use sfml::graphics::Drawable;

/// A board
pub struct Board {}

impl Drawable for Board {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        todo!()
    }
}
