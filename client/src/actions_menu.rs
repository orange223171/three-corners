//! Actions menu definitions

use core_3c::{info::building, vector::Vector};
use network_core::{
    bytes_represented::{
        build_message::BuildMessage, destroy_message::DestroyMessage, grab_message::GrabMessage,
    },
    message::Message,
};
use sfml::{
    cpp::FBox,
    graphics::{Drawable, Font, RcText, RenderStates, Text, Transform, Vertex},
    system::{SfStr, Vector2},
};
use tokio::sync::mpsc::Sender;

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
    /// triangle location
    location: Option<Vector>,
}

impl ActionsMenu {
    /// Returns empty actions menu
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            location: None,
        }
    }

    pub fn set_location(&mut self, location: Vector) {
        self.location = Some(location);
    }

    /// Clears actions menu
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Add action to the actions menu
    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Handles button pressing
    pub async fn handle_button_pressing(&self, x: i32, y: i32, sender: Sender<Message>) {
        let location = if let Some(location) = self.location {
            location
        } else {
            return;
        };

        let action_n = y / ACTION_SIZE as i32;
        match self.actions.get(action_n as usize) {
            Some(action) => sender
                .send(match action {
                    Action::Build(build_name, _) => Message::Build(BuildMessage {
                        location: location,
                        build_name: build_name.clone(),
                    }),
                    Action::Grab(_) => Message::Grab(GrabMessage { location }),
                    Action::Destroy(_) => Message::Destroy(DestroyMessage { location }),
                })
                .await
                .unwrap(),
            None => (),
        }
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
