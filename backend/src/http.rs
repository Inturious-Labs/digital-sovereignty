//! HTTP request handlers for REST API access
//!
//! This module enables the canister to respond to HTTP requests,
//! allowing frontend JavaScript to call the backend using standard fetch().

use candid::{CandidType, Deserialize};
use serde::Serialize;
use serde_json;

/// HTTP request structure (IC standard)
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// HTTP response structure (IC standard)
#[derive(CandidType, Serialize, Clone, Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    /// If true, IC gateway will forward request to http_request_update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade: Option<bool>,
}

impl HttpResponse {
    /// Create a JSON response with CORS headers
    pub fn json<T: Serialize>(status: u16, data: &T) -> Self {
        let body = serde_json::to_vec(data).unwrap_or_else(|_| b"{}".to_vec());
        Self {
            status_code: status,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
                ("Access-Control-Allow-Methods".to_string(), "GET, POST, OPTIONS".to_string()),
                ("Access-Control-Allow-Headers".to_string(), "Content-Type".to_string()),
            ],
            body,
            upgrade: None,
        }
    }

    /// Create a CORS preflight response
    pub fn cors_preflight() -> Self {
        Self {
            status_code: 204,
            headers: vec![
                ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
                ("Access-Control-Allow-Methods".to_string(), "GET, POST, OPTIONS".to_string()),
                ("Access-Control-Allow-Headers".to_string(), "Content-Type".to_string()),
                ("Access-Control-Max-Age".to_string(), "86400".to_string()),
            ],
            body: vec![],
            upgrade: None,
        }
    }

    /// Create a response that triggers upgrade to http_request_update
    pub fn upgrade() -> Self {
        Self {
            status_code: 200,
            headers: vec![],
            body: vec![],
            upgrade: Some(true),
        }
    }

    /// Create a 404 not found response
    pub fn not_found() -> Self {
        Self::json(404, &serde_json::json!({"error": "Not found"}))
    }

    /// Create a 400 bad request response
    pub fn bad_request(message: &str) -> Self {
        Self::json(400, &serde_json::json!({"error": message}))
    }
}

/// Extract path from URL (remove query string)
pub fn get_path(url: &str) -> &str {
    url.split('?').next().unwrap_or(url)
}

/// Parse JSON body from request
pub fn parse_json<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|e| format!("Invalid JSON: {}", e))
}
