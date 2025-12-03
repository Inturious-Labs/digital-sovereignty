//! Stripe payment integration
//!
//! Handles:
//! - Creating Payment Intents for article purchases
//! - Processing Stripe webhooks for payment completion
//! - Webhook signature verification

use candid::{CandidType, Deserialize};
use hmac::{Hmac, Mac};
use ic_cdk::management_canister::{
    http_request, transform_context_from_query, HttpHeader, HttpMethod, HttpRequestArgs,
    HttpRequestResult, TransformArgs,
};
use serde::Serialize;
use sha2::Sha256;
use std::cell::RefCell;

type HmacSha256 = Hmac<Sha256>;

// Stripe API configuration
const STRIPE_API_URL: &str = "https://api.stripe.com/v1";
const ARTICLE_PRICE_CENTS: u64 = 500; // $5.00

// Configuration storage
thread_local! {
    static STRIPE_SECRET_KEY: RefCell<String> = RefCell::new(String::new());
    static STRIPE_WEBHOOK_SECRET: RefCell<String> = RefCell::new(String::new());
}

/// Result type for Stripe operations
pub type StripeResult<T> = Result<T, StripeError>;

/// Stripe-related errors
#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum StripeError {
    NotConfigured,
    HttpRequestFailed(String),
    InvalidResponse(String),
    WebhookSignatureInvalid,
    WebhookTimestampExpired,
    ParseError(String),
}

impl std::fmt::Display for StripeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StripeError::NotConfigured => write!(f, "Stripe API keys not configured"),
            StripeError::HttpRequestFailed(msg) => write!(f, "HTTP request failed: {}", msg),
            StripeError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            StripeError::WebhookSignatureInvalid => write!(f, "Webhook signature is invalid"),
            StripeError::WebhookTimestampExpired => write!(f, "Webhook timestamp has expired"),
            StripeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Set Stripe API secret key
pub fn set_stripe_secret_key(key: String) {
    STRIPE_SECRET_KEY.with(|k| {
        *k.borrow_mut() = key;
    });
}

/// Set Stripe webhook secret
pub fn set_stripe_webhook_secret(secret: String) {
    STRIPE_WEBHOOK_SECRET.with(|s| {
        *s.borrow_mut() = secret;
    });
}

/// Check if Stripe is configured
pub fn is_configured() -> bool {
    STRIPE_SECRET_KEY.with(|k| !k.borrow().is_empty())
}

fn get_secret_key() -> StripeResult<String> {
    STRIPE_SECRET_KEY.with(|k| {
        let key = k.borrow().clone();
        if key.is_empty() {
            Err(StripeError::NotConfigured)
        } else {
            Ok(key)
        }
    })
}

fn get_webhook_secret() -> StripeResult<String> {
    STRIPE_WEBHOOK_SECRET.with(|s| {
        let secret = s.borrow().clone();
        if secret.is_empty() {
            Err(StripeError::NotConfigured)
        } else {
            Ok(secret)
        }
    })
}

// ============================================================================
// Payment Intent Creation
// ============================================================================

/// Request to create a payment intent
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct CreatePaymentRequest {
    pub email: String,
    pub article_slug: String,
    pub article_title: String,
}

/// Response from creating a payment intent
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct CreatePaymentResponse {
    pub payment_intent_id: String,
    pub client_secret: String,
    pub amount: u64,
    pub currency: String,
}

/// Stripe API response for Payment Intent
#[derive(Debug, Deserialize)]
struct StripePaymentIntent {
    id: String,
    client_secret: String,
    amount: u64,
    currency: String,
}

/// Create a Payment Intent for an article purchase
pub async fn create_payment_intent(request: CreatePaymentRequest) -> StripeResult<CreatePaymentResponse> {
    let secret_key = get_secret_key()?;

    // Build form-encoded body for Stripe API
    let body = format!(
        "amount={}&currency=usd&metadata[email]={}&metadata[article_slug]={}&metadata[article_title]={}&automatic_payment_methods[enabled]=true",
        ARTICLE_PRICE_CENTS,
        urlencoded(&request.email),
        urlencoded(&request.article_slug),
        urlencoded(&request.article_title)
    );

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/x-www-form-urlencoded".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", secret_key),
        },
    ];

    let request_arg = HttpRequestArgs {
        url: format!("{}/payment_intents", STRIPE_API_URL),
        method: HttpMethod::POST,
        body: Some(body.into_bytes()),
        max_response_bytes: Some(10_000),
        transform: Some(transform_context_from_query("transform_stripe_response".to_string(), vec![])),
        headers: request_headers,
        is_replicated: Some(false), // Non-replicated outcall for deterministic response
    };

    let response: HttpRequestResult = http_request(&request_arg)
        .await
        .map_err(|e| StripeError::HttpRequestFailed(e.to_string()))?;

    if response.status != 200u64 {
        let error_body = String::from_utf8_lossy(&response.body);
        return Err(StripeError::InvalidResponse(format!(
            "Status {}: {}",
            response.status, error_body
        )));
    }

    let payment_intent: StripePaymentIntent = serde_json::from_slice(&response.body)
        .map_err(|e| StripeError::ParseError(e.to_string()))?;

    Ok(CreatePaymentResponse {
        payment_intent_id: payment_intent.id,
        client_secret: payment_intent.client_secret,
        amount: payment_intent.amount,
        currency: payment_intent.currency,
    })
}

// ============================================================================
// Checkout Session (for redirect flow with immediate unlock)
// ============================================================================

/// Request to create a Checkout Session
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct CreateCheckoutSessionRequest {
    pub article_slug: String,
    pub article_title: String,
    pub price_cents: u64,  // Price in cents (e.g., 500 = $5.00)
    pub success_url: String,
    pub cancel_url: String,
}

/// Response from creating a Checkout Session
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct CreateCheckoutSessionResponse {
    pub session_id: String,
    pub checkout_url: String,
}

/// Stripe API response for Checkout Session
#[derive(Debug, Deserialize)]
struct StripeCheckoutSession {
    id: String,
    url: String,
}

/// Create a Checkout Session for article purchase (redirect flow)
pub async fn create_checkout_session(request: CreateCheckoutSessionRequest) -> StripeResult<CreateCheckoutSessionResponse> {
    let secret_key = get_secret_key()?;

    // Build form-encoded body for Stripe API
    // Note: We don't collect email here - Stripe will collect it on the checkout page
    let body = format!(
        "mode=payment\
        &line_items[0][price_data][currency]=usd\
        &line_items[0][price_data][product_data][name]={}\
        &line_items[0][price_data][unit_amount]={}\
        &line_items[0][quantity]=1\
        &metadata[article_slug]={}\
        &metadata[article_title]={}\
        &success_url={}\
        &cancel_url={}",
        urlencoded(&request.article_title),
        request.price_cents,
        urlencoded(&request.article_slug),
        urlencoded(&request.article_title),
        urlencoded(&format!("{}?session_id={{CHECKOUT_SESSION_ID}}", request.success_url)),
        urlencoded(&request.cancel_url)
    );

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/x-www-form-urlencoded".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", secret_key),
        },
    ];

    let request_arg = HttpRequestArgs {
        url: format!("{}/checkout/sessions", STRIPE_API_URL),
        method: HttpMethod::POST,
        body: Some(body.into_bytes()),
        max_response_bytes: Some(10_000),
        transform: Some(transform_context_from_query("transform_stripe_response".to_string(), vec![])),
        headers: request_headers,
        is_replicated: Some(false), // Non-replicated outcall for deterministic response
    };

    let response: HttpRequestResult = http_request(&request_arg)
        .await
        .map_err(|e| StripeError::HttpRequestFailed(e.to_string()))?;

    if response.status != 200u64 {
        let error_body = String::from_utf8_lossy(&response.body);
        return Err(StripeError::InvalidResponse(format!(
            "Status {}: {}",
            response.status, error_body
        )));
    }

    let session: StripeCheckoutSession = serde_json::from_slice(&response.body)
        .map_err(|e| StripeError::ParseError(e.to_string()))?;

    Ok(CreateCheckoutSessionResponse {
        session_id: session.id,
        checkout_url: session.url,
    })
}

/// Response from verifying a payment session
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct VerifyPaymentResponse {
    pub success: bool,
    pub email: Option<String>,
    pub article_slug: Option<String>,
    pub article_title: Option<String>,
    pub payment_status: String,
}

/// Stripe API response for retrieving Checkout Session
#[derive(Debug, Deserialize)]
struct StripeCheckoutSessionDetails {
    id: String,
    payment_status: String,
    customer_details: Option<StripeCustomerDetails>,
    metadata: Option<StripeMetadata>,
}

#[derive(Debug, Deserialize)]
struct StripeCustomerDetails {
    email: Option<String>,
}

/// Verify a completed payment by session ID
/// Called after user returns from Stripe checkout
pub async fn verify_payment_session(session_id: &str) -> StripeResult<VerifyPaymentResponse> {
    let secret_key = get_secret_key()?;

    let request_headers = vec![
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", secret_key),
        },
    ];

    let request_arg = HttpRequestArgs {
        url: format!("{}/checkout/sessions/{}", STRIPE_API_URL, session_id),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(10_000),
        transform: Some(transform_context_from_query("transform_stripe_response".to_string(), vec![])),
        headers: request_headers,
        is_replicated: Some(false), // Non-replicated outcall for deterministic response
    };

    let response: HttpRequestResult = http_request(&request_arg)
        .await
        .map_err(|e| StripeError::HttpRequestFailed(e.to_string()))?;

    if response.status != 200u64 {
        let error_body = String::from_utf8_lossy(&response.body);
        return Err(StripeError::InvalidResponse(format!(
            "Status {}: {}",
            response.status, error_body
        )));
    }

    let session: StripeCheckoutSessionDetails = serde_json::from_slice(&response.body)
        .map_err(|e| StripeError::ParseError(e.to_string()))?;

    let email = session.customer_details.and_then(|c| c.email);
    let metadata = session.metadata.unwrap_or(StripeMetadata {
        email: None,
        article_slug: None,
        article_title: None,
    });

    Ok(VerifyPaymentResponse {
        success: session.payment_status == "paid",
        email,
        article_slug: metadata.article_slug,
        article_title: metadata.article_title,
        payment_status: session.payment_status,
    })
}

// ============================================================================
// Webhook Handling
// ============================================================================

/// Stripe webhook event structure
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct WebhookEvent {
    pub event_type: String,
    pub payment_intent_id: String,
    pub email: Option<String>,
    pub article_slug: Option<String>,
    pub article_title: Option<String>,
    pub amount: u64,
}

/// Parsed Stripe event from webhook
#[derive(Debug, Deserialize)]
struct StripeEvent {
    #[serde(rename = "type")]
    event_type: String,
    data: StripeEventData,
}

#[derive(Debug, Deserialize)]
struct StripeEventData {
    object: StripeEventObject,
}

#[derive(Debug, Deserialize)]
struct StripeEventObject {
    id: String,
    amount: Option<u64>,
    metadata: Option<StripeMetadata>,
}

#[derive(Debug, Deserialize)]
struct StripeMetadata {
    email: Option<String>,
    article_slug: Option<String>,
    article_title: Option<String>,
}

/// Verify Stripe webhook signature
pub fn verify_webhook_signature(
    payload: &[u8],
    signature_header: &str,
    tolerance_seconds: u64,
) -> StripeResult<()> {
    let webhook_secret = get_webhook_secret()?;

    // Parse signature header: t=timestamp,v1=signature
    let mut timestamp: Option<&str> = None;
    let mut signature: Option<&str> = None;

    for part in signature_header.split(',') {
        let kv: Vec<&str> = part.splitn(2, '=').collect();
        if kv.len() == 2 {
            match kv[0] {
                "t" => timestamp = Some(kv[1]),
                "v1" => signature = Some(kv[1]),
                _ => {}
            }
        }
    }

    let timestamp = timestamp.ok_or(StripeError::WebhookSignatureInvalid)?;
    let signature = signature.ok_or(StripeError::WebhookSignatureInvalid)?;

    // Check timestamp is within tolerance
    let timestamp_secs: u64 = timestamp
        .parse()
        .map_err(|_| StripeError::WebhookSignatureInvalid)?;

    let current_time_secs = ic_cdk::api::time() / 1_000_000_000;
    if current_time_secs > timestamp_secs + tolerance_seconds {
        return Err(StripeError::WebhookTimestampExpired);
    }

    // Compute expected signature
    let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(signed_payload.as_bytes());
    let expected_signature = hex::encode(mac.finalize().into_bytes());

    // Constant-time comparison
    if !constant_time_eq(expected_signature.as_bytes(), signature.as_bytes()) {
        return Err(StripeError::WebhookSignatureInvalid);
    }

    Ok(())
}

/// Parse webhook payload into structured event
pub fn parse_webhook_event(payload: &[u8]) -> StripeResult<WebhookEvent> {
    let event: StripeEvent = serde_json::from_slice(payload)
        .map_err(|e| StripeError::ParseError(e.to_string()))?;

    let metadata = event.data.object.metadata.unwrap_or(StripeMetadata {
        email: None,
        article_slug: None,
        article_title: None,
    });

    Ok(WebhookEvent {
        event_type: event.event_type,
        payment_intent_id: event.data.object.id,
        email: metadata.email,
        article_slug: metadata.article_slug,
        article_title: metadata.article_title,
        amount: event.data.object.amount.unwrap_or(0),
    })
}

/// Check if event is a successful payment
pub fn is_payment_succeeded(event: &WebhookEvent) -> bool {
    event.event_type == "payment_intent.succeeded"
}

// ============================================================================
// Transform Function (for IC HTTP outcalls)
// ============================================================================

/// Transform function to clean up HTTP response for consensus
/// This is required for IC HTTP outcalls to work properly
/// Note: Exposed as a query function in lib.rs
///
/// Stripe responses contain variable fields (timestamps, request IDs) that
/// cause different replicas to see different responses. This transform
/// normalizes the response by keeping only the fields we need.
pub fn transform_stripe_response(args: TransformArgs) -> HttpRequestResult {
    let mut response = args.response;
    // Remove headers that may vary between replicas
    response.headers.clear();

    // Try to parse and normalize the JSON body
    if let Ok(body_str) = String::from_utf8(response.body.clone()) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_str) {
            // Extract only the fields we care about, removing variable ones
            let normalized = normalize_stripe_json(&json);
            if let Ok(normalized_bytes) = serde_json::to_vec(&normalized) {
                response.body = normalized_bytes;
            }
        }
    }

    response
}

/// Normalize Stripe JSON by keeping only stable fields
fn normalize_stripe_json(json: &serde_json::Value) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut result = serde_json::Map::new();
            for (key, value) in map {
                // Skip fields that vary between requests/replicas
                if matches!(key.as_str(),
                    "created" | "request" | "livemode" | "request_log_url" |
                    "balance_transaction" | "receipt_url" | "latest_charge"
                ) {
                    continue;
                }
                result.insert(key.clone(), normalize_stripe_json(value));
            }
            serde_json::Value::Object(result)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(normalize_stripe_json).collect())
        }
        _ => json.clone()
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// URL encode a string
fn urlencoded(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ' ' => result.push_str("%20"),
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}

/// Constant-time byte comparison
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

// ============================================================================
// Test Helpers
// ============================================================================

/// Test configuration status
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct StripeConfigStatus {
    pub is_configured: bool,
    pub has_secret_key: bool,
    pub has_webhook_secret: bool,
}

/// Get configuration status (for testing)
pub fn get_config_status() -> StripeConfigStatus {
    StripeConfigStatus {
        is_configured: is_configured(),
        has_secret_key: STRIPE_SECRET_KEY.with(|k| !k.borrow().is_empty()),
        has_webhook_secret: STRIPE_WEBHOOK_SECRET.with(|s| !s.borrow().is_empty()),
    }
}
