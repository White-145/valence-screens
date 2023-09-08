pub mod matrix_generator;
pub mod rainbow_generator;

use valence::prelude::*;

use crate::screen::buffer::ScreenBuffer;
use crate::screen::game_manager::GameManager;
use crate::screen::pixel::ScreenPixel;
use crate::screen::input::{Uid, PlayerAction, MoveDir};

// Older version of screen manager that doesnt respond to inputs and draws entire buffer instead of individual pixels
pub trait Generator: Sync + Send + 'static {
    fn init(&mut self, width: u32, height: u32, has_fg: bool);

    fn tick(&mut self, time: f64);

    fn draw(&self) -> ScreenBuffer;
}

// Game manager that uses generator
#[derive(Component)]
pub struct StaticGameManager<T: Generator> {
    width : u32,
    height : u32,
    buffer : ScreenBuffer,
    generator : T,
}

impl<T: Generator> StaticGameManager<T> {
    pub fn new(generator: T) -> Self {
        StaticGameManager { width: 0, height: 0, buffer: ScreenBuffer::default(), generator: generator }
    }
}

impl<T: Generator> GameManager for StaticGameManager<T> {
    fn init(&mut self, width: u32, height: u32, has_fg: bool) {
        self.width = width;
        self.height = height;
        self.generator.init(width, height, has_fg);
        self.generator.tick(0.0);
        self.buffer = self.generator.draw();
    }

    fn draw(&self, x: u32, y: u32) -> ScreenPixel {
        self.buffer.get(x, y).unwrap_or_default()
    }

    fn tick(&mut self, time: f64) {
        self.generator.tick(time);
        self.buffer = self.generator.draw();
    }

    fn action(&mut self, player: Uid, action: PlayerAction, is_sneaking: bool) {
        // debug
        let action_str = match action {
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
            PlayerAction::Move(direction) => format!("Move to {}", match direction {
                MoveDir::Left => "Left".to_owned(),
                MoveDir::Up => "Up".to_owned(),
                MoveDir::Down => "Down".to_owned(),
                MoveDir::Right => "Right".to_owned(),
            }),
            PlayerAction::SpecialMove(direction) => format!("Special Move to {}", match direction {
                MoveDir::Left => "Left".to_owned(),
                MoveDir::Up => "Up".to_owned(),
                MoveDir::Down => "Down".to_owned(),
                MoveDir::Right => "Right".to_owned(),
            }),
            PlayerAction::Input(input) => format!("Input \"{}\"", input),
            PlayerAction::Hover(position) => {
                match position {
                    None => "Hover".to_owned(),
                    Some(position) => format!("Hover at {} {}", position.0, position.1)
                }
            },
            PlayerAction::Free => "Freed".to_owned(),
        };
        println!("action {action_str} with player {player} and sneaking {is_sneaking}");
    }
}
