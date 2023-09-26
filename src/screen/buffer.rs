use image::{GenericImageView, DynamicImage};
use palette::color_difference::Ciede2000;
use palette::convert::FromColorUnclamped;
use palette::{Srgb, Lab};
use valence::prelude::*;
use valence::text::color::RgbColor;
use game_manager::GameManager;
use input::PlayerAction;
use crate::screen::pixel::Style;

use super::*;
use super::pixel::ScreenPixel;

// Useful struct to manage screen pixels
#[derive(Component, Clone)]
pub struct ScreenBuffer {
    width : u32,
    height : u32,
    // couldnt do [ScreenPixel; width * height] or smth
    buffer : Vec<ScreenPixel>,
}

impl Default for ScreenBuffer {
    fn default() -> Self {
        ScreenBuffer { width : 0, height : 0, buffer: vec![] }
    }
}

impl ScreenBuffer {
    // create a new buffer of default screen pixels with given size
    pub fn new(width: u32, height: u32) -> Self {
        ScreenBuffer { width, height, buffer: vec![ScreenPixel::default(); (width * height) as usize] }
    }

    // inner function to get single number index from position
    fn get_index(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((x * self.height + y) as usize)
        }
    }

    // gets screen pixel at given position, None if position is out of bounds
    pub fn get(&self, x: u32, y: u32) -> Option<ScreenPixel> {
        let index = self.get_index(x, y)?;
        Some(self.buffer[index].clone())
    }

    // puts screen pixel to given position
    pub fn put(&mut self, x: u32, y: u32, pixel: ScreenPixel) {
        let index = self.get_index(x, y);
        if let Some(i) = index {
            self.buffer[i] = pixel;
        }
    }

    // puts screen pixel foreground to given position, keeps background
    pub fn put_fg(&mut self, x: u32, y: u32, fg_char: char, fg_color: RgbColor, fg_style: Style) {
        let index = self.get_index(x, y);
        if let Some(i) = index {
            self.buffer[i].fg = (fg_char, fg_color, fg_style);
        }
    }

    // puts screen pixel background to given position, keeps foreground
    pub fn put_bg(&mut self, x: u32, y: u32, bg: RgbColor) {
        let index = self.get_index(x, y);
        if let Some(i) = index {
            self.buffer[i].bg = bg;
        }
    }

    // fills buffer with given pixel
    pub fn fill(&mut self, pixel: ScreenPixel) {
        for i in 0..(self.width * self.height) as usize {
            self.buffer[i] = pixel.clone();
        }
    }

    // draws buffer from function
    pub fn construct<F>(width: u32, height: u32, mut generator: F) -> ScreenBuffer
    where
        F: FnMut(u32, u32) -> ScreenPixel,
    {
        let mut buffer = ScreenBuffer::new(width, height);
        for x in 0..width {
            for y in 0..height {
                buffer.put(x, y, generator(x, y));
            }
        }
        buffer
    }
}

// Buffer could be used as game manager
impl GameManager for ScreenBuffer {
    // dont really care about init
    fn init(&mut self, _width: u32, _height: u32, _has_fg: bool) { }

    fn draw(&self) -> ScreenBuffer {
        self.clone()
    }

    fn tick(&mut self, _time: f64) { }

    fn action(&mut self, _player: u8, _action: PlayerAction) { }
}

impl ScreenBuffer {
    // some *simple* function i made to load an image to the buffer.
    // You can display it using result of this as a game manager
    pub fn load_image(filename: &str, width: u32, height: u32, use_fg: bool) -> ScreenBuffer {
        const INTENSITY: [char; 7] = ['*', '+', '=', '%', '$', '#', '@'];
        
        fn get_dominant_color(part: &DynamicImage) -> RgbColor {
            let dominant = dominant_color::get_colors(part.to_rgb8().into_raw().as_slice(), false);
            RgbColor::new(dominant[0], dominant[1], dominant[2])
        }
        
        fn get_mean_color(part: &DynamicImage) -> RgbColor {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            for (_x, _y, pixel) in part.pixels() {
                r += pixel.0[0] as u32;
                g += pixel.0[1] as u32;
                b += pixel.0[2] as u32;
            }
            let i = part.width() * part.height();
            RgbColor::new((r / i) as u8, (g / i) as u8, (b / i) as u8)
        }
        
        fn get_diff(color1: &RgbColor, color2: &RgbColor) -> f32 {
            let srgb_color1 = Srgb::new(color1.r as f32 / 255.0, color1.g as f32 / 255.0, color1.b as f32 / 255.0);
            let srgb_color2 = Srgb::new(color2.r as f32 / 255.0, color2.g as f32 / 255.0, color2.b as f32 / 255.0);
            let lab_color1: Lab = Lab::from_color_unclamped(srgb_color1);
            let lab_color2: Lab = Lab::from_color_unclamped(srgb_color2);
            // 57.120705 is maximum possible difference between mean and dominant of the same image
            // found using diff(grayscale(255), grayscale((255 + 255 + 0) / 3))
            // not sure tho but it didnt fail yet
            let diff = lab_color1.difference(lab_color2) / 57.120705;
            diff
        }
        
        let mut img = image::open(filename).unwrap();
        if !use_fg || img.width() < width || img.height() < height {
            img = img.resize(width, height, image::imageops::FilterType::CatmullRom);
        }

        let mut buffer = ScreenBuffer::new(width, height);
        if !use_fg {
            for (x, y, pixel) in img.pixels() {
                buffer.put_bg(x, y, RgbColor::new(pixel.0[0], pixel.0[1], pixel.0[2]));
            }
        } else {
            let hor_ratio = img.width() as f64 / width as f64;
            let ver_ratio = img.height() as f64 / height as f64;
            for x in 0..width {
                for y in 0..height {
                    let part = img.crop(
                        (x as f64 * hor_ratio) as u32,
                        (y as f64 * ver_ratio) as u32,
                        hor_ratio as u32,
                        ver_ratio as u32
                    );
                    let mean = get_mean_color(&part);
                    let dominant = get_dominant_color(&part);
                    let difference = get_diff(&dominant, &mean);
                    let intensity = INTENSITY[(difference * INTENSITY.len() as f32) as usize];
                    buffer.put(x, y, ScreenPixel::new(mean, intensity, dominant, Style::default()));
                }
            }
        }
        buffer
    }
}
