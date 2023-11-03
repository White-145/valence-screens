use valence::prelude::*;
use valence::text::color::RgbColor;
use crate::screen::buffer::ScreenBuffer;
use crate::screen::game_manager::GameManager;
use crate::screen::input::{PlayerAction, Uid};
use crate::screen::pixel::{BIAS_PIXEL, ScreenPixel, Style};

const PALETTE_WIDTH: u32 = 3;
const PALETTE_HEIGHT: u32 = 8;
const PALETTE: [RgbColor; 24] = [
    RgbColor { r : 170, g : 0, b : 0 },
    RgbColor { r : 255, g : 85, b : 85 },
    RgbColor { r : 255, g : 153, b : 255 },
    RgbColor { r : 102, g : 51, b : 0 },
    RgbColor { r : 255, g : 170, b : 0 },
    RgbColor { r : 255, g : 255, b : 85 },
    RgbColor { r : 0, g : 51, b : 0 },
    RgbColor { r : 0, g : 170, b : 0 },
    RgbColor { r : 85, g : 255, b : 85 },
    RgbColor { r : 0, g : 51, b : 102 },
    RgbColor { r : 0, g : 170, b : 170 },
    RgbColor { r : 85, g : 255, b : 255 },
    RgbColor { r : 0, g : 0, b : 170 },
    RgbColor { r : 85, g : 85, b : 255 },
    RgbColor { r : 153, g : 204, b : 255 },
    RgbColor { r : 170, g : 0, b : 170 },
    RgbColor { r : 153, g : 102, b : 255 },
    RgbColor { r : 255, g : 85, b : 255 },
    RgbColor { r : 85, g : 85, b : 85 },
    RgbColor { r : 170, g : 170, b : 170 },
    RgbColor { r : 255, g : 255, b : 255 },
    RgbColor { r : 0, g : 0, b : 0 },
    RgbColor { r : 0, g : 0, b : 0 },
    RgbColor { r : 0, g : 0, b : 0 },
];
const DECORATIONS_WIDTH: u32 = 10;
const DECORATIONS_HEIGHT: u32 = 8;
const DECORATIONS: [char; 80] = [
    '🗡', '🏹', '⛏', '🪓', '🔱', '🎣', '🧪',
    '⚗', '🍖', '🔥', '🌊', '☀', '☁', '🌧', '⛈',
    '☜', '☞', '☠', '☮', '☯',
    '★', '☆', '⯪', '⯫', '☽', ' ', '♠', '♣', '♥',
    '♦', '♯', '⚓', '⚔', '⚠', '🔔', '⚡', '✔',
    '❌', '❤', '✉', '⌛', '⏳', '⌚', '⚐', '⚑',
    '✎', '☈', '⧈', '0', '1',
    '2', '3', '4', '5', '6', '7', '8',
    '9', ',', '.', '/', '\\', '(', ')',
    '*', '&', '^', '%', '$', '#', '@', '!',
    '-', '=', '_', '+', '⏴', '⏵', '⏶', '⏷'
];

#[derive(Component)]
pub struct PaintGameManager {
    width : u32,
    height : u32,
    buffer : ScreenBuffer,
    is_palette_opened : bool,
    palette_cooldown : u32,
    // my solution on storing information for each player
    primary : [(RgbColor, char, RgbColor); Uid::MAX as usize],
    secondary : [(RgbColor, char, RgbColor); Uid::MAX as usize],
    positions : [Option<(u32, u32)>; Uid::MAX as usize],
    is_shown : [bool; Uid::MAX as usize],
    show_cooldown : [u32; Uid::MAX as usize],
}

impl Default for PaintGameManager {
    fn default() -> Self {
        PaintGameManager {
            width : 0,
            height : 0,
            buffer : ScreenBuffer::default(),
            is_palette_opened : true,
            palette_cooldown : 0,
            primary : [(RgbColor::new(0, 0, 0), ' ', RgbColor::new(255, 255, 255)); Uid::MAX as usize],
            secondary : [(RgbColor::new(255, 255, 255), ' ', RgbColor::new(0, 0, 0)); Uid::MAX as usize],
            positions : [None; Uid::MAX as usize],
            is_shown : [true; Uid::MAX as usize],
            show_cooldown : [0; Uid::MAX as usize],
        }
    }
}

impl GameManager for PaintGameManager {
    fn init(&mut self, width: u32, height: u32, _has_fg: bool) {
        self.width = width;
        self.height = height;
        self.buffer = ScreenBuffer::new(width, height);
        self.buffer.fill(ScreenPixel::new_bg(RgbColor::new(255, 255, 255)));
    }

    fn draw(&self) -> ScreenBuffer {
        let mut buffer = self.buffer.clone();
        if self.is_palette_opened {
            for x in 0..PALETTE_WIDTH {
                for y in 0..PALETTE_HEIGHT {
                    buffer.put(x, y, ScreenPixel::new(RgbColor::new(0, 0, 0), BIAS_PIXEL, PALETTE[(y * PALETTE_WIDTH + x) as usize], Style::default()));
                }
            }
            for x in 0..DECORATIONS_WIDTH {
                for y in 0..DECORATIONS_HEIGHT {
                    buffer.put(x + PALETTE_WIDTH, y, ScreenPixel::new(
                        RgbColor::new(0, 0, 0),
                        DECORATIONS[(y * DECORATIONS_WIDTH + x) as usize],
                        RgbColor::new(255, 255, 255),
                        Style::default()
                    ));
                }
            }
            for x in 0..PALETTE_WIDTH {
                for y in 0..PALETTE_HEIGHT {
                    buffer.put(self.width - PALETTE_WIDTH + x, y, ScreenPixel::new_bg(PALETTE[(y * PALETTE_WIDTH + x) as usize]));
                }
            }
        }
        for i in 0..Uid::MAX {
            let position = self.positions[i];
            let is_shown = self.is_shown[i];
            if is_shown {
                if let Some((x, y)) = position {
                    let color = buffer.get(x, y).unwrap_or_default().bg;
                    buffer.put_fg(x, y, '⏶', RgbColor::new(255 - color.r, 255 - color.g, 255 - color.b), Style::default());
                }
            }
        }
        buffer
    }

    fn tick(&mut self) {
        if self.palette_cooldown > 0 {
            self.palette_cooldown -= 1;
        }
        for i in 0..Uid::MAX {
            if self.show_cooldown[i] > 0 {
                self.show_cooldown[i] -= 1;
            }
        }
    }

    fn action(&mut self, player: Uid, action: PlayerAction) {
        if let PlayerAction::Secondary { position, is_sneaking } = action {
            let Some((x, y)) = position else {
                return;
            };

            let color = if is_sneaking {
                self.secondary[player as usize]
            } else {
                self.primary[player as usize]
            };

            self.buffer.put(x, y, ScreenPixel::new(color.0, color.1, color.2, Style::default()));
        } else if let PlayerAction::Swap { is_sneaking : _is_sneaking } = action {
            if self.palette_cooldown == 0 {
                self.is_palette_opened = !self.is_palette_opened;
                self.palette_cooldown = 20;
            }
        } else if let PlayerAction::Primary { position, is_sneaking } = action {
            let Some((x, y)) = position else {
                return;
            };
            let Some(pixel) = self.buffer.get(x, y) else {
                return;
            };

            let mut color = if is_sneaking {
                self.secondary[player as usize]
            } else {
                self.primary[player as usize]
            };
            if self.is_palette_opened {
                if x >= self.width - PALETTE_WIDTH && y < PALETTE_HEIGHT {
                    color.0 = PALETTE[(y * PALETTE_WIDTH + x + PALETTE_WIDTH - self.width) as usize];
                } else if x < PALETTE_WIDTH && y < PALETTE_HEIGHT {
                    color.2 = PALETTE[(y * PALETTE_WIDTH + x) as usize];
                } else if x < PALETTE_WIDTH + DECORATIONS_WIDTH && y < PALETTE_HEIGHT {
                    color.1 = DECORATIONS[(y * DECORATIONS_WIDTH + x - PALETTE_WIDTH) as usize];
                } else {
                    color = (pixel.bg, pixel.fg_char, pixel.fg_color);
                }
            } else {
                color = (pixel.bg, pixel.fg_char, pixel.fg_color);
            }

            if is_sneaking {
                self.secondary[player as usize] = color;
            } else {
                self.primary[player as usize] = color;
            }
        } else if let PlayerAction::Drop { is_sneaking : _is_sneaking } = action {
            if self.show_cooldown[player as usize] == 0 {
                self.show_cooldown[player as usize] = 10;
                let is_shown = self.is_shown[player as usize];
                self.is_shown[player as usize] = !is_shown;
            }
        } else if let PlayerAction::Hover { position, is_sneaking : _is_sneaking} = action {
            self.positions[player as usize] = position;
        } else if let PlayerAction::Disconnect = action {
            self.primary[player as usize] = (RgbColor::new(0, 0, 0), ' ', RgbColor::new(255, 255, 255));
            self.secondary[player as usize] = (RgbColor::new(255, 255, 255), ' ', RgbColor::new(0, 0, 0));
            self.positions[player as usize] = None;
            self.is_shown[player as usize] = true;
            self.show_cooldown[player as usize] = 0;
        }
    }
}
