use clap::{AppSettings, Parser};

/// palscii - generate ASCII font palettes for rougelike engines
///
/// palscii is designed to be a dead-simple UNIX-like tool.
/// It takes font files and makes PNG palettes, and it aims to do that well.
#[derive(Debug, Parser)]
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
    #[clap(short, long)]
    format: Option<String>,

    /// Optional path to output to, if not provided, STDOUT will be used
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    println!("{:?}", opts);
}
