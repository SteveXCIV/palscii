use fontdue::Font;
use std::{ffi::OsStr, io::Read};

#[derive(Debug)]
pub struct Rasterizer {
    font: Font,
}

impl Rasterizer {
    /// Load a [Rasterizer] from a file path.
    pub fn load_from_file<S: AsRef<OsStr> + ?Sized>(s: &S) -> Result<Rasterizer, String> {
        todo!()
    }

    /// Load a [Rasterizer] from any [Read] implementation.
    pub fn load_from<R: Read>(reader: &mut R) -> Result<Rasterizer, String> {
        todo!()
    }

    /// Given dimensions and a glyph, get the px size needed to make Fontdue
    /// rasterize within those bounds.
    fn get_scaled_px(&self, width: u32, height: u32, glyph: char) -> f32 {
        todo!()
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
