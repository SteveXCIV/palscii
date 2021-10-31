use crate::palette::Palette;

use super::error::AppError;
use fontdue::{Font, FontSettings};
use std::{ffi::OsStr, fs::File, io::Read, path::Path};

#[derive(Debug)]
pub struct Rasterizer {
    font: Font,
}

impl Rasterizer {
    /// Load a [Rasterizer] from a file path.
    pub fn load_from_file<S: AsRef<OsStr> + ?Sized>(s: &S) -> Result<Rasterizer, AppError> {
        let mut file = File::open(Path::new(s)).map_err(|e| AppError::IOError(e.to_string()))?;
        Self::load_from(&mut file)
    }

    /// Load a [Rasterizer] from any [Read] implementation.
    pub fn load_from<R: Read>(reader: &mut R) -> Result<Rasterizer, AppError> {
        let mut buf = Vec::new();
        reader
            .read_to_end(&mut buf)
            .map_err(|e| AppError::IOError(e.to_string()))?;
        let font = Font::from_bytes(buf, FontSettings::default())
            .map_err(|e| AppError::FormatError(e.to_string()))?;
        Ok(Rasterizer { font })
    }

    pub fn render_to(&self, glyphs: &[char], palette: &mut Palette) {
        let (rows, cols) = palette.get_dimensions();
        let (width, height) = palette.get_cell_dimensions();

        assert_eq!(
            rows * cols,
            glyphs.len() as u32,
            "dimension mismatch - cannot render {} glyph(s) to {}x{} palette",
            glyphs.len(),
            rows,
            cols
        );

        for (index, &glyph) in glyphs.iter().enumerate() {
            let index = index as u32;
            let px = self.get_scaled_px(width, height, glyph);
            let (metrics, buffer) = self.font.rasterize(glyph, px);
            let (glyph_width, glyph_height) = (
                u32::min(metrics.width as u32, width),
                u32::min(metrics.height as u32, height),
            );
            let (offset_x, offset_y) = ((width - glyph_width) / 2, (height - glyph_height) / 2);
            let row = index / cols;
            let col = index % cols;
            for x in 0..glyph_width {
                for y in 0..glyph_height {
                    let is_filled = buffer[(y * glyph_width + x) as usize] > 0;
                    palette.set(row, col, x + offset_x, y + offset_y, is_filled);
                }
            }
        }
    }

    /// Given dimensions and a glyph, get the max px size needed to rasterize
    /// within those bounds.
    fn get_scaled_px(&self, width: u32, height: u32, glyph: char) -> f32 {
        let metric = self.font.metrics(glyph, width as f32);
        // these will never render anything, so the size is not relevant
        if metric.width == 0 || metric.height == 0 {
            return 1.0;
        }
        // TODO: is this actually right?
        let px;
        if metric.width > metric.height {
            px = (width as f32 / metric.width as f32) * width as f32;
        } else {
            px = (height as f32 / metric.height as f32) * height as f32;
        }

        #[cfg(debug_assertions)]
        {
            let metric = self.font.metrics(glyph, px);
            debug_assert!(
                metric.width <= width as usize,
                "Width failed debug check: expected={}, actual={}",
                width,
                metric.width
            );
            debug_assert!(
                metric.height <= height as usize,
                "Height failed debug check: expected={}, actual={}",
                height,
                metric.height
            );
        }

        px
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_loads_otf_font() {
        let actual =
            Rasterizer::load_from_file("resources/source-code-pro/SourceCodePro-Regular.otf");

        assert!(actual.is_ok(), "Expected Ok, got: {:?}", actual);
    }

    #[test]
    fn it_loads_ttf_font() {
        let actual = Rasterizer::load_from_file("resources/VT323/VT323-Regular.ttf");

        assert!(actual.is_ok(), "Expected Ok, got: {:?}", actual);
    }

    const TEST_FONT_BYTES: &[u8] =
        include_bytes!("../resources/VT323/VT323-Regular.ttf") as &[u8];

    #[test]
    fn it_scales_glyphs() {
        let rasterizer = Rasterizer::load_from(&mut TEST_FONT_BYTES.clone())
            .expect("Failed to load embedded test font.");

        for code in 0u32..256u32 {
            let ch = char::from_u32(code).expect(&format!("Failed to convert to char: {}", code));
            let px = rasterizer.get_scaled_px(16, 8, ch);
            let (metric, _) = rasterizer.font.rasterize(ch, px);
            assert!(
                metric.width <= 16,
                "Expected width <= 16: ch={}, px={}, metric={:?}",
                ch,
                px,
                metric
            );
            assert!(
                metric.height <= 8,
                "Expected width <= 8: ch={}, px={}, metric={:?}",
                ch,
                px,
                metric
            );
        }
    }
}
