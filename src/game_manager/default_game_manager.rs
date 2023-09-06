use valence::prelude::*;

use crate::buffer::ScreenBuffer;
use crate::game_manager::*;
use crate::screen::ScreenPixel;

// Older version of screen manager that doesnt respond to inputs and draws entire buffer instead of individual pixels
pub trait Generator: Sync + Send + 'static {
    fn init(&mut self, width: u32, height: u32);

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
    fn init(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.generator.init(width, height);
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

    fn action(&mut self, player: u8, action: PlayerAction, is_sneaking: bool) {
        // debug
        println!("action {} with player {player} and sneaking {is_sneaking}", action.into_string());
    }
}
