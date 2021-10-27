#[derive(Debug)]
pub enum AppError {
    IOError(String),
    FormatError(String),
}
