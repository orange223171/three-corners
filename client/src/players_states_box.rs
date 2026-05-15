//! Players' states box definitions

use sfml::graphics::Drawable;

/// A players' states box
pub struct PlayersStatesBox {}

impl Drawable for PlayersStatesBox {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        todo!()
    }
}
