use clap::{FromArgMatches, IntoApp, Parser};
use std::ffi::OsString;

/// palscii - generate ASCII font palettes for rougelike engines
///
/// palscii is designed to be a dead-simple UNIX-like tool.
/// It takes font files and makes PNG palettes, and it aims to do that well.
///
/// ### NOTES
///
/// The glyph parameters `width` and `height` are scaled to on a best-case
/// scenario.
/// In situations where it is not possible to do so, cropping will occur and may
/// result in undesirable visual artifacts.
///
/// In DEBUG builds of palscii, an assertion checks the validity of the scale
/// operation, so palscii will exit with an error.
#[derive(Debug, Eq, PartialEq, Parser)]
#[clap()]
struct Opts {
    /// Optional path to input file, if not provided, STDIN will be used
    #[clap(short, long)]
    input: Option<String>,

    /// Optional path to output to, if not provided, STDOUT will be used
    #[clap(short, long)]
    output: Option<String>,

    /// Optional maximum glyph width, default: 8. See **Notes** for more details.
    #[clap(short, long, default_value = "8")]
    width: u32,

    /// Optional maximum glyph height, defualt: 16. See **Notes** for more details.
    #[clap(short, long, default_value = "16")]
    height: u32,
}

/// A font file input.
#[derive(Debug, PartialEq, Eq)]
pub enum Source {
    /// Input from a file on disk.
    File(String),

    /// Input from STDIN.
    StdIn,
}

/// A palette file output.
#[derive(Debug, PartialEq, Eq)]
pub enum Sink {
    /// Output to a file on disk.
    File(String),

    /// Output to STDOUT.
    StdOut,
}

/// A higher-level options type that can be created from [Opts].
#[derive(Debug, PartialEq, Eq)]
pub struct AppOptions {
    pub source: Source,
    pub sink: Sink,
    pub width: u32,
    pub height: u32,
}

impl AppOptions {
    /// Attempts to parse options from an arbitrary source, failing on an error.
    fn try_parse<I, T>(args: I) -> Result<Self, clap::Error>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = Opts::into_app().try_get_matches_from(args)?;
        let inner = Opts::from_arg_matches(&matches).expect("Failed to parse options.");
        let mut source = Source::StdIn;
        let mut sink = Sink::StdOut;

        if let Some(path) = inner.input {
            source = Source::File(path);
        }

        if let Some(path) = inner.output {
            sink = Sink::File(path);
        }

        Ok(AppOptions {
            source,
            sink,
            width: inner.width,
            height: inner.height,
        })
    }

    /// Attempts to parse options from `std::env::args_os()` and exits on an error.
    /// This is a thin wrapper around the underlying `clap` types, see `clap::App`
    /// for more details.
    pub fn parse() -> Self {
        match Self::try_parse(std::env::args_os()) {
            Err(e) => {
                e.exit();
            }
            Ok(app_opts) => app_opts,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(
        &[],
        Ok(Opts{input: None, output: None, width: 8, height: 16});
        "empty args"
    )]
    #[test_case(
        &["palscii", "-i", "/home/some_font.otf", "-o", "/home/some_font.png", "-w", "64", "-h", "128"],
        Ok(Opts{input: Some("/home/some_font.otf".to_string()), output: Some("/home/some_font.png".to_string()), width: 64, height: 128});
        "short args"
    )]
    #[test_case(
        &["palscii", "--input=/home/some_font.otf", "--width", "64"],
        Ok(Opts{input: Some("/home/some_font.otf".to_string()), output: None, width: 64, height: 16});
        "long args"
    )]
    fn it_parses_opts(args: &[&'static str], expected: Result<Opts, ()>) {
        let actual = Opts::try_parse_from(args.iter());

        match expected {
            Ok(expected) => {
                assert!(actual.is_ok(), "Expected Ok, got: {:?}", actual);
                assert_eq!(actual.unwrap(), expected);
            }
            Err(_) => assert!(actual.is_err(), "Expected Err, got: {:?}", actual),
        }
    }

    #[test_case(
        &[],
        Ok(AppOptions{source: Source::StdIn, sink: Sink::StdOut, width: 8, height: 16});
        "empty args"
    )]
    #[test_case(
        &["palscii", "-i", "/home/some_font.otf"],
        Ok(AppOptions{source: Source::File("/home/some_font.otf".to_string()), sink: Sink::StdOut, width: 8, height: 16});
        "short args"
    )]
    #[test_case(
        &["palscii", "--input=/home/some_font.otf"],
        Ok(AppOptions{source: Source::File("/home/some_font.otf".to_string()), sink: Sink::StdOut, width: 8, height: 16});
        "long args"
    )]
    fn it_parses_app_options(args: &[&'static str], expected: Result<AppOptions, ()>) {
        let actual = AppOptions::try_parse(args.iter());

        match expected {
            Ok(expected) => {
                assert!(actual.is_ok(), "Expected Ok, got: {:?}", actual);
                assert_eq!(actual.unwrap(), expected);
            }
            Err(_) => assert!(actual.is_err(), "Expected Err, got: {:?}", actual),
        }
    }
}
