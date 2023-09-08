use rand::Rng;
use valence::prelude::*;
use valence::text::color::RgbColor;
use crate::screen::buffer::ScreenBuffer;
use crate::static_game_manager::Generator;
use crate::screen::pixel::{ScreenPixel, Style};

use super::StaticGameManager;

const DEFAULT_CHARSET: [char; 12] = ['佘', '佡', '佴', '価', '俪', '倾', '偀', '偧', '偰', '傟', '僀', '價'];

#[derive(Component)]
pub struct MatrixGenerator {
    width : u32,
    height : u32,
    trails : Vec<(i32, i32)>,
    charset : Vec<char>,
    trail_len : i32,
}

impl MatrixGenerator {
    pub fn default_manager() -> StaticGameManager<MatrixGenerator> {
        StaticGameManager::new(MatrixGenerator::default())
    }

    pub fn new(trail_len: i32) -> MatrixGenerator {
        let mut matrix = MatrixGenerator::default();
        matrix.trail_len = trail_len;
        matrix
    }

    pub fn new_charset(charset: Vec<char>, trail_len: i32) -> MatrixGenerator {
        let mut matrix = MatrixGenerator::default();
        matrix.charset = charset;
        matrix.trail_len = trail_len;
        matrix
    }
}

impl Default for MatrixGenerator {
    fn default() -> Self {
        Self { width: 0, height: 0, trails: vec![], charset: DEFAULT_CHARSET.to_vec(), trail_len: 7 }
    }
}

impl Generator for MatrixGenerator {
    fn init(&mut self, width: u32, height: u32, _has_fg: bool) {
        self.width = width;
        self.height = height;
    }

    fn tick(&mut self, _time: f64) {
        let mut new_trails: Vec<(i32, i32)> = Vec::new();
        for trail in &self.trails {
            if trail.1 - 1 <= self.height as i32 + self.trail_len {
                new_trails.push((trail.0, trail.1 + 1));
            }
        }
        let mut random = rand::thread_rng();
        if new_trails.len() < (self.height as f64 * 2.5) as usize {
            for _i in 0..random.gen_range(1..3) {
                new_trails.push((random.gen_range(0..(self.width as i32 - 1)), 0));
            }
        }
        self.trails = new_trails;
    }

    fn draw(&self) -> ScreenBuffer {
        let mut random = rand::thread_rng();
        let mut buffer = ScreenBuffer::new(self.width, self.height);
        for trail in &self.trails {
            for i in 0..self.trail_len {
                if trail.1 - i > 0 {
                    buffer.put(trail.0 as u32, trail.1 as u32 - i as u32, ScreenPixel::new(
                        RgbColor::new(0, 0, 0),
                        self.charset
                            .get(random.gen_range(0..self.charset.len()))
                            .unwrap_or(&' ')
                            .clone(),
                        RgbColor::new(0, (255 / self.trail_len * (self.trail_len - i)) as u8, 0),
                        Style::new(
                            random.gen_bool(0.5),
                            random.gen_bool(0.5),
                            random.gen_bool(0.5),
                            random.gen_bool(0.5)
                        )
                    ));
                }
            }
        }
        buffer
    }
}
