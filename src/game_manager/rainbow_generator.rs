use colors_transform::{Hsl, Color};
use noise::{NoiseFn, Simplex, Perlin};
use valence::text::color::RgbColor;
use valence::prelude::*;
use crate::buffer::ScreenBuffer;
use crate::game_manager::default_game_manager::Generator;
use crate::screen::*;

use super::default_game_manager::StaticGameManager;

const DEPTH: [char; 5] = ['*', '%', '#', '$', '@'];

#[derive(Component, Default)]
pub struct RainbowGenerator {
    width : u32,
    height : u32,
    saturation_noise : Simplex,
    lightness_noise : Perlin,
    time : f64
}

impl RainbowGenerator {
    pub fn default_manager() -> StaticGameManager<RainbowGenerator> {
        StaticGameManager::new(RainbowGenerator::default())
    }
}

impl Generator for RainbowGenerator {
    fn init(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn tick(&mut self, time: f64) {
        self.time = time;
    }

    fn draw(&self) -> ScreenBuffer {
        let mut buffer = ScreenBuffer::new(self.width, self.height);
        let ratio = self.width as f64 / self.height as f64;
        for x in 0..self.width {
            for y in 0..self.height {
                let x_norm = x as f64 / self.width as f64 * ratio;
                let y_norm = y as f64 / self.height as f64;

                let saturation = self.saturation_noise.get([
                    x_norm,
                    y_norm,
                    self.time / 25.0
                ]);
                let lightness = self.lightness_noise.get([
                    x_norm * 100.0,
                    y_norm * 100.0,
                    self.time * 5.0
                ]);

                let mut depth = ((self.lightness_noise.get([
                    x_norm * 2.5,
                    y_norm * 2.5,
                    self.time * 2.0
                ]) / 2.0 + 0.5) * DEPTH.len() as f64) as usize;
                if depth == DEPTH.len() { depth -= 1; }
                
                let color = Hsl::from(
                    ((x_norm + y_norm + self.time * 2.0) as f32 * 180.0) % 360.0,
                    50.0 + 50.0 * saturation as f32,
                    75.0 + 2.0 * lightness as f32
                );
                buffer.put(x, y, ScreenPixel::new(
                    RgbColor::new(
                        color.get_red() as u8,
                        color.get_green() as u8,
                        color.get_blue() as u8
                    ),
                    DEPTH[depth],
                    RgbColor::new(255, 255, 255),
                    Style::default()
                ))
            }
        }
        buffer
    }
}