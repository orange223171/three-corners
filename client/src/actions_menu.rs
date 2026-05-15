//! Actions menu definitions

use sfml::graphics::Drawable;

/// An actions menu
pub struct ActionsMenu {}

impl Drawable for ActionsMenu {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        todo!()
    }
}
