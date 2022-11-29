use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub status_code: u16,
    pub message: String,
}

impl CustomError {
    pub fn new(status_code: u16, message: String) -> Self {
        CustomError { status_code, message }
    }
}