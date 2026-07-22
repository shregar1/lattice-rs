use super::abstraction::AppException;
pub fn new_connection_failure(msg: impl Into<String>) -> AppException { AppException { message: msg.into(), status_code: 503, response_key: "CONNECTION_FAILURE".into() } }
