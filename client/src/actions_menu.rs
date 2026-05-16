//! Actions menu definitions

use core_3c::info::building;
use sfml::{
    cpp::FBox,
    graphics::{Drawable, Font, RcText, RenderStates, Text, Transform, Vertex},
    system::{SfStr, Vector2},
};

/// Action size in pixels
const ACTION_SIZE: u32 = 32;

/// An action
pub enum Action {
    /// A build action
    Build(String, u32),
    /// A grab action
    Grab(u32),
    /// A destroy action
    Destroy(u32),
}

impl Drawable for Action {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let action_text = match self {
            Action::Build(building, price) => {
                String::from("Построить ") + building + " " + price.to_string().as_str()
            }
            Action::Grab(price) => String::from("Ограбить ") + price.to_string().as_str(),
            Action::Destroy(price) => String::from("Уничтожить ") + price.to_string().as_str(),
        };

        let font =
            Font::from_file("/usr/share/fonts/TTF/DejaVuSans.ttf").expect("Error to load font");
        let text = Text::new(&action_text, &font, 16);

        target.draw_with_renderstates(&text, states);
    }
}

/// An actions menu
pub struct ActionsMenu {
    /// Actions
    actions: Vec<Action>,
}

impl ActionsMenu {
    /// Returns empty actions menu
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Clears actions menu
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Add action to the actions menu
    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }
}

impl Drawable for ActionsMenu {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let size = target.size();

        self.actions.iter().for_each(|action| {
            let mut render_states = states.clone();
            render_states.transform = Transform::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
            render_states
                .transform
                .translate((3 * size.x / 4) as f32, ACTION_SIZE as f32);
            target.draw_with_renderstates(action, &render_states);
        });
    }
}
