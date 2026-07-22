use super::abstraction::AppException;
pub fn new_unauthorized(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 403, response_key: "UNAUTHORIZED".into() } }
