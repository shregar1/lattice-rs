use super::abstraction::AppException;
pub fn new_forbidden(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 403, response_key: "FORBIDDEN".into() } }
