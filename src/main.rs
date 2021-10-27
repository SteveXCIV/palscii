use opts::AppOptions;


mod error;
mod font;
mod image;
mod opts;
mod palette;

fn main() {
    let app_options = AppOptions::parse();

    println!("{:?}", app_options);
}
