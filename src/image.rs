use super::error::AppError;
use super::palette::Palette;
use image::png::{CompressionType, FilterType, PngEncoder};
use image::{ColorType, ImageBuffer, ImageEncoder, Rgba, RgbaImage};
use std::io::Write;

#[derive(Debug)]
pub struct ImageWriter {
    buffer: RgbaImage,
}

const FOREGROUND: Rgba<u8> = Rgba([255, 255, 255, 255]);
const BACKGROUND: Rgba<u8> = Rgba([0, 0, 0, 0]);

impl ImageWriter {
    pub fn from_palette(palette: Palette) -> Self {
        let (rows, cols) = palette.get_dimensions();
        let (width, height) = palette.get_cell_dimensions();
        let (buf_width, buf_height) = (cols * width, rows * height);
        let mut buffer: RgbaImage = ImageBuffer::new(buf_width, buf_height);
        // time to build some pyramids!
        for row in 0..rows {
            for col in 0..cols {
                let cell = palette.get(row, col);
                for x in 0..width {
                    for y in 0..height {
                        let set = cell[(y * width + x) as usize];
                        let global_x = x + col * width;
                        let global_y = y + row * height;
                        if set {
                            buffer.put_pixel(global_x, global_y, FOREGROUND);
                        } else {
                            buffer.put_pixel(global_x, global_y, BACKGROUND);
                        }
                    }
                }
            }
        }

        ImageWriter { buffer }
    }

    pub fn write_to<W: Write>(&self, writer: W) -> Result<(), AppError> {
        let enc = PngEncoder::new_with_quality(writer, CompressionType::Fast, FilterType::NoFilter);
        enc.write_image(
            &self.buffer,
            self.buffer.width(),
            self.buffer.height(),
            ColorType::Rgba8,
        )
        .map_err(|e| AppError::IOError(e.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_builds_from_palette() {
        let mut palette = Palette::new(1, 1, 3, 4);
        palette.set(0, 0, 1, 1, true);
        palette.set(0, 0, 2, 2, true);
        let expected = &[
            //
            BACKGROUND, BACKGROUND, BACKGROUND, //
            BACKGROUND, FOREGROUND, BACKGROUND, //
            BACKGROUND, BACKGROUND, FOREGROUND, //
            BACKGROUND, BACKGROUND, BACKGROUND,
        ];
        let writer = ImageWriter::from_palette(palette);

        let actual: Vec<Rgba<u8>> = writer.buffer.pixels().map(|p| *p).collect();

        assert_eq!(actual, expected);
    }
}
