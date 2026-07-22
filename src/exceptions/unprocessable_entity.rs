use super::abstraction::AppException;
pub fn new_unprocessable_entity(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 422, response_key: "UNPROCESSABLE_ENTITY".into() } }
