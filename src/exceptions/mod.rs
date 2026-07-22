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

impl AppException {
    pub fn bad_request(msg: impl Into<String>) -> Self { Self { message: msg.into(), status_code: 400, response_key: "BAD_REQUEST".into() } }
    pub fn unauthenticated(msg: impl Into<String>) -> Self { Self { message: msg.into(), status_code: 401, response_key: "UNAUTHENTICATED".into() } }
    pub fn unauthorized(msg: impl Into<String>) -> Self { Self { message: msg.into(), status_code: 403, response_key: "UNAUTHORIZED".into() } }
    pub fn forbidden(msg: impl Into<String>) -> Self { Self { message: msg.into(), status_code: 403, response_key: "FORBIDDEN".into() } }
    pub fn not_found(msg: impl Into<String>) -> Self { Self { message: msg.into(), status_code: 404, response_key: "NOT_FOUND".into() } }
    pub fn conflict(msg: impl Into<String>) -> Self { Self { message: msg.into(), status_code: 409, response_key: "CONFLICT".into() } }
    pub fn internal_server(msg: impl Into<String>) -> Self { Self { message: msg.into(), status_code: 500, response_key: "INTERNAL_ERROR".into() } }
}
