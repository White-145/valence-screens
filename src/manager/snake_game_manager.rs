use rand::Rng;
use valence::prelude::*;
use valence::text::color::RgbColor;

use crate::screen::pixel::Style;
use crate::screen::input::{PlayerAction, Uid, MoveDir};
use crate::screen::buffer::ScreenBuffer;
use crate::screen::game_manager::GameManager;

#[derive(Component)]
pub struct SnakeGameManager {
    width : u32,
    height : u32,
    snake : Vec<(i32, i32)>,
    apple: (i32, i32),
    direction : MoveDir,
    current_player : Option<Uid>,
    score : u32,
    high_score : u32,
    delay : u32,
}

impl Default for SnakeGameManager {
    fn default() -> Self {
        SnakeGameManager {
            width: 0,
            height: 0,
            snake: vec![],
            apple: (0, 0),
            direction : MoveDir::Up,
            current_player: None,
            score: 0,
            high_score: 0,
            delay: 0,
        }
    }
}

impl SnakeGameManager {
    fn gen_apple(&mut self) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(3..(self.width - 3)) as i32;
        let mut y = rng.gen_range(3..(self.height - 3)) as i32;
        while self.snake.contains(&(x, y)) {
            x = rng.gen_range(3..(self.width - 3)) as i32;
            y = rng.gen_range(3..(self.height - 3)) as i32;
        }
        self.apple = (x, y);
    }
}

impl GameManager for SnakeGameManager {
    fn init(&mut self, width: u32, height: u32, _has_fg: bool) {
        self.width = width;
        self.height = height;
        self.snake = vec![(4, 4)];
        self.direction = MoveDir::Up;
        self.current_player = None;
        self.score = 1;
        self.delay = 0;
        self.gen_apple();
    }

    fn draw(&self) -> ScreenBuffer {
        let mut buffer = ScreenBuffer::new(self.width, self.height);
        for (i, snake) in self.snake.iter().enumerate() {
            let color = 127 + (128.0 / self.snake.len() as f64 * (i + 1) as f64) as u8;
            buffer.put_bg(snake.0 as u32, snake.1 as u32, RgbColor::new(0, color, 0));
        }
        buffer.put_bg(self.apple.0 as u32, self.apple.1 as u32, RgbColor::new(255, 0, 0));
        let pointer_pos = self.direction.apply(self.snake.get(self.snake.len() - 1).unwrap(), 2);
        if pointer_pos.0 >= 0 && pointer_pos.1 >= 0 && self.current_player.is_some() {
            let pointer = match self.direction {
                MoveDir::Up => '⏶',
                MoveDir::Left => '⏴',
                MoveDir::Down => '⏷',
                MoveDir::Right => '⏵',
            };
            buffer.put_fg(pointer_pos.0 as u32, pointer_pos.1 as u32, pointer, RgbColor::new(128, 128, 128), Style::default());
        }
        let score_str = format!("SCORE:{}", self.score);
        for (i, char) in score_str.chars().enumerate() {
            buffer.put_fg(i as u32 + 1, 1, char, RgbColor::new(255, 255, 255), Style::default());
        }
        let high_score_str = format!("HIGH:{}", self.high_score);
        for (i, char) in high_score_str.chars().enumerate() {
            buffer.put_fg(self.width - high_score_str.len() as u32 - 1 + i as u32, 1, char, RgbColor::new(255, 255, 255), Style::default());
        }
        buffer
    }

    fn tick(&mut self, _time: f64) {
        if let None = self.current_player {
            return;
        }
        if self.delay >= 1 {
            self.delay = 0;
            return;
        } else {
            self.delay += 1;
        }
        let pos = self.snake.get(self.snake.len() - 1).unwrap();
        let next_pos = self.direction.apply(pos, 1);
        if self.apple == next_pos {
            self.gen_apple();
            self.score += 1;
        } else {
            self.snake.remove(0);
        }
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= self.width as i32 || next_pos.1 >= self.height as i32 || self.snake.contains(&next_pos) {
            if self.high_score < self.score {
                self.high_score = self.score;
            }
            self.init(self.width, self.height, false);
        } else {
            self.snake.push(next_pos);
        }
    }

    fn action(&mut self, player: Uid, action: PlayerAction) {
        if let PlayerAction::SpecialMove { direction, is_sneaking : _is_sneaking } = action {
            if let Some(current_player) = self.current_player {
                if player == current_player && direction != self.direction.opposite() {
                    self.direction = direction;
                }
            } else {
                self.current_player = Some(player);
                self.direction = direction;
            }
        } else if let PlayerAction::Disconnect = action {
            if let Some(current_player) = self.current_player {
                if current_player == player {
                    self.init(self.width, self.height, false);
                }
            }
        }
    }
}