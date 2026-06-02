use std::{collections::HashMap, sync::Arc};

use core_3c::{board::Board, kit::Kit, player_state::PlayerState, vector::Vector};
use network_core::message::Message;
use sfml::{
    cpp::FBox,
    graphics::{Color, RenderStates, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style, VideoMode, mouse::Button},
};
use tokio::sync::{Mutex, mpsc};

use crate::{
    actions_menu::{Action, ActionsMenu},
    auth_window::AuthWindow,
    board_box::BoardBox,
    players_states_box::PlayersStatesBox,
    texture_pack::TexturePack,
};

mod actions_menu;
mod auth_window;
mod board_box;
mod button;
mod input_field;
mod players_states_box;
mod texture_pack;

#[tokio::main]
async fn main() {
    let auth_window = AuthWindow::new().await;
    let (connection, player_name) = match auth_window.run().await {
        Some(c) => c,
        None => return,
    };

    let (mut window, board_mutex, players_states_mutex, texture_pack) = init().await;

    let players_states_box = PlayersStatesBox::new(players_states_mutex.clone());
    let board_box = BoardBox::new(board_mutex.clone(), texture_pack);
    let mut actions_menu = ActionsMenu::new(player_name.clone());

    tokio::spawn(handle_message_loop(
        board_mutex.clone(),
        players_states_mutex.clone(),
        connection.reciever,
        connection.sender.clone(),
    ));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            handler_sfml_event(
                event,
                &mut window,
                board_mutex.clone(),
                players_states_mutex.clone(),
                &mut actions_menu,
                connection.sender.clone(),
                &player_name,
            )
            .await;
        }

        draw(&mut window, &players_states_box, &board_box, &actions_menu);
    }
}

async fn init() -> (
    FBox<RenderWindow>,
    Arc<Mutex<Board>>,
    Arc<Mutex<HashMap<String, PlayerState>>>,
    TexturePack,
) {
    let window = RenderWindow::new(
        VideoMode::new(1920, 1080, 32),
        "three corners",
        Style::CLOSE,
        &ContextSettings::default(),
    )
    .unwrap();

    let kit = Kit::from_files(String::from("core_3c/data/")).expect("Error to load game kit");

    let texture_pack = TexturePack::from_kit(&kit);

    let board = Arc::new(Mutex::new(Board::new(Vector { x: 11, y: 10 }, kit)));
    let players_states = Arc::new(Mutex::new(HashMap::new()));

    (window, board, players_states, texture_pack)
}

fn draw(
    window: &mut RenderWindow,
    players_states_box: &PlayersStatesBox,
    board_box: &BoardBox,
    actions_menu: &ActionsMenu,
) {
    window.clear(Color::rgb(255, 127, 127));

    let players_states_box_render_states = RenderStates::DEFAULT;
    window.draw_with_renderstates(players_states_box, &players_states_box_render_states);

    let mut board_box_render_states = RenderStates::DEFAULT;
    board_box_render_states.transform.translate(400.0, 300.0);
    window.draw_with_renderstates(board_box, &board_box_render_states);

    let mut actions_menu_render_states = RenderStates::DEFAULT;
    actions_menu_render_states
        .transform
        .translate((3 * window.size().x / 4) as f32, 0.0);
    window.draw_with_renderstates(actions_menu, &actions_menu_render_states);

    window.display();
}

async fn handler_sfml_event(
    event: Event,
    window: &mut RenderWindow,
    board_mutex: Arc<Mutex<Board>>,
    players_states: Arc<Mutex<HashMap<String, PlayerState>>>,
    actions_menu: &mut ActionsMenu,
    sender: mpsc::Sender<Message>,
    player_name: &str,
) {
    match event {
        Event::Closed => window.close(),
        Event::Resized { width, height } => (),
        Event::LostFocus => (),
        Event::GainedFocus => (),
        Event::TextEntered { unicode } => (),
        Event::KeyPressed {
            code,
            scan,
            alt,
            ctrl,
            shift,
            system,
        } => (),
        Event::KeyReleased {
            code,
            scan,
            alt,
            ctrl,
            shift,
            system,
        } => (),
        Event::MouseWheelScrolled { wheel, delta, x, y } => (),
        Event::MouseButtonPressed { button, x, y } => {
            if x > (3 * window.size().x / 4) as i32 {
                actions_menu
                    .handle_button_pressing(x, y, sender.clone())
                    .await;
            }
            if (x < 400) || (y < 284) || (x > 576) || (y > 594) {
                ()
            } else {
                let x = x - 400;
                let y = y - 284;

                let x = if x % 16 <= 7 {
                    (x / 16) as u32
                } else {
                    (x / 16 + 1) as u32
                };
                let y = (y / 31) as u32;

                let location = Vector { x, y };

                if button == Button::Left {
                    actions_menu.clear();
                    actions_menu.set_location(location);

                    let board = board_mutex.lock().await;
                    if let Ok(triangle) = board.triangle(location) {
                        match triangle {
                            Some(building) => {
                                let is_own = building.player == player_name;
                                actions_menu.set_owner(Some(building.player.clone()));

                                actions_menu.add(Action::Destroy(
                                    board
                                        .kit()
                                        .building_kit()
                                        .get(&building.name)
                                        .expect("fail to found building in kit")
                                        .base_destroy_price,
                                ));
                                if !is_own {
                                    actions_menu.add(Action::Grab(
                                        board
                                            .kit()
                                            .building_kit()
                                            .get(&building.name)
                                            .expect("fail to found building in kit")
                                            .base_grab_price,
                                    ));
                                }
                            }
                            None => board.kit().building_kit().iter().for_each(|building| {
                                actions_menu.add(Action::Build(
                                    building.0.clone(),
                                    building.1.base_build_price,
                                ))
                            }),
                        }
                    }
                }
            }
        }
        Event::MouseButtonReleased { button, x, y } => (),
        Event::MouseMoved { x, y } => (),
        Event::MouseEntered => (),
        Event::MouseLeft => (),
        Event::JoystickButtonPressed { joystickid, button } => (),
        Event::JoystickButtonReleased { joystickid, button } => (),
        Event::JoystickMoved {
            joystickid,
            axis,
            position,
        } => (),
        Event::JoystickConnected { joystickid } => (),
        Event::JoystickDisconnected { joystickid } => (),
        Event::TouchBegan { finger, x, y } => (),
        Event::TouchMoved { finger, x, y } => (),
        Event::TouchEnded { finger, x, y } => (),
        Event::SensorChanged { type_, x, y, z } => (),
        _ => (),
    }
}

async fn handle_message_loop(
    board_mutex: Arc<Mutex<Board>>,
    players_states_mutex: Arc<Mutex<HashMap<String, PlayerState>>>,
    mut reciever: mpsc::Receiver<Message>,
    sender: mpsc::Sender<Message>,
) {
    loop {
        match reciever.recv().await {
            Some(message) => {
                handler_message(
                    message,
                    board_mutex.clone(),
                    players_states_mutex.clone(),
                    sender.clone(),
                )
                .await;
            }
            None => break,
        }
    }
}

async fn handler_message(
    message: Message,
    board_mutex: Arc<Mutex<Board>>,
    players_states_mutex: Arc<Mutex<HashMap<String, PlayerState>>>,
    sender: mpsc::Sender<Message>,
) {
    match message {
        Message::Ok => (),
        Message::Error(error_message) => println!("{:?}", error_message),
        Message::VersionRequest => todo!(),
        Message::VersionResponce(version_responce_message) => (),
        Message::LogIn(_) => (),
        Message::SignUp(_) => (),
        Message::LogInSuccessful => (),
        Message::SignUpSuccessful => (),
        Message::TotpRequest => (),
        Message::TotpResponce(_) => (),
        Message::Add2faRequest => (),
        Message::Add2faResponce(_) => (),
        Message::Remove2fa => (),
        Message::Build(_) => (),
        Message::Destroy(_) => (),
        Message::Grab(_) => (),
        Message::SetTriangle(set_triangle_message) => {
            board_mutex
                .lock()
                .await
                .set_triangle(set_triangle_message.triangle, set_triangle_message.location)
                .unwrap();
        }
        Message::PlayerState(player_state_message) => {
            players_states_mutex
                .lock()
                .await
                .insert(player_state_message.player, player_state_message.state);
        }
    }
}
