use super::palette::Palette;
use image::RgbaImage;
use std::io::Write;

#[derive(Debug)]
pub struct ImageWriter {
    buffer: RgbaImage,
}

impl ImageWriter {
    pub fn from_palette(palette: Palette) -> Self {
        todo!()
    }

    pub fn write_to<W: Write>(writer: W) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use image::Rgba;

    use super::*;

    #[test]
    fn it_builds_from_palette() {
        let mut palette = Palette::new(1, 1, 3, 4);
        palette.set(0, 0, 1, 1, true);
        palette.set(0, 0, 2, 2, true);
        let expected = &[
            //
            false, false, false, //
            false, true, false, //
            false, false, true, //
            false, false, false,
        ];
        let writer = ImageWriter::from_palette(palette);

        let actual: Vec<bool> = writer
            .buffer
            .pixels()
            .map(|pixel| *pixel == Rgba([255, 255, 255, 255]))
            .collect();

        assert_eq!(actual, expected);
    }
}
