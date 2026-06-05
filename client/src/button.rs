//! Button widget definitions

use sfml::{
    cpp::FBox,
    graphics::{
        Color, Drawable, Font, RectangleShape, RenderStates, RenderTarget, Shape, Text,
        Transformable,
    },
};

/// A button
pub struct Button {
    label: String,
    position: (f32, f32),
    size: (f32, f32),
    hovered: bool,
    font: FBox<Font>,
}

impl Button {
    /// Returns new button
    pub fn new(label: &str, position: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            label: label.to_string(),
            position,
            size,
            hovered: false,
            font: Font::from_file("/usr/share/fonts/TTF/DejaVuSans.ttf")
                .expect("Error to load font"),
        }
    }

    /// Returns true if point (x, y) is inside the button
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.position.0
            && x <= self.position.0 + self.size.0
            && y >= self.position.1
            && y <= self.position.1 + self.size.1
    }

    /// Sets the button hovered
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    /// Returns positions of the button
    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    /// Returns size of the button
    pub fn size(&self) -> (f32, f32) {
        self.size
    }

    /// Returns true if button is hovered
    pub fn hovered(&self) -> bool {
        self.hovered
    }
}

impl Drawable for Button {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        // Background
        let mut bg = RectangleShape::new();
        bg.set_position(self.position);
        bg.set_size(self.size);
        if self.hovered {
            bg.set_fill_color(Color::rgb(90, 140, 220));
            bg.set_outline_color(Color::rgb(60, 100, 180));
        } else {
            bg.set_fill_color(Color::rgb(70, 120, 200));
            bg.set_outline_color(Color::rgb(50, 90, 160));
        }
        bg.set_outline_thickness(2.0);
        target.draw_with_renderstates(&bg, states);

        // Label centered
        let mut text = Text::new(&self.label, &*self.font, 20);
        text.set_fill_color(Color::WHITE);
        let bounds = text.local_bounds();
        text.set_position((
            self.position.0 + (self.size.0 - bounds.width) / 2.0,
            self.position.1 + (self.size.1 - bounds.height) / 2.0 - 4.0,
        ));
        target.draw_with_renderstates(&text, states);
    }
}
