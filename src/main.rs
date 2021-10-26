use opts::AppOptions;


mod font;
mod opts;
mod palette;

fn main() {
    let app_options = AppOptions::parse();

    println!("{:?}", app_options);
}
