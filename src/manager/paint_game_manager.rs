use rand::Rng;
use valence::prelude::*;
use valence::text::color::RgbColor;
use crate::screen::buffer::ScreenBuffer;
use crate::screen::game_manager::GameManager;
use crate::screen::input::{PlayerAction, Uid};
use crate::screen::pixel::{ScreenPixel, Style};

const DECORATIONS: [char; 15] = [
    '/', '#', '$', '%', '&',
    '@', '*', '=', '+', '[',
    ']', '~', '\'', '<', '>',
];
const DECORATION_COLOR_DIFF: u8 = 10;
const PALETTE_WIDTH: u32 = 3;
const PALETTE_HEIGHT: u32 = 8;

#[derive(Component)]
pub struct PaintGameManager {
    width : u32,
    height : u32,
    buffer : ScreenBuffer,
    is_palette_opened : bool,
    palette_cooldown : u32,
    primary_colors : [RgbColor; Uid::MAX as usize],
    secondary_colors : [RgbColor; Uid::MAX as usize],
    positions : [Option<(u32, u32)>; Uid::MAX as usize],
}

impl Default for PaintGameManager {
    fn default() -> Self {
        PaintGameManager {
            width : 0,
            height : 0,
            buffer : ScreenBuffer::default(),
            is_palette_opened : true,
            palette_cooldown : 0,
            primary_colors : [RgbColor::new(0, 0, 0); Uid::MAX as usize],
            secondary_colors : [RgbColor::new(255, 255, 255); Uid::MAX as usize],
            positions : [None; Uid::MAX as usize],
        }
    }
}

impl PaintGameManager {
    fn get_palette() -> ScreenBuffer {
        let mut buffer = ScreenBuffer::new(PALETTE_WIDTH, PALETTE_HEIGHT);
        buffer.put(0, 0, ScreenPixel::new_bg(RgbColor::new(170, 0, 0)));
        buffer.put(1, 0, ScreenPixel::new_bg(RgbColor::new(255, 85, 85)));
        buffer.put(2, 0, ScreenPixel::new_bg(RgbColor::new(255, 153, 255)));
        buffer.put(0, 1, ScreenPixel::new_bg(RgbColor::new(102, 51, 0)));
        buffer.put(1, 1, ScreenPixel::new_bg(RgbColor::new(255, 170, 0)));
        buffer.put(2, 1, ScreenPixel::new_bg(RgbColor::new(255, 255, 85)));
        buffer.put(0, 2, ScreenPixel::new_bg(RgbColor::new(0, 51, 0)));
        buffer.put(1, 2, ScreenPixel::new_bg(RgbColor::new(0, 170, 0)));
        buffer.put(2, 2, ScreenPixel::new_bg(RgbColor::new(85, 255, 85)));
        buffer.put(0, 3, ScreenPixel::new_bg(RgbColor::new(0, 51, 102)));
        buffer.put(1, 3, ScreenPixel::new_bg(RgbColor::new(0, 170, 170)));
        buffer.put(2, 3, ScreenPixel::new_bg(RgbColor::new(85, 255, 255)));
        buffer.put(0, 4, ScreenPixel::new_bg(RgbColor::new(0, 0, 170)));
        buffer.put(1, 4, ScreenPixel::new_bg(RgbColor::new(85, 85, 255)));
        buffer.put(2, 4, ScreenPixel::new_bg(RgbColor::new(153, 204, 255)));
        buffer.put(0, 5, ScreenPixel::new_bg(RgbColor::new(170, 0, 170)));
        buffer.put(1, 5, ScreenPixel::new_bg(RgbColor::new(153, 102, 255)));
        buffer.put(2, 5, ScreenPixel::new_bg(RgbColor::new(255, 85, 255)));
        buffer.put(0, 6, ScreenPixel::new_bg(RgbColor::new(85, 85, 85)));
        buffer.put(1, 6, ScreenPixel::new_bg(RgbColor::new(170, 170, 170)));
        buffer.put(2, 6, ScreenPixel::new_bg(RgbColor::new(255, 255, 255)));
        buffer.put(0, 7, ScreenPixel::new_bg(RgbColor::new(0, 0, 0)));
        buffer.put(1, 7, ScreenPixel::new_bg(RgbColor::new(0, 0, 0)));
        buffer.put(2, 7, ScreenPixel::new_bg(RgbColor::new(0, 0, 0)));
        buffer
    }
}

fn get_decoration_color(mut color: RgbColor) -> RgbColor {
    if color.r > DECORATION_COLOR_DIFF {
        color.r -= DECORATION_COLOR_DIFF;
    } else {
        color.r += DECORATION_COLOR_DIFF * 2;
    }
    if color.g > DECORATION_COLOR_DIFF {
        color.g -= DECORATION_COLOR_DIFF;
    } else {
        color.g += DECORATION_COLOR_DIFF * 2;
    }
    if color.b > DECORATION_COLOR_DIFF {
        color.b -= DECORATION_COLOR_DIFF;
    } else {
        color.b += DECORATION_COLOR_DIFF * 2;
    }
    color
}

impl GameManager for PaintGameManager {
    fn init(&mut self, width: u32, height: u32, _has_fg: bool) {
        self.width = width;
        self.height = height;
        let mut rng = rand::thread_rng();
        self.buffer = ScreenBuffer::construct(width, height, |_x, _y| {
            let mut pixel = ScreenPixel::new_bg(RgbColor::new(255, 255, 255));
            if rng.gen_bool(0.5) {
                let fg = DECORATIONS[rng.gen_range(0..DECORATIONS.len())];
                pixel.fg = (fg, get_decoration_color(RgbColor::new(255, 255, 255)), Style::default());
            }
            pixel
        });
    }

    fn draw(&self) -> ScreenBuffer {
        let mut buffer = self.buffer.clone();
        if self.is_palette_opened {
            buffer.insert(self.width - PALETTE_WIDTH, 0, PaintGameManager::get_palette());
        }
        for position in self.positions {
            if let Some((x, y)) = position {
                let color = buffer.get(x, y).unwrap_or_default().bg;
                buffer.put_fg(x, y, '⏶', RgbColor::new(255 - color.r, 255 - color.g, 255 - color.b), Style::default());
            }
        }
        buffer
    }

    fn tick(&mut self) {
        if self.palette_cooldown > 0 {
            self.palette_cooldown -= 1;
        }
    }

    fn action(&mut self, player: Uid, action: PlayerAction) {
        if let PlayerAction::Secondary { position, is_sneaking } = action {
            let Some((x, y)) = position else {
                return;
            };

            let color = if is_sneaking {
                self.secondary_colors[player as usize]
            } else {
                self.primary_colors[player as usize]
            };

            let decoration = self.buffer.get(x, y).unwrap_or_default().fg;
            let decoration_color = get_decoration_color(color);
            self.buffer.put(x, y, ScreenPixel::new(color, decoration.0, decoration_color, decoration.2));
        } else if let PlayerAction::Swap { is_sneaking : _is_sneaking } = action {
            if self.palette_cooldown == 0 {
                self.is_palette_opened = !self.is_palette_opened;
                self.palette_cooldown = 40;
            }
        } else if let PlayerAction::Primary { position, is_sneaking } = action {
            let Some((x, y)) = position else {
                return;
            };
            let Some(pixel) = self.buffer.get(x, y) else {
                return;
            };

            let mut color = pixel.bg;
            if self.is_palette_opened {
                if x >= self.width - PALETTE_WIDTH && y < PALETTE_HEIGHT {
                    color = PaintGameManager::get_palette().get(x + PALETTE_WIDTH - self.width, y).unwrap_or_default().bg;
                }
            }

            if is_sneaking {
                self.secondary_colors[player as usize] = color;
            } else {
                self.primary_colors[player as usize] = color;
            }
        } else if let PlayerAction::Hover { position, is_sneaking : _is_sneaking} = action {
            self.positions[player as usize] = position;
        } else if let PlayerAction::Disconnect = action {
            self.primary_colors[player as usize] = RgbColor::new(0, 0, 0);
            self.secondary_colors[player as usize] = RgbColor::new(255, 255, 255);
            self.positions[player as usize] = None;
        }
    }
}
