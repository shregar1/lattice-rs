use super::abstraction::AppException;
pub fn new_unauthenticated(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 401, response_key: "UNAUTHENTICATED".into() } }
