use fontdue::{Font, FontSettings};
use std::{ffi::OsStr, fs::File, io::Read, path::Path};

#[derive(Debug)]
pub struct Rasterizer {
    font: Font,
}

impl Rasterizer {
    /// Load a [Rasterizer] from a file path.
    pub fn load_from_file<S: AsRef<OsStr> + ?Sized>(s: &S) -> Result<Rasterizer, String> {
        let mut file = File::open(Path::new(s)).map_err(|e| e.to_string())?;
        Self::load_from(&mut file)
    }

    /// Load a [Rasterizer] from any [Read] implementation.
    pub fn load_from<R: Read>(reader: &mut R) -> Result<Rasterizer, String> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        let font = Font::from_bytes(buf, FontSettings::default()).map_err(|e| e.to_string())?;
        Ok(Rasterizer { font })
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
        if metric.width > metric.height {
            width as f32 / metric.width as f32
        } else {
            height as f32 / metric.height as f32
        }
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
