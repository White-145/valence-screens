use colors_transform::{Color, Hsl};
use noise::{NoiseFn, Perlin, Simplex};
use valence::prelude::*;
use valence::text::color::RgbColor;
use valence_screens::buffer::ScreenBuffer;
use valence_screens::game_manager::GameManager;
use valence_screens::input::{PlayerAction, Uid};
use valence_screens::pixel::{ScreenPixel, Style};

const DEPTH: [char; 5] = ['*', '%', '#', '$', '@'];

#[derive(Component, Default)]
pub struct RainbowGameManager {
    width: u32,
    height: u32,
    has_fg: bool,
    saturation_noise: Simplex,
    lightness_noise: Perlin,
    time: f64,
}

impl GameManager for RainbowGameManager {
    fn init(&mut self, width: u32, height: u32, has_fg: bool) {
        self.width = width;
        self.height = height;
        self.has_fg = has_fg;
        self.time = 0.0;
    }

    fn draw(&self) -> ScreenBuffer {
        let ratio = self.width as f64 / self.height as f64;
        ScreenBuffer::construct(self.width, self.height, |x, y| {
            let x_norm = x as f64 / self.width as f64 * ratio;
            let y_norm = y as f64 / self.height as f64;
            let mut pixel = ScreenPixel::default();

            let saturation = self
                .saturation_noise
                .get([x_norm, y_norm, self.time / 25.0]);
            let lightness =
                self.lightness_noise
                    .get([x_norm * 100.0, y_norm * 100.0, self.time * 5.0]);

            let color = Hsl::from(
                ((x_norm + y_norm + self.time * 2.0) as f32 * 180.0) % 360.0,
                50.0 + 50.0 * saturation as f32,
                75.0 + 2.0 * lightness as f32,
            );

            pixel.bg = RgbColor::new(
                color.get_red() as u8,
                color.get_green() as u8,
                color.get_blue() as u8,
            );

            if self.has_fg {
                let mut depth =
                    ((self
                        .lightness_noise
                        .get([x_norm * 2.5, y_norm * 2.5, self.time * 2.0])
                        / 2.0
                        + 0.5)
                        * DEPTH.len() as f64) as usize;
                if depth == DEPTH.len() {
                    depth -= 1;
                }

                pixel.fg_char = DEPTH[depth];
                pixel.fg_color = RgbColor::new(255, 255, 255);
                pixel.fg_style = Style::default();
            }

            pixel
        })
    }

    fn tick(&mut self) {
        self.time += 0.05;
    }

    fn action(&mut self, _player: Uid, _action: PlayerAction) {}
}
