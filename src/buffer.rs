use game_manager::GameManager;
use valence::prelude::*;
use valence::text::color::RgbColor;

#[cfg(feature = "input")]
use crate::input::PlayerAction;
use crate::pixel::Style;

use super::pixel::ScreenPixel;
use super::*;

// Useful struct to manage screen pixels
#[derive(Component, Clone, Default, Debug)]
pub struct ScreenBuffer {
    width: u32,
    height: u32,
    // couldnt do [ScreenPixel; width * height] or smth
    buffer: Vec<ScreenPixel>,
}

impl ScreenBuffer {
    // create a new buffer of default screen pixels with given size
    pub fn new(width: u32, height: u32) -> Self {
        ScreenBuffer {
            width,
            height,
            buffer: vec![ScreenPixel::default(); (width * height) as usize],
        }
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
        Some(self.buffer[index])
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
            self.buffer[i].fg_char = fg_char;
            self.buffer[i].fg_color = fg_color;
            self.buffer[i].fg_style = fg_style;
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
            self.buffer[i] = pixel;
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

    pub fn insert(&mut self, x: u32, y: u32, other: ScreenBuffer) {
        for i in 0..other.width {
            for j in 0..other.height {
                if let Some(pixel) = other.get(i, j) {
                    self.put(x + i, y + j, pixel);
                }
            }
        }
    }

    pub fn insert_bg(&mut self, x: u32, y: u32, other: ScreenBuffer) {
        for i in 0..other.width {
            for j in 0..other.height {
                if let Some(pixel) = other.get(i, j) {
                    self.put_bg(x + i, y + j, pixel.bg);
                }
            }
        }
    }

    pub fn insert_fg(&mut self, x: u32, y: u32, other: ScreenBuffer) {
        for i in 0..other.width {
            for j in 0..other.height {
                if let Some(pixel) = other.get(i, j) {
                    self.put_fg(x + i, y + j, pixel.fg_char, pixel.fg_color, pixel.fg_style);
                }
            }
        }
    }
}

// Buffer could be used as game manager
impl GameManager for ScreenBuffer {
    // dont really care about init
    fn init(&mut self, _width: u32, _height: u32, _has_fg: bool) {}

    fn draw(&self) -> ScreenBuffer {
        self.clone()
    }

    fn tick(&mut self) {}
    #[cfg(feature = "input")]
    fn action(&mut self, _player: u8, _action: PlayerAction) {}
}
