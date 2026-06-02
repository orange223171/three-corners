//! Authentication window — handles login, signup and 2FA via RAII SFML window.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use network_client::connection::Connection;
use network_core::{
    bytes_represented::{
        log_in_message::LogInMessage, sign_up_message::SignUpMessage,
        totp_responce_message::TotpResponceMessage,
    },
    message::Message,
};
use sfml::{
    cpp::FBox,
    graphics::{
        Color, Drawable, Font, RenderStates, RenderTarget, RenderWindow, Text, Transformable,
    },
    system::Clock,
    window::{Event, Style, VideoMode, mouse::Button as SfButton},
};
use tokio::sync::mpsc;

use crate::button::Button;
use crate::input_field::InputField;

/// Which auth mode the window is in
#[derive(PartialEq, Clone, Copy)]
enum AuthMode {
    Login,
    SignUp,
}

/// Represents the current stage of authentication flow
#[derive(PartialEq, Clone)]
enum AuthStage {
    /// Entering login or signup credentials
    Credentials,
    /// TOTP code required (after login for 2FA-enabled accounts)
    Totp,
    /// Showing 2FA secret (after signup or adding 2FA)
    Add2fa { secret: String },
    /// Logged in — show Add/Remove TOTP + Continue
    LoggedIn,
    /// Authentication complete — window will close
    Done,
}

/// RAII-based authentication window.
///
/// On creation, opens an SFML window and establishes a server connection.
/// Call `run()` to start the event loop; returns `Connection` on success.
/// The window is automatically closed when the struct is dropped.
pub struct AuthWindow {
    window: FBox<RenderWindow>,
    connection: Connection,
    clock: FBox<Clock>,

    // Input fields
    login_field: InputField,
    password_field: InputField,
    totp_field: InputField,

    // Buttons
    submit_button: Button,
    toggle_mode_button: Button,
    submit_totp_button: Button,
    confirm_add2fa_button: Button,
    back_button: Button,
    continue_button: Button,
    add_totp_button: Button,
    remove_totp_button: Button,

    // State
    mode: AuthMode,
    stage: AuthStage,
    error_text: String,
    info_text: String,
}

impl AuthWindow {
    /// Creates a new auth window, connecting to the server.
    /// Uses RAII: window resources are freed on drop.
    pub async fn new() -> Self {
        let window = RenderWindow::new(
            VideoMode::new(500, 420, 32),
            "three corners — authentication",
            Style::TITLEBAR | Style::CLOSE,
            &sfml::window::ContextSettings::default(),
        )
        .unwrap();

        let connection = Connection::init(
            &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 23171),
            "localhost",
        )
        .await
        .expect("Error to connect to server");

        let field_w = 320.0;
        let field_h = 36.0;
        let field_x = 90.0;

        // Credentials stage fields
        let login_field =
            InputField::new("Player name", (field_x, 80.0), (field_w, field_h), false);
        let password_field =
            InputField::new("Password", (field_x, 150.0), (field_w, field_h), true);

        // TOTP stage field
        let totp_field = InputField::new("TOTP code", (field_x, 170.0), (field_w, field_h), false);

        let button_w = 140.0;
        let button_h = 36.0;

        let submit_button = Button::new("Submit", (175.0, 220.0), (button_w, button_h));
        let toggle_mode_button = Button::new("Sign Up", (175.0, 270.0), (button_w, button_h));
        let submit_totp_button = Button::new("Submit TOTP", (175.0, 240.0), (button_w, button_h));
        let confirm_add2fa_button = Button::new("I saved it", (155.0, 300.0), (190.0, button_h));
        let back_button = Button::new("Back", (175.0, 340.0), (button_w, button_h));
        let continue_button = Button::new("Continue", (155.0, 280.0), (190.0, button_h));
        let add_totp_button = Button::new("Add TOTP", (175.0, 180.0), (button_w, button_h));
        let remove_totp_button = Button::new("Remove TOTP", (175.0, 230.0), (button_w, button_h));

        Self {
            window,
            connection,
            clock: Clock::start().unwrap(),
            login_field,
            password_field,
            totp_field,
            submit_button,
            toggle_mode_button,
            submit_totp_button,
            confirm_add2fa_button,
            back_button,
            continue_button,
            add_totp_button,
            remove_totp_button,
            mode: AuthMode::Login,
            stage: AuthStage::Credentials,
            error_text: String::new(),
            info_text: String::new(),
        }
    }

    /// Runs the authentication event loop.
    /// Blocks until auth is complete, then returns the `Connection`.
    pub async fn run(mut self) -> Connection {
        let font =
            Font::from_file("/usr/share/fonts/TTF/DejaVuSans.ttf").expect("Error to load font");

        // Focus the first field initially
        self.login_field.focus();

        while self.window.is_open() {
            let dt = self.clock.restart().as_seconds();
            self.update_fields(dt);

            // Poll server messages
            self.poll_messages().await;

            // Stop if done
            if self.stage == AuthStage::Done {
                self.window.close();
                return self.connection;
            }

            // Handle SFML events
            while let Some(event) = self.window.poll_event() {
                self.handle_event(event);
            }

            // Draw
            self.draw(&font);
        }

        // Window closed by user — but we still return the connection
        self.connection
    }

    fn update_fields(&mut self, dt: f32) {
        self.login_field.update(dt);
        self.password_field.update(dt);
        self.totp_field.update(dt);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Closed => self.window.close(),

            Event::TextEntered { unicode } => match &self.stage {
                AuthStage::Credentials | AuthStage::Add2fa { .. } => {
                    self.login_field.handle_text_entered(unicode);
                    self.password_field.handle_text_entered(unicode);
                }
                AuthStage::Totp => {
                    self.totp_field.handle_text_entered(unicode);
                }
                AuthStage::LoggedIn | AuthStage::Done => {}
            },

            Event::KeyPressed { code, .. } => {
                use sfml::window::Key;
                match code {
                    Key::Backspace => match &self.stage {
                        AuthStage::Credentials | AuthStage::Add2fa { .. } => {
                            self.login_field.handle_backspace();
                            self.password_field.handle_backspace();
                        }
                        AuthStage::Totp => {
                            self.totp_field.handle_backspace();
                        }
                        AuthStage::LoggedIn | AuthStage::Done => {}
                    },
                    Key::Tab => {
                        // Toggle focus between login and password fields
                        if self.login_field.focused() {
                            self.login_field.unfocus();
                            self.password_field.focus();
                        } else if self.password_field.focused() {
                            self.password_field.unfocus();
                            self.login_field.focus();
                        } else {
                            self.login_field.focus();
                        }
                    }
                    Key::Enter => {
                        // Submit on Enter
                        self.submit_current_stage();
                    }
                    _ => {}
                }
            }

            Event::MouseButtonPressed { button, x, y } => {
                if button != SfButton::Left {
                    return;
                }
                let (x, y) = (x as f32, y as f32);

                match &self.stage {
                    AuthStage::Credentials => {
                        // Focus management
                        if self.login_field.contains(x, y) {
                            self.login_field.focus();
                            self.password_field.unfocus();
                        } else if self.password_field.contains(x, y) {
                            self.password_field.focus();
                            self.login_field.unfocus();
                        } else {
                            self.login_field.unfocus();
                            self.password_field.unfocus();
                        }

                        // Submit button
                        if self.submit_button.contains(x, y) {
                            self.submit_current_stage();
                        }

                        // Toggle mode button
                        if self.toggle_mode_button.contains(x, y) {
                            self.toggle_mode();
                        }
                    }

                    AuthStage::Totp => {
                        if self.totp_field.contains(x, y) {
                            self.totp_field.focus();
                        } else {
                            self.totp_field.unfocus();
                        }

                        if self.submit_totp_button.contains(x, y) {
                            self.submit_current_stage();
                        }

                        if self.back_button.contains(x, y) {
                            self.stage = AuthStage::Credentials;
                            self.totp_field.clear();
                            self.error_text.clear();
                        }
                    }

                    AuthStage::Add2fa { .. } => {
                        if self.confirm_add2fa_button.contains(x, y) {
                            // User confirmed they saved the secret → back to LoggedIn
                            self.stage = AuthStage::LoggedIn;
                            self.info_text.clear();
                        }
                    }

                    AuthStage::LoggedIn => {
                        if self.add_totp_button.contains(x, y) {
                            self.send_add2fa_request();
                        }
                        if self.remove_totp_button.contains(x, y) {
                            self.send_remove2fa();
                        }
                        if self.continue_button.contains(x, y) {
                            self.stage = AuthStage::Done;
                        }
                    }

                    AuthStage::Done => {}
                }
            }

            Event::MouseMoved { x, y } => {
                let (x, y) = (x as f32, y as f32);
                match &self.stage {
                    AuthStage::Credentials => {
                        self.submit_button
                            .set_hovered(self.submit_button.contains(x, y));
                        self.toggle_mode_button
                            .set_hovered(self.toggle_mode_button.contains(x, y));
                    }
                    AuthStage::Totp => {
                        self.submit_totp_button
                            .set_hovered(self.submit_totp_button.contains(x, y));
                        self.back_button
                            .set_hovered(self.back_button.contains(x, y));
                    }
                    AuthStage::Add2fa { .. } => {
                        self.confirm_add2fa_button
                            .set_hovered(self.confirm_add2fa_button.contains(x, y));
                    }
                    AuthStage::LoggedIn => {
                        self.add_totp_button
                            .set_hovered(self.add_totp_button.contains(x, y));
                        self.remove_totp_button
                            .set_hovered(self.remove_totp_button.contains(x, y));
                        self.continue_button
                            .set_hovered(self.continue_button.contains(x, y));
                    }
                    AuthStage::Done => {}
                }
            }

            _ => {}
        }
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            AuthMode::Login => AuthMode::SignUp,
            AuthMode::SignUp => AuthMode::Login,
        };
        self.error_text.clear();
        self.info_text.clear();
    }

    fn submit_current_stage(&mut self) {
        match &self.stage {
            AuthStage::Credentials => {
                let player = self.login_field.content().to_string();
                let password = self.password_field.content().to_string();

                if player.is_empty() || password.is_empty() {
                    self.error_text = "Both fields are required.".to_string();
                    return;
                }

                self.error_text.clear();

                match self.mode {
                    AuthMode::Login => {
                        let sender = self.connection.sender.clone();
                        tokio::spawn(async move {
                            sender
                                .send(Message::LogIn(LogInMessage { player, password }))
                                .await
                                .unwrap();
                        });
                    }
                    AuthMode::SignUp => {
                        let sender = self.connection.sender.clone();
                        tokio::spawn(async move {
                            sender
                                .send(Message::SignUp(SignUpMessage { player, password }))
                                .await
                                .unwrap();
                        });
                    }
                }
            }

            AuthStage::Totp => {
                let code = self.totp_field.content().to_string();
                if code.is_empty() {
                    self.error_text = "Enter TOTP code.".to_string();
                    return;
                }
                self.error_text.clear();

                let sender = self.connection.sender.clone();
                tokio::spawn(async move {
                    sender
                        .send(Message::TotpResponce(TotpResponceMessage {
                            totp_code: code,
                        }))
                        .await
                        .unwrap();
                });
            }

            AuthStage::Add2fa { .. } | AuthStage::LoggedIn | AuthStage::Done => {}
        }
    }

    fn send_add2fa_request(&mut self) {
        self.error_text.clear();
        self.info_text = "Requesting 2FA setup...".to_string();
        let sender = self.connection.sender.clone();
        tokio::spawn(async move {
            sender.send(Message::Add2faRequest).await.unwrap();
        });
    }

    fn send_remove2fa(&mut self) {
        self.error_text.clear();
        self.info_text = "Removing TOTP...".to_string();
        let sender = self.connection.sender.clone();
        tokio::spawn(async move {
            sender.send(Message::Remove2fa).await.unwrap();
        });
    }

    /// Poll messages from the server without blocking
    async fn poll_messages(&mut self) {
        loop {
            match self.connection.reciever.try_recv() {
                Ok(message) => self.handle_server_message(message),
                Err(mpsc::error::TryRecvError::Empty) => break,
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    self.error_text = "Connection lost.".to_string();
                    break;
                }
            }
        }
    }

    fn handle_server_message(&mut self, message: Message) {
        match message {
            Message::LogInSuccessful => {
                self.stage = AuthStage::LoggedIn;
                self.info_text = "Logged in!".to_string();
            }

            Message::SignUpSuccessful => {
                self.stage = AuthStage::Credentials;
                self.mode = AuthMode::Login;
                self.login_field.clear();
                self.password_field.clear();
                self.info_text = "Sign up successful! Please log in.".to_string();
            }

            Message::Ok => match &self.stage {
                AuthStage::LoggedIn => {
                    // TOTP removed successfully
                    self.info_text = "TOTP removed.".to_string();
                }
                AuthStage::Totp => {
                    self.stage = AuthStage::LoggedIn;
                }
                AuthStage::Add2fa { .. } | AuthStage::Credentials | AuthStage::Done => {}
            },

            Message::Error(error_message) => {
                use network_core::bytes_represented::error_message::ErrorMessage;
                self.error_text = match error_message {
                    ErrorMessage::FailToLogIn => "Failed to log in. Check credentials.",
                    ErrorMessage::FailToSignUp => "Failed to sign up. Name may be taken.",
                    ErrorMessage::OperationDenied => "Operation denied by server.",
                    ErrorMessage::UnexpectedMessage => "Server sent unexpected response.",
                }
                .to_string();
            }

            Message::TotpRequest => {
                self.stage = AuthStage::Totp;
                self.error_text.clear();
                self.info_text = "Enter your TOTP code from the authenticator app.".to_string();
                self.totp_field.focus();
            }

            Message::Add2faResponce(add2fa) => {
                self.stage = AuthStage::Add2fa {
                    secret: add2fa.secret.clone(),
                };
                self.error_text.clear();
                self.info_text = format!(
                    "Your 2FA secret: {}\nSave it in your authenticator app!",
                    add2fa.secret
                );
            }

            Message::VersionResponce(_)
            | Message::VersionRequest
            | Message::LogIn(_)
            | Message::SignUp(_)
            | Message::TotpResponce(_)
            | Message::Remove2fa
            | Message::Add2faRequest
            | Message::Build(_)
            | Message::Destroy(_)
            | Message::Grab(_)
            | Message::SetTriangle(_)
            | Message::PlayerState(_) => {
                // Ignore non-auth messages during auth phase
            }
        }
    }

    fn draw(&mut self, font: &Font) {
        self.window.clear(Color::rgb(230, 230, 240));

        // Title
        let title_text = match self.mode {
            AuthMode::Login => "Log In",
            AuthMode::SignUp => "Sign Up",
        };
        let mut title = Text::new(title_text, font, 28);
        title.set_fill_color(Color::rgb(30, 30, 50));
        title.set_position((190.0, 20.0));
        self.window.draw(&title);

        match &self.stage {
            AuthStage::Credentials => {
                // Draw input fields
                self.window.draw(&self.login_field);
                self.window.draw(&self.password_field);

                // Submit button with label
                let submit_label = match self.mode {
                    AuthMode::Login => "Log In",
                    AuthMode::SignUp => "Sign Up",
                };
                let submit = self.button_with_label(&self.submit_button, submit_label);
                self.window.draw(&submit);

                // Toggle mode button
                let toggle_label = match self.mode {
                    AuthMode::Login => "Sign Up instead",
                    AuthMode::SignUp => "Log In instead",
                };
                let toggle = self.button_with_label(&self.toggle_mode_button, toggle_label);
                self.window.draw(&toggle);
            }

            AuthStage::Totp => {
                let mut info = Text::new(&self.info_text, font, 14);
                info.set_fill_color(Color::rgb(50, 50, 80));
                info.set_position((90.0, 60.0));
                self.window.draw(&info);

                self.window.draw(&self.totp_field);

                let submit = self.button_with_label(&self.submit_totp_button, "Submit TOTP");
                self.window.draw(&submit);

                let back = self.button_with_label(&self.back_button, "Back");
                self.window.draw(&back);
            }

            AuthStage::Add2fa { .. } => {
                let mut info = Text::new(&self.info_text, font, 14);
                info.set_fill_color(Color::rgb(180, 40, 40));
                info.set_position((50.0, 80.0));
                self.window.draw(&info);

                let confirm = self.button_with_label(&self.confirm_add2fa_button, "I saved it");
                self.window.draw(&confirm);
            }

            AuthStage::LoggedIn => {
                let mut msg = Text::new("Logged in!", font, 22);
                msg.set_fill_color(Color::rgb(30, 120, 30));
                msg.set_position((190.0, 60.0));
                self.window.draw(&msg);

                if !self.info_text.is_empty() {
                    let mut info = Text::new(&self.info_text, font, 14);
                    info.set_fill_color(Color::rgb(50, 50, 80));
                    info.set_position((90.0, 100.0));
                    self.window.draw(&info);
                }

                let add_btn = self.button_with_label(&self.add_totp_button, "Add TOTP");
                self.window.draw(&add_btn);

                let remove_btn = self.button_with_label(&self.remove_totp_button, "Remove TOTP");
                self.window.draw(&remove_btn);

                let continue_btn = self.button_with_label(&self.continue_button, "Continue");
                self.window.draw(&continue_btn);
            }

            AuthStage::Done => {}
        }

        // Error text
        if !self.error_text.is_empty() {
            let mut error = Text::new(&self.error_text, font, 14);
            error.set_fill_color(Color::rgb(200, 40, 40));
            error.set_position((90.0, 370.0));
            self.window.draw(&error);
        }

        // Info text (non-error notifications)
        if !self.info_text.is_empty() && self.stage == AuthStage::Credentials {
            let mut info = Text::new(&self.info_text, font, 14);
            info.set_fill_color(Color::rgb(30, 120, 30));
            info.set_position((50.0, 370.0));
            self.window.draw(&info);
        }

        self.window.display();
    }

    /// Helper to create a Text-based button with a dynamic label.
    /// SFML `Button` has a fixed label, so we draw on top.
    fn button_with_label<'a>(&self, button: &'a Button, label: &'a str) -> ButtonWithLabel<'a> {
        ButtonWithLabel { button, label }
    }
}

/// A wrapper that draws a button with a dynamic label string
struct ButtonWithLabel<'a> {
    button: &'a Button,
    label: &'a str,
}

impl<'a> Drawable for ButtonWithLabel<'a> {
    fn draw<'b: 'shader, 'texture, 'shader, 'shader_texture>(
        &'b self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        use sfml::graphics::{RectangleShape, Shape};

        let (bx, by) = self.button.position();
        let (bw, bh) = self.button.size();

        // Draw background (NOT the original button — avoid label overlap)
        let mut bg = RectangleShape::new();
        bg.set_position((bx, by));
        bg.set_size((bw, bh));
        if self.button.hovered() {
            bg.set_fill_color(Color::rgb(90, 140, 220));
            bg.set_outline_color(Color::rgb(60, 100, 180));
        } else {
            bg.set_fill_color(Color::rgb(70, 120, 200));
            bg.set_outline_color(Color::rgb(50, 90, 160));
        }
        bg.set_outline_thickness(2.0);
        target.draw_with_renderstates(&bg, states);

        // Draw custom label centered on the button
        let font =
            Font::from_file("/usr/share/fonts/TTF/DejaVuSans.ttf").expect("Error to load font");
        let mut text = Text::new(self.label, &font, 18);
        text.set_fill_color(Color::WHITE);
        let bounds = text.local_bounds();
        text.set_position((
            bx + (bw - bounds.width) / 2.0,
            by + (bh - bounds.height) / 2.0 - 4.0,
        ));
        target.draw_with_renderstates(&text, states);
    }
}
