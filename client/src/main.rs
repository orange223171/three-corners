use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style, VideoMode},
};

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600, 32),
        "three corners",
        Style::CLOSE,
        &ContextSettings::default(),
    )
    .unwrap();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        window.clear(Color::rgb(255, 127, 127));
        window.display();
    }
}
