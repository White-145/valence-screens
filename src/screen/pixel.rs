use valence::Text;
use valence::text::IntoText;
use valence::text::color::RgbColor;

pub const BG_PIXEL: char = '■';
pub const BIAS_PIXEL: char = '█';

// Foreground style
#[derive(Clone, Copy)]
pub struct Style {
    bold : bool,
    strikethrough : bool,
    underlined : bool,
    italic : bool,
}

impl Default for Style {
    fn default() -> Self {
        Style { bold : false, strikethrough : false, underlined : false, italic : false }
    }
}

impl Style {
    pub fn new(bold: bool, strikethrough: bool, underlined: bool, italic: bool) -> Self {
        Style { bold, strikethrough, underlined, italic }
    }

    // apply style to the text
    pub fn apply(&self, mut text: Text) -> Text {
        text = if self.bold { text.bold() } else { text.not_bold() };
        text = if self.strikethrough { text.strikethrough() } else { text.not_strikethrough() };
        text = if self.underlined { text.underlined() } else { text.not_underlined() };
        text = if self.italic { text.italic() } else { text.not_italic() };
        text
    }
}

#[derive(Clone)]
pub struct ScreenPixel {
    pub bg : RgbColor,
    pub fg : (char, RgbColor, Style),
}

impl Default for ScreenPixel {
    fn default() -> Self {
        ScreenPixel { bg: RgbColor::new(0, 0, 0), fg: (' ', RgbColor::new(255, 255, 255), Style::default()) }
    }
}

impl ScreenPixel {
    pub fn new(bg: RgbColor, fg_char: char, fg_color: RgbColor, fg_style: Style) -> Self {
        ScreenPixel { bg, fg : (fg_char, fg_color, fg_style) }
    }

    pub fn new_bg(bg: RgbColor) -> Self {
        ScreenPixel { bg, fg : (' ', RgbColor::new(255, 255, 255), Style::default()) }
    }

    pub fn new_fg(fg_char: char, fg_color: RgbColor, fg_style: Style) -> Self {
        ScreenPixel { bg : RgbColor::new(0, 0, 0), fg : (fg_char, fg_color, fg_style) }
    }
}
