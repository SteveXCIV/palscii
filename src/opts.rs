use clap::{ErrorKind, FromArgMatches, IntoApp, Parser};
use std::{ffi::OsString, path::Path, str::FromStr};

/// palscii - generate ASCII font palettes for rougelike engines
///
/// palscii is designed to be a dead-simple UNIX-like tool.
/// It takes font files and makes PNG palettes, and it aims to do that well.
#[derive(Debug, Eq, PartialEq, Parser)]
#[clap()]
struct Opts {
    /// Optional path to input file, if not provided, STDIN will be used
    #[clap(short, long)]
    input: Option<String>,

    /// The input format; palscii supports .otf, .ttc, and .ttf files.
    /// If `--input` is supplied, this value will be inferred from the filename.
    /// If `--input` is not supplied, this argument must be.
    /// If both `--input` and this argument are supplied, this argument takes priority.
    /// Supplying nothing, or an invalid option is an error and terminates the program.
    #[clap(short, long, possible_values = [Format::OPEN_TYPE, Format::TRUE_TYPE])]
    format: Option<Format>,

    /// Optional path to output to, if not provided, STDOUT will be used
    #[clap(short, long)]
    output: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Parser)]
pub enum Format {
    OpenType,
    TrueType,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Format::OPEN_TYPE => Ok(Format::OpenType),
            Format::TRUE_TYPE => Ok(Format::TrueType),
            other => Err(format!("Unknown font type: {}", other)),
        }
    }
}

impl Format {
    /// The string constant `"otf"`.
    const OPEN_TYPE: &'static str = "otf";

    /// The string constant `"ttf"`.
    const TRUE_TYPE: &'static str = "ttf";
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
    source: Source,
    sink: Sink,
    format: Format,
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

        let mut format: Option<Format> = None;
        let mut source = Source::StdIn;
        let mut sink = Sink::StdOut;

        if let Some(explicit_format) = inner.format {
            format = Some(explicit_format);
        }

        if let Some(path) = inner.input {
            if format.is_none() {
                let extension = Path::new(&path)
                    .extension()
                    .and_then(|os_str| os_str.to_str());
                match extension {
                    Some("otf") => format = Some(Format::OpenType),
                    Some("ttf") | Some("ttc") => format = Some(Format::TrueType),
                    _ => {}
                }
            }

            source = Source::File(path);
        }

        // if format is not explicitly set, and we can't infer it, we need to bail
        if format.is_none() {
            return Err(Opts::into_app().error(
                ErrorKind::InvalidValue,
                "Could not determine font format, supported formats are: .otf, .ttc, .ttf",
            ));
        }

        if let Some(path) = inner.output {
            sink = Sink::File(path);
        }

        Ok(AppOptions {
            source,
            sink,
            format: format.unwrap(),
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

    #[test]
    fn it_parses_format() {
        let open_type = Format::from_str(Format::OPEN_TYPE);
        let true_type = Format::from_str(Format::TRUE_TYPE);
        let other = Format::from_str("some other font type");

        assert_eq!(open_type, Ok(Format::OpenType));
        assert_eq!(true_type, Ok(Format::TrueType));
        assert!(other.is_err());
    }

    #[test_case(
        &[],
        Ok(Opts{input: None, format: None, output: None});
        "empty args"
    )]
    #[test_case(
        &["palscii", "-i", "/home/some_font.otf", "-o", "/home/some_font.png"],
        Ok(Opts{input: Some("/home/some_font.otf".to_string()), format: None, output: Some("/home/some_font.png".to_string())});
        "short args"
    )]
    #[test_case(
        &["palscii", "--input=/home/some_font.otf"],
        Ok(Opts{input: Some("/home/some_font.otf".to_string()), format: None, output: None});
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
        Err(());
        "empty args"
    )]
    #[test_case(
        &["palscii", "-i", "/home/some_font.otf"],
        Ok(AppOptions{source: Source::File("/home/some_font.otf".to_string()), sink: Sink::StdOut, format: Format::OpenType});
        "implied format OTF"
    )]
    #[test_case(
        &["palscii", "-i", "/home/some_font.ttf"],
        Ok(AppOptions{source: Source::File("/home/some_font.ttf".to_string()), sink: Sink::StdOut, format: Format::TrueType});
        "implied format TTF"
    )]
    #[test_case(
        &["palscii", "-i", "/home/some_font.asdf"],
        Err(());
        "implied format unknown"
    )]
    #[test_case(
        &["palscii", "-f", Format::OPEN_TYPE],
        Ok(AppOptions{source: Source::StdIn, sink: Sink::StdOut, format: Format::OpenType});
        "explicit format OTF"
    )]
    #[test_case(
        &["palscii", "-f", Format::TRUE_TYPE],
        Ok(AppOptions{source: Source::StdIn, sink: Sink::StdOut, format: Format::TrueType});
        "explicit format TTF"
    )]
    #[test_case(
        &["palscii", "-f", "asdf"],
        Err(());
        "explicit format unknown"
    )]
    #[test_case(
        &["palscii", "-i", "/home/some_font.ttc", "-f", Format::OPEN_TYPE],
        Ok(AppOptions{source: Source::File("/home/some_font.ttc".to_string()), sink: Sink::StdOut, format: Format::OpenType});
        "explicit format overrides implied"
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
