use super::abstraction::AppException;
pub fn new_internal_server(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 500, response_key: "INTERNAL_ERROR".into() } }
