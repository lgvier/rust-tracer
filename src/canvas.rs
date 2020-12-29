use std::io::{Result, BufWriter};
use std::fs::File;

use super::color::*;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    canvas: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let black_row = vec![BLACK; width];
        let canvas = vec![black_row; height];
        Self {width, height, canvas}
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.canvas[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.canvas[y][x] = color;
    }

    fn to_u8_rgb(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; self.width * self.height * 3];
        let mut index = 0usize;
        for row in &self.canvas {
            for c in row {
                c.write_as_u8_rgb(&mut bytes, index);
                index += 3;
            }
        }
        bytes
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let file_writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(file_writer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut png_writer = encoder.write_header()?;

        png_writer.write_image_data(&self.to_u8_rgb())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_ctor() {
        let c = Canvas::new(10, 20);
        assert_eq!(10, c.width);
        assert_eq!(20, c.height);
        assert_eq!(BLACK, c.pixel_at(1, 1));
    }

    #[test]
    fn canvas_write() {
        let mut c = Canvas::new(10, 20);
        assert_eq!(10, c.width);
        assert_eq!(20, c.height);
        assert_eq!(BLACK, c.pixel_at(1, 1));
        c.write_pixel(1, 1, RED);
        assert_eq!(RED, c.pixel_at(1, 1));
    }

    #[test]
    fn canvas_save() -> Result<()> {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(1, 1, RED);
        c.write_pixel(1, 2, GREEN);
        c.write_pixel(1, 3, GREEN);
        c.write_pixel(1, 4, GREEN);
        c.write_pixel(1, 5, GREEN);
        c.write_pixel(1, 6, GREEN);
        c.write_pixel(2, 1, BLUE);
        c.write_pixel(3, 1, BLUE);
        c.write_pixel(4, 1, BLUE);
        c.write_pixel(5, 1, BLUE);
        c.write_pixel(6, 1, BLUE);

        let pixels = c.to_u8_rgb();
        // println!("{:?}", pixels);

        // 1, 1 is red
        assert_eq!(255, pixels[(1 + c.width * 1) * 3]);
        assert_eq!(0, pixels[(1 + c.width * 1) * 3 + 1]);
        assert_eq!(0, pixels[(1 + c.width * 1) * 3 + 2]);
        // 1, 6 is green
        assert_eq!(0, pixels[(1 + c.width * 6) * 3]);
        assert_eq!(255, pixels[(1 + c.width * 6) * 3 + 1]);
        assert_eq!(0, pixels[(1 + c.width * 6) * 3 + 2]);
        // 6, 1 is blue
        assert_eq!(0, pixels[(6 + c.width * 1) * 3]);
        assert_eq!(0, pixels[(6 + c.width * 1) * 3 + 1]);
        assert_eq!(255, pixels[(6 + c.width * 1) * 3 + 2]);

        // c.save("/tmp/canvas_save_test.png")?;
        Ok(())
    }
}