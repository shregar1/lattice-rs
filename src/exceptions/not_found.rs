use super::abstraction::AppException;
pub fn new_not_found(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 404, response_key: "NOT_FOUND".into() } }
