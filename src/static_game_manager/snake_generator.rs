use rand::Rng;
use valence::prelude::*;
use valence::text::color::RgbColor;

use crate::screen::pixel::Style;
use crate::static_game_manager::Generator;
use crate::screen::input::{PlayerAction, Uid, MoveDir};
use crate::screen::buffer::ScreenBuffer;

use super::StaticGameManager;

#[derive(Component)]
pub struct SnakeGenerator {
    width : u32,
    height : u32,
    snake : Vec<(i32, i32)>,
    apple: (i32, i32),
    direction : MoveDir,
    player : Option<Uid>,
    length : u32,
    delay : u32,
}

impl Default for SnakeGenerator {
    fn default() -> Self {
        SnakeGenerator {
            width : 0,
            height : 0,
            snake : vec![],
            apple : (0, 0),
            direction : MoveDir::Up,
            player : None,
            length : 0,
            delay : 0,
        }
    }
}

impl SnakeGenerator {
    fn gen_apple(&mut self) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..self.width) as i32;
        let mut y = rng.gen_range(0..self.height) as i32;
        while self.snake.contains(&(x, y)) {
            x = rng.gen_range(0..self.width) as i32;
            y = rng.gen_range(0..self.height) as i32;
        }
        self.apple = (x, y);
    }

    pub fn default_manager() -> StaticGameManager<SnakeGenerator> {
        StaticGameManager::new(SnakeGenerator::default())
    }
}

impl Generator for SnakeGenerator {
    fn init(&mut self, width: u32, height: u32, _has_fg: bool) {
        self.width = width;
        self.height = height;
        self.snake = vec![(1, 1)];
        self.direction = MoveDir::Up;
        self.player = None;
        self.length = 1;
        self.delay = 0;
        self.gen_apple();
    }

    fn tick(&mut self, _time: f64) {
        if let None = self.player {
            return;
        }
        if self.delay >= 1 {
            self.delay = 0;
            return;
        } else {
            self.delay += 1;
        }
        let pos = self.snake.get(self.snake.len() - 1).unwrap();
        let mut next_pos = pos.clone();
        match self.direction {
            MoveDir::Up => next_pos.1 -= 1,
            MoveDir::Left => next_pos.0 -= 1,
            MoveDir::Down => next_pos.1 += 1,
            MoveDir::Right => next_pos.0 += 1,
        }
        if self.apple == next_pos {
            self.gen_apple();
            self.length += 1;
        } else {
            self.snake.remove(0);
        }
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= self.width as i32 || next_pos.1 >= self.height as i32 || self.snake.contains(&next_pos) {
            self.init(self.width, self.height, false);
            return;
        }
        self.snake.push(next_pos);
    }

    fn draw(&self) -> ScreenBuffer {
        let mut buffer = ScreenBuffer::new(self.width, self.height);
        for snake in self.snake.iter() {
            buffer.put_bg(snake.0 as u32, snake.1 as u32, RgbColor::new(0, 255, 0));
        }
        buffer.put_bg(self.apple.0 as u32, self.apple.1 as u32, RgbColor::new(255, 0, 0));
        for (i, char) in self.length.to_string().chars().enumerate() {
            buffer.put_fg(i as u32, 0, char, RgbColor::new(255, 255, 255), Style::default());
        }
        buffer
    }

    fn action(&mut self, player: Uid, action: PlayerAction) {
        if let PlayerAction::SpecialMove { direction, is_sneaking : _is_sneaking } = action {
            if let None = self.player {
                self.player = Some(player);
                self.direction = direction;
            } else if player == self.player.unwrap() && direction != self.direction.opposite() {
                self.direction = direction;
            }
        }
    }
}