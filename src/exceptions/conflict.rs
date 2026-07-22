use super::abstraction::AppException;
pub fn new_conflict(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 409, response_key: "CONFLICT".into() } }
