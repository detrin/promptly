use serde::Deserialize;
use serde_json::{self, Value as JsonValue};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    param: Option<String>,
    #[serde(rename = "type")]
    error_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ApiError,
}

impl ErrorResponse {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn is_error(json: &str) -> bool {
        Self::from_json(json).is_ok()
    }
}