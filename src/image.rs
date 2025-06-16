use image::{DynamicImage, GenericImageView};
use palette::{color_difference::Ciede2000, convert::FromColorUnclamped, Lab, Srgb};
use valence::text::color::RgbColor;

use crate::{
    buffer::ScreenBuffer,
    pixel::{ScreenPixel, Style},
};

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
            let srgb_color1 = Srgb::new(
                color1.r as f32 / 255.0,
                color1.g as f32 / 255.0,
                color1.b as f32 / 255.0,
            );
            let srgb_color2 = Srgb::new(
                color2.r as f32 / 255.0,
                color2.g as f32 / 255.0,
                color2.b as f32 / 255.0,
            );
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
                        ver_ratio as u32,
                    );
                    let mean = get_mean_color(&part);
                    let dominant = get_dominant_color(&part);
                    let difference = get_diff(&dominant, &mean);
                    let intensity = INTENSITY[(difference * INTENSITY.len() as f32) as usize];
                    buffer.put(
                        x,
                        y,
                        ScreenPixel::new(mean, intensity, dominant, Style::default()),
                    );
                }
            }
        }
        buffer
    }
}
