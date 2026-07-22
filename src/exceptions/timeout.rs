use super::abstraction::AppException;
pub fn new_timeout(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 504, response_key: "REQUEST_TIMEOUT".into() } }
