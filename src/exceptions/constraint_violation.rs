use super::abstraction::AppException;
pub fn new_constraint_violation(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 400, response_key: "CONSTRAINT_VIOLATION".into() } }
