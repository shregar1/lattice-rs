use super::abstraction::AppException;
pub fn new_duplicate_key(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 409, response_key: "DUPLICATE_KEY".into() } }
