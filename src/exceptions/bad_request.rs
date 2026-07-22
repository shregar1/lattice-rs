use super::abstraction::AppException;
pub fn new_bad_request(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 400, response_key: "BAD_REQUEST".into() } }
