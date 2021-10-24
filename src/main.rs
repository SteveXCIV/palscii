use clap::Parser;
use opts::Opts;

mod opts;

fn main() {
    let opts = Opts::parse();

    println!("{:?}", opts);
}
