//! Input field widget definitions

use sfml::{
    cpp::FBox,
    graphics::{
        Color, Drawable, Font, RectangleShape, RenderStates, RenderTarget, Shape, Text,
        Transformable,
    },
};

/// A input field
pub struct InputField {
    label: String,
    content: String,
    position: (f32, f32),
    size: (f32, f32),
    focused: bool,
    cursor_timer: f32,
    password_mode: bool,
    font: FBox<Font>,
}

impl InputField {
    /// Returns new fields
    pub fn new(label: &str, position: (f32, f32), size: (f32, f32), password_mode: bool) -> Self {
        Self {
            label: label.to_string(),
            content: String::new(),
            position,
            size,
            focused: false,
            cursor_timer: 0.0,
            password_mode,
            font: Font::from_file("/usr/share/fonts/TTF/DejaVuSans.ttf")
                .expect("Error to load font"),
        }
    }

    /// Returns content of the field
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Sets content of the field
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    /// Clears content of the field
    pub fn clear(&mut self) {
        self.content.clear();
    }

    /// Sets focus on the field
    pub fn focus(&mut self) {
        self.focused = true;
        self.cursor_timer = 0.0;
    }

    /// Remove focus from the field
    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    /// Returns true if field is focused
    pub fn focused(&self) -> bool {
        self.focused
    }

    /// Returns true if point (x, y) is inside the field
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.position.0
            && x <= self.position.0 + self.size.0
            && y >= self.position.1
            && y <= self.position.1 + self.size.1
    }

    /// Update cursor blink timer
    pub fn update(&mut self, dt: f32) {
        if self.focused {
            self.cursor_timer += dt;
            if self.cursor_timer > 1.0 {
                self.cursor_timer -= 1.0;
            }
        }
    }

    /// Handles entered contents
    pub fn handle_text_entered(&mut self, unicode: char) {
        if !self.focused {
            return;
        }
        // Printable ASCII and basic multilingual
        if (unicode as u32) >= 0x20 && unicode != '\x7f' {
            self.content.push(unicode);
        }
    }

    /// Handles backspase
    pub fn handle_backspace(&mut self) {
        if self.focused {
            self.content.pop();
        }
    }
}

impl Drawable for InputField {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        // Background
        let mut bg = RectangleShape::new();
        bg.set_position(self.position);
        bg.set_size(self.size);
        bg.set_fill_color(if self.focused {
            Color::rgb(240, 240, 255)
        } else {
            Color::rgb(220, 220, 230)
        });
        bg.set_outline_color(Color::rgb(100, 100, 120));
        bg.set_outline_thickness(2.0);
        target.draw_with_renderstates(&bg, states);

        // Label above the field
        let mut label_text = Text::new(&self.label, &*self.font, 16);
        label_text.set_fill_color(Color::rgb(60, 60, 80));
        label_text.set_position((self.position.0, self.position.1 - 22.0));
        target.draw_with_renderstates(&label_text, states);

        // Content (or masked)
        let display_text = if self.password_mode {
            "*".repeat(self.content.len())
        } else {
            self.content.clone()
        };

        let mut content_text = Text::new(&display_text, &*self.font, 20);
        content_text.set_fill_color(Color::rgb(20, 20, 40));
        content_text.set_position((self.position.0 + 6.0, self.position.1 + 4.0));
        target.draw_with_renderstates(&content_text, states);

        // Blinking cursor
        if self.focused && self.cursor_timer < 0.5 {
            let text_width = content_text.local_bounds().width;
            let cursor_x = self.position.0 + 6.0 + text_width + 1.0;

            let mut cursor = RectangleShape::new();
            cursor.set_position((cursor_x, self.position.1 + 5.0));
            cursor.set_size((2.0, self.size.1 - 10.0));
            cursor.set_fill_color(Color::rgb(40, 40, 60));
            target.draw_with_renderstates(&cursor, states);
        }
    }
}
