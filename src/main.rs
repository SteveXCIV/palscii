use crate::image::ImageWriter;
use error::AppError;
use font::Rasterizer;
use opts::{AppOptions, Sink, Source};
use palette::Palette;
use std::{fs::File, io};

mod error;
mod font;
mod image;
mod opts;
mod palette;

static GLYPHS: [char; 256] = [
    '\0', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', '►', '◄', '↕',
    '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', ' ', '!', '"', '#', '$', '%',
    '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8',
    '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^',
    '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
    'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂', 'Ç', 'ü', 'é', 'â', 'ä',
    'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', 'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù',
    'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', 'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬',
    '½', '¼', '¡', '«', '»', '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜',
    '╛', '┐', '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', '╨',
    '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', 'α', 'ß', 'Γ', 'π',
    'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', '≡', '±', '≥', '≤', '⌠', '⌡', '÷',
    '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ',
];
const ROWS: u32 = 16;
const COLS: u32 = 16;

fn main() -> Result<(), AppError> {
    let app_options = AppOptions::parse();

    let rasterizer = match app_options.source {
        Source::File(input_path) => Rasterizer::load_from_file(&input_path)?,
        Source::StdIn => Rasterizer::load_from(&mut io::stdin())?,
    };

    let mut palette = Palette::new(ROWS, COLS, app_options.width, app_options.height);
    rasterizer.render_to(&GLYPHS, &mut palette);

    let writer = ImageWriter::from_palette(palette);
    match app_options.sink {
        Sink::File(output_path) => {
            let output = File::create(output_path).map_err(|e| AppError::IOError(e.to_string()))?;
            writer.write_to(output)?;
        }
        Sink::StdOut => {
            writer.write_to(io::stdout())?;
        }
    };

    Ok(())
}
