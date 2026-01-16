use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::fmt::Write;

pub struct Renderer {
    pub area_size: u32, // The "Zoom" factor (area_size x area_size pixels per block)
}

impl Renderer {
    pub fn new(area_size: u32) -> Self {
        Self { area_size }
    }

    /// Calculates the average RGB color for a specific rectangular area of the image
    fn get_average_rgb(&self, img: &DynamicImage, start_x: u32, start_y: u32) -> [u8; 3] {
        let (w, h) = img.dimensions();
        let mut r_total: u64 = 0;
        let mut g_total: u64 = 0;
        let mut b_total: u64 = 0;
        let mut count: u64 = 0;

        // Iterate over the square area defined by area_size
        for py in start_y..(start_y + self.area_size).min(h) {
            for px in start_x..(start_x + self.area_size).min(w) {
                let pixel = img.get_pixel(px, py);
                r_total += pixel[0] as u64;
                g_total += pixel[1] as u64;
                b_total += pixel[2] as u64;
                count += 1;
            }
        }

        if count == 0 {
            return [0, 0, 0];
        }

        [
            (r_total / count) as u8,
            (g_total / count) as u8,
            (b_total / count) as u8,
        ]
    }

    pub fn render(&self, img: &DynamicImage) -> Result<String> {
        let (w, h) = img.dimensions();
        let mut output = String::new();

        // Move in steps of area_size for X
        // Move in steps of (area_size * 2) for Y because one char = two vertical blocks
        for y in (0..h).step_by((self.area_size * 2) as usize) {
            for x in (0..w).step_by(self.area_size as usize) {
                // Average for the "Top" half of the terminal character
                let top = self.get_average_rgb(img, x, y);

                // Average for the "Bottom" half (offset by one area_size vertically)
                let bot = if y + self.area_size < h {
                    self.get_average_rgb(img, x, y + self.area_size)
                } else {
                    top // Fallback if image ends mid-character
                };

                write!(
                    output,
                    "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄",
                    top[0], top[1], top[2], bot[0], bot[1], bot[2]
                )?;
            }
            output.push_str("\x1b[0m\n");
        }
        Ok(output)
    }
}
