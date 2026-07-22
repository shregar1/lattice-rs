use serde::Serialize;
use chrono::Utc;

#[derive(Debug, Serialize)]
pub struct BaseResponseEnvelope<T> {
    pub transaction_urn: String,
    pub status: String,
    pub response_message: String,
    pub response_key: String,
    pub errors: Vec<String>,
    pub timestamp: String,
    pub data: Option<T>,
    pub reference_urn: String,
}

pub struct BaseController;

impl BaseController {
    pub fn envelope<T>(data: Option<T>, message: impl Into<String>, response_key: impl Into<String>, status_code: u16) -> BaseResponseEnvelope<T> {
        let status = if status_code < 400 { "SUCCESS" } else { "FAILED" };
        BaseResponseEnvelope {
            transaction_urn: String::new(),
            status: status.to_string(),
            response_message: message.into(),
            response_key: response_key.into(),
            errors: Vec::new(),
            timestamp: Utc::now().to_rfc3339(),
            data,
            reference_urn: String::new(),
        }
    }

    pub fn success<T>(data: T, message: impl Into<String>, response_key: impl Into<String>) -> BaseResponseEnvelope<T> {
        Self::envelope(Some(data), message, response_key, 200)
    }

    pub fn created<T>(data: T, message: impl Into<String>, response_key: impl Into<String>) -> BaseResponseEnvelope<T> {
        Self::envelope(Some(data), message, response_key, 201)
    }
}
