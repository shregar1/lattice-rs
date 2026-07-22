use std::fmt;

#[derive(Debug)]
pub struct AppException {
    pub message: String,
    pub status_code: u16,
    pub response_key: String,
}

impl fmt::Display for AppException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}] {}", self.status_code, self.response_key, self.message)
    }
}

impl std::error::Error for AppException {}
