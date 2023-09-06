pub mod default_game_manager;
pub mod rainbow_generator;
pub mod matrix_generator;

use crate::screen::ScreenPixel;

pub enum MoveDir {
    Up,
    Left,
    Down,
    Right,
}

pub enum PlayerAction {
    // Right Click ( Position On Screen ) doesnt include position
    Primary(Option<(u32, u32)>),
    // Left Click ( Position On Screen ) doesnt include position
    Secondary(Option<(u32, u32)>),
    // Swap Hands
    Swap,
    // Drop doesnt work
    Drop,
    // Player Movement ( Direction ) not implemented
    Move(MoveDir),
    // Slot Movement ( Direction ) doesnt work
    SpecialMove(MoveDir),
    // Text Input not implemented
    Input(String),
}

impl MoveDir {
    fn into_string(self) -> String {
        match self {
            MoveDir::Left => "Left".to_owned(),
            MoveDir::Up => "Up".to_owned(),
            MoveDir::Down => "Down".to_owned(),
            MoveDir::Right => "Right".to_owned(),
        }
    }
}

impl PlayerAction {
    fn into_string(self) -> String {
        match self {
            PlayerAction::Primary(position) => {
                match position {
                    None => "Primary".to_owned(),
                    Some(position) => format!("Primary at {} {}", position.0, position.1)
                }
            },
            PlayerAction::Secondary(position) => {
                match position {
                    None => "Secondary".to_owned(),
                    Some(position) => format!("Secondary at {} {}", position.0, position.1)
                }
            },
            PlayerAction::Swap => "Swap".to_owned(),
            PlayerAction::Drop => "Drop".to_owned(),
            PlayerAction::Move(direction) => format!("Move to {}", direction.into_string()),
            PlayerAction::SpecialMove(direction) => format!("Special Move to {}", direction.into_string()),
            PlayerAction::Input(input) => format!("Input \"{}\"", input),
        }
    }
}

// Thing responsible for drawing screen and responding to inputs
#[bevy_trait_query::queryable]
pub trait GameManager: Send + Sync + 'static {
    // init game manager with size of the screen. Called only once in screen spawn function
    fn init(&mut self, width: u32, height: u32);

    // draw screen pixel at given position. x, y is always 0..width, 0..height
    fn draw(&self, x: u32, y: u32) -> ScreenPixel;

    // called every tick with time server been running for
    fn tick(&mut self, time: f64);

    // player action with player uid, action and whether player is sneaking (for extra functionality)
    fn action(&mut self, player: u8, action: PlayerAction, is_sneaking: bool);
}
