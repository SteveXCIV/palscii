use opts::AppOptions;

mod opts;

fn main() {
    let app_options = AppOptions::parse();

    println!("{:?}", app_options);
}
