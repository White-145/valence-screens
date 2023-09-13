use colors_transform::{Hsl, Color};
use noise::{NoiseFn, Simplex, Perlin};
use valence::text::color::RgbColor;
use valence::prelude::*;
use crate::screen::buffer::ScreenBuffer;
use crate::screen::input::{Uid, PlayerAction};
use crate::screen::pixel::{ScreenPixel, Style};

use super::{StaticGameManager, Generator};

const DEPTH: [char; 5] = ['*', '%', '#', '$', '@'];

#[derive(Component, Default)]
pub struct RainbowGenerator {
    width : u32,
    height : u32,
    has_fg : bool,
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
    fn init(&mut self, width: u32, height: u32, has_fg: bool) {
        self.width = width;
        self.height = height;
        self.has_fg = has_fg;
    }

    fn tick(&mut self, time: f64) {
        self.time = time;
    }

    fn draw(&self) -> ScreenBuffer {
        let ratio = self.width as f64 / self.height as f64;
        ScreenBuffer::construct(self.width, self.height, |x, y| {
            let x_norm = x as f64 / self.width as f64 * ratio;
            let y_norm = y as f64 / self.height as f64;
            let mut pixel = ScreenPixel::default();

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
            
            let color = Hsl::from(
                ((x_norm + y_norm + self.time * 2.0) as f32 * 180.0) % 360.0,
                50.0 + 50.0 * saturation as f32,
                75.0 + 2.0 * lightness as f32
            );

            pixel.bg = RgbColor::new(
                color.get_red() as u8,
                color.get_green() as u8,
                color.get_blue() as u8
            );

            if self.has_fg {
                let mut depth = ((self.lightness_noise.get([
                    x_norm * 2.5,
                    y_norm * 2.5,
                    self.time * 2.0
                ]) / 2.0 + 0.5) * DEPTH.len() as f64) as usize;
                if depth == DEPTH.len() { depth -= 1; }

                pixel.fg = (
                    DEPTH[depth],
                    RgbColor::new(255, 255, 255),
                    Style::default()
                )
            }
            
            pixel
        })
    }

    fn action(&mut self, _player: Uid, _action: PlayerAction) { }
}