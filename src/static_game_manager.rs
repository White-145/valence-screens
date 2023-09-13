pub mod matrix_generator;
pub mod rainbow_generator;
pub mod snake_generator;

use valence::prelude::*;

use crate::screen::buffer::ScreenBuffer;
use crate::screen::game_manager::GameManager;
use crate::screen::pixel::ScreenPixel;
use crate::screen::input::{Uid, PlayerAction};

// Screen manager that draws entire buffer instead of individual pixels
pub trait Generator: Sync + Send + 'static {
    fn init(&mut self, width: u32, height: u32, has_fg: bool);

    fn tick(&mut self, time: f64);

    fn draw(&self) -> ScreenBuffer;

    fn action(&mut self, player: Uid, action: PlayerAction);
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

    fn action(&mut self, player: Uid, action: PlayerAction) {
        self.generator.action(player, action);
    }
}
